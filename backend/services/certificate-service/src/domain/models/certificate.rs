use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::certificate::CertificateSummaryResponse;
use crate::dto::certificate::{CertificateResponse, CertificateStatus};

#[derive(Debug, sqlx::FromRow)]
pub struct CertificateRow {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub expiration: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
    pub status: String,
    pub pem: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CertificateSanRow {
    pub dns_name: String,
}

impl CertificateRow {
    pub fn into_response(self, san_dns_names: Vec<String>) -> Result<CertificateResponse, String> {
        Ok(CertificateResponse {
            id: self.id,
            subject: self.subject,
            issuer: self.issuer,
            serial_number: self.serial_number,
            issued_at: self.issued_at,
            expiration: self.expiration,
            status: CertificateStatus::try_from(self.status)?,
            san_dns_names,
            pem: self.pem,
        })
    }

    pub fn into_summary(
        self,
        san_dns_names: Vec<String>,
    ) -> Result<CertificateSummaryResponse, String> {
        Ok(CertificateSummaryResponse {
            id: self.id,
            subject: self.subject,
            issuer: self.issuer,
            serial_number: self.serial_number,
            issued_at: self.issued_at,
            expiration: self.expiration,
            status: CertificateStatus::try_from(self.status)?,
            san_dns_names,
        })
    }
}
