use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    domain::models::certificate::{CertificateRow, CertificateSanRow},
    dto::certificate::CertificateResponse,
    error::{AppError, AppResult},
    ports::repository::{CertificateRepository, CertificateListFilter, CertificateListPage},
};

#[derive(Clone)]
pub struct PostgresCertificateRepository {
    db_pool: PgPool,
}

impl PostgresCertificateRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CertificateRepository for PostgresCertificateRepository {
    async fn create(&self, certificate: &CertificateResponse) -> AppResult<()> {
        let mut transaction = self.db_pool.begin().await?;

        sqlx::query(
            r#"
            INSERT INTO certificates (
                id,
                subject,
                issuer,
                serial_number,
                expiration,
                issued_at,
                status,
                pem
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(certificate.id)
        .bind(&certificate.subject)
        .bind(&certificate.issuer)
        .bind(&certificate.serial_number)
        .bind(certificate.expiration)
        .bind(certificate.issued_at)
        .bind(certificate.status.as_str())
        .bind(&certificate.pem)
        .execute(&mut *transaction)
        .await?;

        insert_sans(&mut transaction, certificate).await?;

        transaction.commit().await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<CertificateResponse>> {
        let certificate = sqlx::query_as::<_, CertificateRow>(
            r#"
            SELECT id, subject, issuer, serial_number, expiration, issued_at, status, pem
            FROM certificates
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db_pool)
        .await?;

        let Some(certificate) = certificate else {
            return Ok(None);
        };

        let san_dns_names = sqlx::query_as::<_, CertificateSanRow>(
            r#"
            SELECT dns_name
            FROM certificate_sans
            WHERE certificate_id = $1
            ORDER BY id ASC
            "#,
        )
        .bind(id)
        .fetch_all(&self.db_pool)
        .await?
        .into_iter()
        .map(|row| row.dns_name)
        .collect();

        Ok(Some(
            certificate
                .into_response(san_dns_names)
                .map_err(AppError::DataIntegrity)?,
        ))
    }

    async fn list(&self, filter: CertificateListFilter) -> AppResult<CertificateListPage> {
        let status = filter.status.as_ref().map(|s| s.as_str());

        let certificates = sqlx::query_as::<_, CertificateRow>(
            r#"
            SELECT id, subject, issuer, serial_number, expiration, issued_at, status, pem
            FROM certificates
            WHERE ($1::text IS NULL OR status = $1)
              AND ($2::timestamptz IS NULL OR expiration <= $2)
            ORDER BY issued_at DESC, id DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(status)
        .bind(filter.expires_before)
        .bind(filter.limit)
        .bind(filter.offset)
        .fetch_all(&self.db_pool)
        .await?;

        let total_items: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM certificates
            WHERE ($1::text IS NULL OR status = $1)
              AND ($2::timestamptz IS NULL OR expiration <= $2)
            "#,
        )
        .bind(status)
        .bind(filter.expires_before)
        .fetch_one(&self.db_pool)
        .await?;

        let mut items = Vec::with_capacity(certificates.len());

        for certificate in certificates {
            let san_dns_names = find_sans(&self.db_pool, certificate.id).await?;
            items.push(
                certificate
                    .into_summary(san_dns_names)
                    .map_err(AppError::DataIntegrity)?,
            );
        }

        Ok(CertificateListPage { items, total_items })
    }
}

async fn insert_sans(
    transaction: &mut Transaction<'_, Postgres>,
    certificate: &CertificateResponse,
) -> AppResult<()> {
    for dns_name in &certificate.san_dns_names {
        sqlx::query(
            r#"
            INSERT INTO certificate_sans (certificate_id, dns_name)
            VALUES ($1, $2)
            "#,
        )
        .bind(certificate.id)
        .bind(dns_name)
        .execute(&mut **transaction)
        .await?;
    }

    Ok(())
}

async fn find_sans(db_pool: &PgPool, certificate_id: Uuid) -> AppResult<Vec<String>> {
    Ok(sqlx::query_as::<_, CertificateSanRow>(
        r#"
        SELECT dns_name
        FROM certificate_sans
        WHERE certificate_id = $1
        ORDER BY id ASC
        "#,
    )
    .bind(certificate_id)
    .fetch_all(db_pool)
    .await?
    .into_iter()
    .map(|row| row.dns_name)
    .collect())
}
