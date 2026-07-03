use async_trait::async_trait;
use async_nats::Client;

use shared::events::DomainEvent;
use crate::{
    error::{AppError, AppResult},
    ports::publisher::EventPublisher,
};

#[derive(Clone)]
pub struct NatsPublisher {
    client: Client,
}

impl NatsPublisher {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl EventPublisher for NatsPublisher {
    async fn publish(&self, event: &DomainEvent) -> AppResult<()> {
        let subject = match event {
            DomainEvent::CertificateIssued(_) => "certificate.issued",
        };

        let payload = serde_json::to_vec(event)
            .map_err(|e| AppError::Internal(format!("Failed to serialize event: {e}")))?;

        self.client
            .publish(subject, payload.into())
            .await
            .map_err(|e| AppError::Internal(format!("NATS publish error: {e}")))?;

        tracing::info!(subject = %subject, "Published event to NATS");
        Ok(())
    }
}
