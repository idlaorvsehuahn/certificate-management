use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    dto::inventory::{InventorySummaryResponse, DashboardStatsResponse, CertificateStatus},
    error::AppResult,
};

#[derive(Clone, Debug)]
pub struct InventoryListFilter {
    pub subject: Option<String>,
    pub status: Option<CertificateStatus>,
    pub expires_before: Option<DateTime<Utc>>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug)]
pub struct InventoryListPage {
    pub items: Vec<InventorySummaryResponse>,
    pub total_items: i64,
}

#[async_trait]
pub trait InventoryRepository: Send + Sync + 'static {
    async fn upsert(&self, cert: &InventorySummaryResponse) -> AppResult<()>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<InventorySummaryResponse>>;
    async fn list(&self, filter: InventoryListFilter) -> AppResult<InventoryListPage>;
    async fn get_dashboard_stats(&self) -> AppResult<DashboardStatsResponse>;
}
