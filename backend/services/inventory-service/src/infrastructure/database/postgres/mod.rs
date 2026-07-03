use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Duration, Utc};

use crate::{
    dto::inventory::{InventorySummaryResponse, DashboardStatsResponse},
    error::{AppError, AppResult},
    domain::models::inventory::InventoryRow,
    ports::repository::{InventoryRepository, InventoryListFilter, InventoryListPage},
};

#[derive(Clone)]
pub struct PostgresInventoryRepository {
    db_pool: PgPool,
}

impl PostgresInventoryRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl InventoryRepository for PostgresInventoryRepository {
    async fn upsert(&self, cert: &InventorySummaryResponse) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO inventory_certificates (
                id,
                subject,
                issuer,
                status,
                expires_at,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                subject = EXCLUDED.subject,
                issuer = EXCLUDED.issuer,
                status = EXCLUDED.status,
                expires_at = EXCLUDED.expires_at,
                created_at = EXCLUDED.created_at
            "#,
        )
        .bind(cert.id)
        .bind(&cert.subject)
        .bind(&cert.issuer)
        .bind(cert.status.as_str())
        .bind(cert.expires_at)
        .bind(cert.created_at)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<InventorySummaryResponse>> {
        let row = sqlx::query_as::<_, InventoryRow>(
            r#"
            SELECT id, subject, issuer, status, expires_at, created_at
            FROM inventory_certificates
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        Ok(Some(
            row.into_summary().map_err(AppError::DataIntegrity)?,
        ))
    }

    async fn list(&self, filter: InventoryListFilter) -> AppResult<InventoryListPage> {
        let status = filter.status.as_ref().map(|s| s.as_str());
        let subject_pattern = filter.subject.as_ref().map(|s| format!("%{}%", s));

        let rows = sqlx::query_as::<_, InventoryRow>(
            r#"
            SELECT id, subject, issuer, status, expires_at, created_at
            FROM inventory_certificates
            WHERE ($1::text IS NULL OR status = $1)
              AND ($2::timestamptz IS NULL OR expires_at <= $2)
              AND ($3::text IS NULL OR subject ILIKE $3)
            ORDER BY created_at DESC, id DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(status)
        .bind(filter.expires_before)
        .bind(subject_pattern.as_ref())
        .bind(filter.limit)
        .bind(filter.offset)
        .fetch_all(&self.db_pool)
        .await?;

        let total_items: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM inventory_certificates
            WHERE ($1::text IS NULL OR status = $1)
              AND ($2::timestamptz IS NULL OR expires_at <= $2)
              AND ($3::text IS NULL OR subject ILIKE $3)
            "#,
        )
        .bind(status)
        .bind(filter.expires_before)
        .bind(subject_pattern.as_ref())
        .fetch_one(&self.db_pool)
        .await?;

        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            items.push(row.into_summary().map_err(AppError::DataIntegrity)?);
        }

        Ok(InventoryListPage { items, total_items })
    }

    async fn get_dashboard_stats(&self) -> AppResult<DashboardStatsResponse> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM inventory_certificates")
            .fetch_one(&self.db_pool)
            .await?;

        let active: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM inventory_certificates WHERE status = 'ACTIVE'")
            .fetch_one(&self.db_pool)
            .await?;

        let expired: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM inventory_certificates WHERE status = 'EXPIRED'")
            .fetch_one(&self.db_pool)
            .await?;

        let revoked: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM inventory_certificates WHERE status = 'REVOKED'")
            .fetch_one(&self.db_pool)
            .await?;

        let now = Utc::now();
        let thirty_days_hence = now + Duration::days(30);

        let expiring_soon: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM inventory_certificates
            WHERE status = 'ACTIVE'
              AND expires_at >= $1
              AND expires_at <= $2
            "#,
        )
        .bind(now)
        .bind(thirty_days_hence)
        .fetch_one(&self.db_pool)
        .await?;

        Ok(DashboardStatsResponse {
            total_certificates: total,
            active_certificates: active,
            expired_certificates: expired,
            revoked_certificates: revoked,
            expiring_soon_certificates: expiring_soon,
        })
    }
}
