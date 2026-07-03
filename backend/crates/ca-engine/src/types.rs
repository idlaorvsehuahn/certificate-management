use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CertificateInput {
    pub subject: String,
    pub san_dns_names: Vec<String>,
    pub serial_number: Vec<u8>,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
}

#[derive(Debug)]
pub struct GeneratedCertificate {
    pub issuer: String,
    pub san_dns_names: Vec<String>,
    pub pem: String,
    pub private_key_pem: String,
}
