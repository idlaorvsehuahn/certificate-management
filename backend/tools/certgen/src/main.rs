use std::fs;
use std::path::Path;
use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DistinguishedName, DnType,
    ExtendedKeyUsagePurpose, IsCa, KeyUsagePurpose,
};
use time::{Duration, OffsetDateTime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut certs_dir_str = "certs".to_string(); // Default output dir
    
    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--output" && i + 1 < args.len() {
            certs_dir_str = args[i + 1].clone();
            i += 2;
        } else {
            i += 1;
        }
    }

    let certs_dir = Path::new(&certs_dir_str);
    if !certs_dir.exists() {
        fs::create_dir_all(certs_dir)?;
    }

    println!("Generating CA certificate in {}...", certs_dir.display());
    // 1. Generate Root CA
    let mut ca_params = CertificateParams::new(Vec::new());
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    ca_params.distinguished_name = DistinguishedName::new();
    ca_params.distinguished_name.push(DnType::CommonName, "Arkion Root CA");
    ca_params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    ca_params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    ca_params.key_usages.push(KeyUsagePurpose::CrlSign);
    
    // Set validity: 10 years
    let now = OffsetDateTime::now_utc();
    ca_params.not_before = now - Duration::days(1);
    ca_params.not_after = now + Duration::days(3650);

    let ca_cert = Certificate::from_params(ca_params)?;
    let ca_pem = ca_cert.serialize_pem()?;
    let ca_key_pem = ca_cert.serialize_private_key_pem();

    fs::write(certs_dir.join("ca.crt"), &ca_pem)?;
    fs::write(certs_dir.join("ca.key"), &ca_key_pem)?;

    // Helper to generate and sign cert
    let generate_signed_cert = |common_name: &str, san_names: Vec<String>, filename_prefix: &str| -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating certificate for {}...", common_name);
        let mut params = CertificateParams::new(san_names);
        params.distinguished_name = DistinguishedName::new();
        params.distinguished_name.push(DnType::CommonName, common_name);
        
        // Key usage for server auth and client auth
        params.key_usages.push(KeyUsagePurpose::DigitalSignature);
        params.key_usages.push(KeyUsagePurpose::KeyEncipherment);
        params.extended_key_usages.push(ExtendedKeyUsagePurpose::ServerAuth);
        params.extended_key_usages.push(ExtendedKeyUsagePurpose::ClientAuth);
        
        params.not_before = now - Duration::days(1);
        params.not_after = now + Duration::days(825); // Max allowed validity in many systems

        let cert = Certificate::from_params(params)?;
        let cert_pem = cert.serialize_pem_with_signer(&ca_cert)?;
        let key_pem = cert.serialize_private_key_pem();

        fs::write(certs_dir.join(format!("{}.crt", filename_prefix)), &cert_pem)?;
        fs::write(certs_dir.join(format!("{}.key", filename_prefix)), &key_pem)?;
        Ok(())
    };

    // 2. Generate Certificate Service cert
    generate_signed_cert(
        "certificate-service",
        vec!["localhost".to_string(), "certificate-service".to_string()],
        "certificate-service",
    )?;

    // 3. Generate Inventory Service cert
    generate_signed_cert(
        "inventory-service",
        vec!["localhost".to_string(), "inventory-service".to_string()],
        "inventory-service",
    )?;

    println!("All certificates generated successfully in {}!", certs_dir.display());
    Ok(())
}
