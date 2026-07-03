use async_trait::async_trait;

use shared::events::DomainEvent;
use crate::error::AppResult;

#[async_trait]
pub trait EventPublisher: Send + Sync + 'static {
    async fn publish(&self, event: &DomainEvent) -> AppResult<()>;
}
