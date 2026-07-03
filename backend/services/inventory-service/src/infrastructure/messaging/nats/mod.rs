use futures_util::StreamExt;
use std::sync::Arc;

use crate::{
    application::services::inventory::InventoryService,
    dto::inventory::InventorySummaryResponse,
};

pub async fn run_nats_subscriber(
    client: async_nats::Client,
    service: Arc<InventoryService>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut subscriber = client.subscribe("certificate.issued").await?;
    tracing::info!("Subscribed to NATS subject 'certificate.issued'");

    while let Some(message) = subscriber.next().await {
        let payload = message.payload;

        match serde_json::from_slice::<shared::events::DomainEvent>(&payload) {
            Ok(event) => {
                match event {
                    shared::events::DomainEvent::CertificateIssued(issued_event) => {
                        let summary = InventorySummaryResponse {
                            id: issued_event.certificate_id,
                            subject: issued_event.subject,
                            issuer: issued_event.issuer,
                            status: issued_event.status,
                            expires_at: issued_event.expires_at,
                            created_at: issued_event.created_at,
                        };

                        tracing::info!(id = %summary.id, "Received certificate.issued event via NATS");

                        if let Err(e) = service.upsert_certificate(summary).await {
                            tracing::error!(error = %e, "Failed to upsert certificate in inventory service");
                        } else {
                            tracing::info!(id = %issued_event.certificate_id, "Successfully stored read model for certificate");
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to deserialize NATS message payload");
            }
        }
    }

    Ok(())
}
