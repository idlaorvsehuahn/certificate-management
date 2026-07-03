use x509_parser::pem::parse_x509_pem;
use base64::Engine;

/// Parses a PEM or DER certificate slice and returns the raw DER bytes and a clean PEM string.
pub fn parse_pem_or_der(bytes: &[u8]) -> Result<(Vec<u8>, String), String> {
    // 1. Try to parse as PEM
    if let Ok((_, pem)) = parse_x509_pem(bytes) {
        let pem_str = std::str::from_utf8(bytes)
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| encode_der_to_pem(&pem.contents));
        return Ok((pem.contents, pem_str));
    }

    // 2. Try parsing as raw DER
    if !bytes.is_empty() && bytes[0] == 0x30 {
        if x509_parser::parse_x509_certificate(bytes).is_ok() {
            let pem_str = encode_der_to_pem(bytes);
            return Ok((bytes.to_vec(), pem_str));
        }
    }

    Err("Invalid certificate format: must be valid PEM or DER".to_string())
}

/// Helper function to convert raw DER bytes to a standard PEM string representation.
pub fn encode_der_to_pem(der: &[u8]) -> String {
    use base64::engine::general_purpose::STANDARD;
    let b64 = STANDARD.encode(der);
    let mut pem = String::new();
    pem.push_str("-----BEGIN CERTIFICATE-----\n");
    for chunk in b64.as_bytes().chunks(64) {
        if let Ok(chunk_str) = std::str::from_utf8(chunk) {
            pem.push_str(chunk_str);
            pem.push_str("\n");
        }
    }
    pem.push_str("-----END CERTIFICATE-----\n");
    pem
}
