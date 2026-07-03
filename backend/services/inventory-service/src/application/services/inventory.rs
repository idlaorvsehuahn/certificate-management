use tracing::info;
use uuid::Uuid;
use chrono::{Duration, Utc};

use shared::utils::{total_pages, validate_list_query};
use crate::{
    dto::inventory::{
        InventoryListQuery, InventoryListResponse, DashboardStatsResponse, InventorySummaryResponse,
    },
    error::{AppError, AppResult},
    ports::repository::{InventoryRepository, InventoryListFilter}
};

const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PAGE_SIZE: u32 = 20;
const MAX_PAGE_SIZE: u32 = 100;

#[derive(Clone)]
pub struct InventoryService<R = crate::infrastructure::database::postgres::PostgresInventoryRepository> {
    repository: R,
}

impl<R> InventoryService<R>
where
    R: InventoryRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn upsert_certificate(&self, certificate: InventorySummaryResponse) -> AppResult<()> {
        info!(
            certificate.id = %certificate.id,
            certificate.subject = %certificate.subject,
            "storing/updating certificate metadata in inventory read model"
        );
        self.repository.upsert(&certificate).await
    }

    pub async fn get_certificate(&self, id: Uuid) -> AppResult<InventorySummaryResponse> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("certificate {id} in inventory")))
    }

    pub async fn list_certificates(
        &self,
        query: InventoryListQuery,
    ) -> AppResult<InventoryListResponse> {
        let page = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        validate_list_query(page, page_size, MAX_PAGE_SIZE).map_err(AppError::Validation)?;

        let mut expires_before = query.expires_before;
        if let Some(days) = query.expiring_days {
            expires_before = Some(Utc::now() + Duration::days(days));
        }

        let filter = InventoryListFilter {
            subject: query.subject,
            status: query.status,
            expires_before,
            limit: i64::from(page_size),
            offset: i64::from((page - 1) * page_size),
        };

        let result = self.repository.list(filter).await?;
        let total_pages = total_pages(result.total_items, page_size);

        Ok(InventoryListResponse {
            items: result.items,
            page,
            page_size,
            total_items: result.total_items,
            total_pages,
        })
    }

    pub async fn get_dashboard_stats(&self) -> AppResult<DashboardStatsResponse> {
        self.repository.get_dashboard_stats().await
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::repository::InventoryListPage;
    use async_trait::async_trait;

    #[derive(Clone, Default)]
    struct InMemoryStore {
        certificate: Option<InventorySummaryResponse>,
    }

    #[async_trait]
    impl InventoryRepository for InMemoryStore {
        async fn upsert(&self, _certificate: &InventorySummaryResponse) -> AppResult<()> {
            Ok(())
        }

        async fn find_by_id(&self, _id: Uuid) -> AppResult<Option<InventorySummaryResponse>> {
            Ok(self.certificate.clone())
        }

        async fn list(&self, _filter: InventoryListFilter) -> AppResult<InventoryListPage> {
            Ok(InventoryListPage {
                items: Vec::new(),
                total_items: 0,
            })
        }

        async fn get_dashboard_stats(&self) -> AppResult<DashboardStatsResponse> {
            Ok(DashboardStatsResponse {
                total_certificates: 0,
                active_certificates: 0,
                expired_certificates: 0,
                revoked_certificates: 0,
                expiring_soon_certificates: 0,
            })
        }
    }

    fn test_service() -> InventoryService<InMemoryStore> {
        InventoryService::new(InMemoryStore::default())
    }

    #[tokio::test]
    async fn rejects_zero_page() {
        let service = test_service();
        let query = InventoryListQuery {
            subject: None,
            status: None,
            expires_before: None,
            expiring_days: None,
            page: Some(0),
            page_size: Some(20),
        };

        let result = service.list_certificates(query).await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[tokio::test]
    async fn returns_dashboard_stats() {
        let service = test_service();
        let stats = service.get_dashboard_stats().await.expect("stats");
        assert_eq!(stats.total_certificates, 0);
    }
}
