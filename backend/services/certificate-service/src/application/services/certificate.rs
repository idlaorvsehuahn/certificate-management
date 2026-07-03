use chrono::{Duration, Utc};
use tracing::info;
use uuid::Uuid;

use ca_engine::{CertificateGenerator, CertificateInput};
use shared::events::{CertificateIssuedEvent, DomainEvent};
use crate::{
    dto::certificate::{
        CertificateListQuery, CertificateListResponse, CertificateResponse, CertificateStatus,
        IssueCertificateRequest, IssueCertificateResponse, CertificateSummaryResponse,
    },
    error::{AppError, AppResult},
    ports::{
        publisher::EventPublisher,
        repository::{CertificateRepository, CertificateListFilter},
    },
};

const DEFAULT_VALIDITY_DAYS: i64 = 90;
const MAX_VALIDITY_DAYS: i64 = 825;
const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PAGE_SIZE: u32 = 20;
const MAX_PAGE_SIZE: u32 = 100;

#[derive(Clone)]
pub struct CertificateService<
    R = crate::infrastructure::database::postgres::PostgresCertificateRepository,
    P = crate::infrastructure::messaging::nats::NatsPublisher,
> {
    generator: CertificateGenerator,
    repository: R,
    publisher: P,
}

impl<R, P> CertificateService<R, P>
where
    R: CertificateRepository,
    P: EventPublisher + Clone,
{
    pub fn new(repository: R, publisher: P) -> Self {
        Self {
            generator: CertificateGenerator,
            repository,
            publisher,
        }
    }

    pub async fn issue_certificate(
        &self,
        request: IssueCertificateRequest,
    ) -> AppResult<IssueCertificateResponse> {
        let validity_days = request.validity_days.unwrap_or(DEFAULT_VALIDITY_DAYS);
        validate_issue_request(&request, validity_days)?;

        let issued_at = Utc::now();
        let expiration = issued_at + Duration::days(validity_days);
        let id = Uuid::new_v4();
        let serial_number = id.simple().to_string();

        let generated = self.generator.generate(CertificateInput {
            subject: request.subject.clone(),
            san_dns_names: normalized_sans(&request),
            serial_number: id.as_u128().to_be_bytes().to_vec(),
            not_before: issued_at,
            not_after: expiration,
        })?;

        info!(
            certificate.id = %id,
            certificate.subject = %request.subject,
            certificate.serial_number = %serial_number,
            "certificate issued"
        );

        let response = CertificateResponse {
            id,
            subject: request.subject,
            issuer: generated.issuer,
            serial_number,
            issued_at,
            expiration,
            status: CertificateStatus::Active,
            san_dns_names: generated.san_dns_names,
            pem: generated.pem,
        };

        // Persist to Postgres (commits transaction)
        self.repository.create(&response).await?;

        // Construct Domain Event
        let event = DomainEvent::CertificateIssued(CertificateIssuedEvent {
            certificate_id: response.id,
            subject: response.subject.clone(),
            issuer: response.issuer.clone(),
            status: response.status.clone(),
            expires_at: response.expiration,
            created_at: response.issued_at,
        });

        // Publish to NATS after transaction commits successfully
        let publisher = self.publisher.clone();
        let response_id = response.id;
        tokio::spawn(async move {
            if let Err(e) = publisher.publish(&event).await {
                tracing::error!(error = %e, "Failed to publish certificate.issued event to NATS");
            } else {
                tracing::info!(id = %response_id, "Successfully published event to NATS");
            }
        });

        let response_summary = CertificateSummaryResponse {
            id,
            subject: response.subject.clone(),
            issuer: response.issuer.clone(),
            serial_number: response.serial_number.clone(),
            issued_at: response.issued_at,
            expiration: response.expiration,
            status: response.status.clone(),
            san_dns_names: response.san_dns_names.clone(),
        };

        let issue_response = IssueCertificateResponse {
            certificate: response_summary,
            certificate_pem: response.pem.clone(),
            private_key_pem: generated.private_key_pem,
            warning: "The private key is displayed only once.".to_string(),
        };

        Ok(issue_response)
    }

    pub async fn get_certificate(&self, id: Uuid) -> AppResult<CertificateResponse> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("certificate {id}")))
    }

    pub async fn list_certificates(
        &self,
        query: CertificateListQuery,
    ) -> AppResult<CertificateListResponse> {
        let page = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        validate_list_query(page, page_size)?;

        let filter = CertificateListFilter {
            status: query.status,
            expires_before: query.expires_before,
            limit: i64::from(page_size),
            offset: i64::from((page - 1) * page_size),
        };

        let result = self.repository.list(filter).await?;
        let total_pages = total_pages(result.total_items, page_size);

        Ok(CertificateListResponse {
            items: result.items,
            page,
            page_size,
            total_items: result.total_items,
            total_pages,
        })
    }
}

fn validate_issue_request(request: &IssueCertificateRequest, validity_days: i64) -> AppResult<()> {
    if request.subject.trim().is_empty() {
        return Err(AppError::Validation("subject is required".to_string()));
    }

    if validity_days <= 0 || validity_days > MAX_VALIDITY_DAYS {
        return Err(AppError::Validation(format!(
            "validity_days must be between 1 and {MAX_VALIDITY_DAYS}"
        )));
    }

    if request.san_dns_names.len() > 20 {
        return Err(AppError::Validation(
            "san_dns_names cannot contain more than 20 entries".to_string(),
        ));
    }

    if request
        .san_dns_names
        .iter()
        .any(|dns_name| dns_name.trim().is_empty())
    {
        return Err(AppError::Validation(
            "san_dns_names cannot contain blank entries".to_string(),
        ));
    }

    Ok(())
}

fn normalized_sans(request: &IssueCertificateRequest) -> Vec<String> {
    if request.san_dns_names.is_empty() {
        return vec![request.subject.clone()];
    }

    request.san_dns_names.clone()
}

fn validate_list_query(page: u32, page_size: u32) -> AppResult<()> {
    if page == 0 {
        return Err(AppError::Validation(
            "page must be greater than 0".to_string(),
        ));
    }

    if page_size == 0 || page_size > MAX_PAGE_SIZE {
        return Err(AppError::Validation(format!(
            "page_size must be between 1 and {MAX_PAGE_SIZE}"
        )));
    }

    Ok(())
}

fn total_pages(total_items: i64, page_size: u32) -> u32 {
    if total_items == 0 {
        return 0;
    }

    ((total_items as f64) / f64::from(page_size)).ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::repository::CertificateListPage;
    use async_trait::async_trait;

    #[derive(Clone, Default)]
    struct InMemoryStore {
        certificate: Option<CertificateResponse>,
    }

    #[async_trait]
    impl CertificateRepository for InMemoryStore {
        async fn create(&self, _certificate: &CertificateResponse) -> AppResult<()> {
            Ok(())
        }

        async fn find_by_id(&self, _id: Uuid) -> AppResult<Option<CertificateResponse>> {
            Ok(self.certificate.clone())
        }

        async fn list(&self, _filter: CertificateListFilter) -> AppResult<CertificateListPage> {
            Ok(CertificateListPage {
                items: Vec::new(),
                total_items: 0,
            })
        }
    }

    #[derive(Clone)]
    struct MockPublisher;

    #[async_trait]
    impl EventPublisher for MockPublisher {
        async fn publish(&self, _event: &DomainEvent) -> AppResult<()> {
            Ok(())
        }
    }

    fn test_service() -> CertificateService<InMemoryStore, MockPublisher> {
        CertificateService::new(InMemoryStore::default(), MockPublisher)
    }

    #[tokio::test]
    async fn rejects_blank_subject() {
        let service = test_service();
        let request = IssueCertificateRequest {
            subject: " ".to_string(),
            san_dns_names: vec!["api.example.com".to_string()],
            validity_days: Some(30),
        };

        let result = service.issue_certificate(request).await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }

    #[test]
    fn default_san_uses_subject() {
        let request = IssueCertificateRequest {
            subject: "api.example.com".to_string(),
            san_dns_names: Vec::new(),
            validity_days: None,
        };

        assert_eq!(normalized_sans(&request), vec!["api.example.com"]);
    }

    #[tokio::test]
    async fn issues_certificate_with_default_validity() {
        let service = test_service();
        let request = IssueCertificateRequest {
            subject: "api.example.com".to_string(),
            san_dns_names: Vec::new(),
            validity_days: None,
        };

        let response = service
            .issue_certificate(request)
            .await
            .expect("issue certificate");

        assert_eq!(response.certificate.subject, "api.example.com");
        assert_eq!(response.certificate.san_dns_names, vec!["api.example.com"]);
        assert!(response.certificate_pem.contains("BEGIN CERTIFICATE"));
        assert!(response.private_key_pem.contains("PRIVATE KEY"));
    }

    #[tokio::test]
    async fn rejects_zero_page() {
        let service = test_service();
        let query = CertificateListQuery {
            status: None,
            expires_before: None,
            page: Some(0),
            page_size: Some(20),
        };

        let result = service.list_certificates(query).await;

        assert!(matches!(result, Err(AppError::Validation(_))));
    }
}
