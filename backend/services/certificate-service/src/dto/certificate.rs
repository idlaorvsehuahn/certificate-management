use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct IssueCertificateRequest {
    pub subject: String,
    #[serde(default)]
    pub san_dns_names: Vec<String>,
    pub validity_days: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CertificateListQuery {
    pub status: Option<CertificateStatus>,
    pub expires_before: Option<DateTime<Utc>>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct CertificateListResponse {
    pub items: Vec<CertificateSummaryResponse>,
    pub page: u32,
    pub page_size: u32,
    pub total_items: i64,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct CertificateSummaryResponse {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub issued_at: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub status: CertificateStatus,
    pub san_dns_names: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CertificateResponse {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub issued_at: DateTime<Utc>,
    pub expiration: DateTime<Utc>,
    pub status: CertificateStatus,
    pub san_dns_names: Vec<String>,
    pub pem: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed: Option<ParsedCertificateResponse>,
}

#[derive(Debug, Serialize)]
pub struct IssueCertificateResponse {
    pub certificate: CertificateSummaryResponse,
    pub certificate_pem: String,
    pub private_key_pem: String,
    pub warning: String,
}

#[derive(Debug, Deserialize)]
pub struct ParseCertificateRequest {
    pub pem: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportCertificateRequest {
    pub pem: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParsedCertificateResponse {
    pub subject: String,
    pub issuer: String,
    pub version: i32,
    pub serial_number: String,
    pub signature_algorithm: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub days_remaining: i64,
    pub expiration_status: String,
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

impl From<ca_engine::parser::ParsedCertificate> for ParsedCertificateResponse {
    fn from(c: ca_engine::parser::ParsedCertificate) -> Self {
        Self {
            subject: c.subject,
            issuer: c.issuer,
            version: c.version,
            serial_number: c.serial_number,
            signature_algorithm: c.signature_algorithm,
            not_before: c.not_before,
            not_after: c.not_after,
            days_remaining: c.days_remaining,
            expiration_status: c.expiration_status,
            public_key_algorithm: c.public_key_algorithm,
            key_size: c.key_size,
            sha1_fingerprint: c.sha1_fingerprint,
            sha256_fingerprint: c.sha256_fingerprint,
            is_ca: c.is_ca,
            path_len_constraint: c.path_len_constraint,
            key_usages: c.key_usages,
            extended_key_usages: c.extended_key_usages,
            san_dns_names: c.san_dns_names,
            san_ips: c.san_ips,
            san_emails: c.san_emails,
            pem: c.pem,
        }
    }
}

pub use shared::dto::CertificateStatus;
