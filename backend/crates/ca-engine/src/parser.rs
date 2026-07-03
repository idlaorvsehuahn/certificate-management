use chrono::{DateTime, TimeZone, Utc};
use ring::digest::{digest, SHA1_FOR_LEGACY_USE_ONLY, SHA256};
use serde::{Deserialize, Serialize};
use x509_parser::prelude::*;
use crate::pem::parse_pem_or_der;

/// Strongly typed structure representing the parsed metadata of an X.509 certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCertificate {
    pub subject: String,
    pub issuer: String,
    pub version: i32,
    pub serial_number: String,
    pub signature_algorithm: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub days_remaining: i64,
    pub expiration_status: String, // "Valid", "Expiring Soon", "Expired"
    pub public_key_algorithm: String,
    pub key_size: i32,
    pub sha1_fingerprint: String,
    pub sha256_fingerprint: String,
    pub is_ca: bool,
    pub path_len_constraint: Option<u32>,
    pub key_usages: Vec<String>,
    pub extended_key_usages: Vec<String>,
    pub san_dns_names: Vec<String>,
    pub san_ips: Vec<String>,
    pub san_emails: Vec<String>,
    pub pem: String,
}

/// Parses raw PEM or DER bytes to extract metadata from an X.509 certificate.
pub fn parse_certificate(bytes: &[u8]) -> Result<ParsedCertificate, String> {
    // 1. Convert input to DER and clean PEM representation
    let (der_bytes, pem_str) = parse_pem_or_der(bytes)?;

    // 2. Parse X509 structure from DER bytes
    let (_, cert) = parse_x509_certificate(&der_bytes)
        .map_err(|e| format!("X509 DER parsing error: {}", e))?;

    // 3. Extract Subject and Issuer common names
    let subject = extract_common_name(cert.subject());
    let issuer = extract_common_name(cert.issuer());

    // 4. Version (Stored zero-indexed in DER: 0 = v1, 1 = v2, 2 = v3)
    let version = (cert.version().0 + 1) as i32;

    // 5. Serial Number in uppercase colon-separated hex format
    let serial_bytes = cert.serial.to_bytes_be();
    let serial_number = format_hex_colon(&serial_bytes);

    // 6. Signature Algorithm
    let sig_alg_oid = &cert.signature_algorithm.algorithm;
    let signature_algorithm = x509_parser::objects::oid2description(sig_alg_oid, x509_parser::objects::oid_registry())
        .map(|s| s.to_string())
        .unwrap_or_else(|_| sig_alg_oid.to_string());

    // 7. Validity Dates
    let not_before_ts = cert.validity().not_before.timestamp();
    let not_after_ts = cert.validity().not_after.timestamp();

    let not_before = Utc.timestamp_opt(not_before_ts, 0)
        .single()
        .ok_or_else(|| "Invalid not_before timestamp".to_string())?;
    let not_after = Utc.timestamp_opt(not_after_ts, 0)
        .single()
        .ok_or_else(|| "Invalid not_after timestamp".to_string())?;

    // 8. Days Remaining & Expiration Status
    let now = Utc::now();
    let days_remaining = (not_after - now).num_days();

    let expiration_status = if now > not_after {
        "Expired".to_string()
    } else if days_remaining <= 30 {
        "Expiring Soon".to_string()
    } else {
        "Valid".to_string()
    };

    // 9. Public Key Algorithm and Key Size (bits)
    let spki = &cert.tbs_certificate.subject_pki;
    let pub_key_oid = &spki.algorithm.algorithm;
    let public_key_algorithm = x509_parser::objects::oid2description(pub_key_oid, x509_parser::objects::oid_registry())
        .map(|s| s.to_string())
        .unwrap_or_else(|_| pub_key_oid.to_string());

    let key_size = match spki.parsed() {
        Ok(pub_key) => pub_key.key_size() as i32,
        Err(_) => 0,
    };

    // 10. Calculate SHA-1 and SHA-256 Fingerprints
    let sha1_digest = digest(&SHA1_FOR_LEGACY_USE_ONLY, &der_bytes);
    let sha256_digest = digest(&SHA256, &der_bytes);

    let sha1_fingerprint = format_hex_colon(sha1_digest.as_ref());
    let sha256_fingerprint = format_hex_colon(sha256_digest.as_ref());

    // 11. Extensions: Basic Constraints
    let basic_constraints = cert.basic_constraints()
        .ok()
        .flatten();
    let is_ca = basic_constraints.as_ref().map(|bc| bc.value.ca).unwrap_or(false);
    let path_len_constraint = basic_constraints.as_ref().and_then(|bc| bc.value.path_len_constraint);

    // 12. Extensions: Key Usage Flags
    let key_usages = cert.key_usage()
        .ok()
        .flatten()
        .map(|ku| {
            let flags = ku.value.flags;
            let mut list = Vec::new();
            if flags & 1 != 0 { list.push("Digital Signature".to_string()); }
            if (flags >> 1) & 1 != 0 { list.push("Non Repudiation".to_string()); }
            if (flags >> 2) & 1 != 0 { list.push("Key Encipherment".to_string()); }
            if (flags >> 3) & 1 != 0 { list.push("Data Encipherment".to_string()); }
            if (flags >> 4) & 1 != 0 { list.push("Key Agreement".to_string()); }
            if (flags >> 5) & 1 != 0 { list.push("Key Cert Sign".to_string()); }
            if (flags >> 6) & 1 != 0 { list.push("CRL Sign".to_string()); }
            if (flags >> 7) & 1 != 0 { list.push("Encipher Only".to_string()); }
            if (flags >> 8) & 1 != 0 { list.push("Decipher Only".to_string()); }
            list
        })
        .unwrap_or_default();

    // 13. Extensions: Extended Key Usages
    let extended_key_usages = cert.extended_key_usage()
        .ok()
        .flatten()
        .map(|eku| {
            let value = eku.value;
            let mut list = Vec::new();
            if value.any { list.push("Any".to_string()); }
            if value.server_auth { list.push("Server Authentication".to_string()); }
            if value.client_auth { list.push("Client Authentication".to_string()); }
            if value.code_signing { list.push("Code Signing".to_string()); }
            if value.email_protection { list.push("Email Protection".to_string()); }
            if value.time_stamping { list.push("Time Stamping".to_string()); }
            if value.ocsp_signing { list.push("OCSP Signing".to_string()); }
            for oid in &value.other {
                let name = x509_parser::objects::oid2description(oid, x509_parser::objects::oid_registry())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|_| oid.to_string());
                list.push(name);
            }
            list
        })
        .unwrap_or_default();

    // 14. Extensions: SANs (Subject Alternative Names)
    let mut san_dns_names = Vec::new();
    let mut san_ips = Vec::new();
    let mut san_emails = Vec::new();

    if let Ok(Some(san_ext)) = cert.subject_alternative_name() {
        for gen_name in &san_ext.value.general_names {
            match gen_name {
                GeneralName::DNSName(dns) => san_dns_names.push(dns.to_string()),
                GeneralName::IPAddress(ip_bytes) => {
                    if ip_bytes.len() == 4 {
                        san_ips.push(format!("{}.{}.{}.{}", ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]));
                    } else if ip_bytes.len() == 16 {
                        let mut arr = [0u8; 16];
                        arr.copy_from_slice(ip_bytes);
                        let ip6 = std::net::Ipv6Addr::from(arr);
                        san_ips.push(ip6.to_string());
                    } else {
                        san_ips.push(format!("{:?}", ip_bytes));
                    }
                }
                GeneralName::RFC822Name(email) => san_emails.push(email.to_string()),
                _ => {}
            }
        }
    }

    Ok(ParsedCertificate {
        subject,
        issuer,
        version,
        serial_number,
        signature_algorithm,
        not_before,
        not_after,
        days_remaining,
        expiration_status,
        public_key_algorithm,
        key_size,
        sha1_fingerprint,
        sha256_fingerprint,
        is_ca,
        path_len_constraint,
        key_usages,
        extended_key_usages,
        san_dns_names,
        san_ips,
        san_emails,
        pem: pem_str,
    })
}

fn format_hex_colon(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(":")
}

fn extract_common_name(name: &X509Name) -> String {
    name.iter_common_name()
        .next()
        .and_then(|cn| cn.as_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| name.to_string())
}
