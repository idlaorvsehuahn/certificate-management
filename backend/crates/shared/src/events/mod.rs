use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::CertificateStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum DomainEvent {
    #[serde(rename = "certificate.issued")]
    CertificateIssued(CertificateIssuedEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CertificateIssuedEvent {
    pub certificate_id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub status: CertificateStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
