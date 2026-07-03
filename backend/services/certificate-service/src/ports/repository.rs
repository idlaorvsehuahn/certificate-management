use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    dto::certificate::{CertificateResponse, CertificateSummaryResponse, CertificateStatus},
    error::AppResult,
};

#[derive(Clone, Debug)]
pub struct CertificateListFilter {
    pub status: Option<CertificateStatus>,
    pub expires_before: Option<DateTime<Utc>>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug)]
pub struct CertificateListPage {
    pub items: Vec<CertificateSummaryResponse>,
    pub total_items: i64,
}

#[async_trait]
pub trait CertificateRepository: Send + Sync + 'static {
    async fn create(&self, certificate: &CertificateResponse) -> AppResult<()>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<CertificateResponse>>;
    async fn list(&self, filter: CertificateListFilter) -> AppResult<CertificateListPage>;
}
