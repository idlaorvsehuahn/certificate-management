use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct InventoryListQuery {
    pub subject: Option<String>,
    pub status: Option<CertificateStatus>,
    pub expires_before: Option<DateTime<Utc>>,
    pub expiring_days: Option<i64>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InventoryListResponse {
    pub items: Vec<InventorySummaryResponse>,
    pub page: u32,
    pub page_size: u32,
    pub total_items: i64,
    pub total_pages: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventorySummaryResponse {
    pub id: Uuid,
    pub subject: String,
    pub issuer: String,
    pub status: CertificateStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DashboardStatsResponse {
    pub total_certificates: i64,
    pub active_certificates: i64,
    pub expired_certificates: i64,
    pub revoked_certificates: i64,
    pub expiring_soon_certificates: i64,
}

pub use shared::dto::CertificateStatus;
