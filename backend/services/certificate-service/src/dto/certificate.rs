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
}

#[derive(Debug, Serialize)]
pub struct IssueCertificateResponse {
    pub certificate: CertificateSummaryResponse,
    pub certificate_pem: String,
    pub private_key_pem: String,
    pub warning: String,
}

pub use shared::dto::CertificateStatus;
