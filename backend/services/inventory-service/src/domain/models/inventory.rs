use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::dto::inventory::{CertificateStatus, InventorySummaryResponse};

#[derive(Debug, sqlx::FromRow)]
pub struct InventoryRow {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub status: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl InventoryRow {
    pub fn into_summary(self) -> Result<InventorySummaryResponse, String> {
        Ok(InventorySummaryResponse {
            id: self.id,
            subject: self.subject,
            issuer: self.issuer,
            status: CertificateStatus::try_from(self.status)?,
            expires_at: self.expires_at,
            created_at: self.created_at,
        })
    }
}
