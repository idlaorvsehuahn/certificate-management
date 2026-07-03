use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;

use crate::{
    api::routes,
    config::AppConfig,
    infrastructure::{
        database::postgres::PostgresCertificateRepository,
        messaging::nats::NatsPublisher,
    },
    application::services::certificate::CertificateService,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub certificate_service: Arc<CertificateService>,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: PgPool, nats_client: async_nats::Client) -> Self {
        let repository = PostgresCertificateRepository::new(db_pool);
        let publisher = NatsPublisher::new(nats_client);
        let config_arc = Arc::new(config);

        Self {
            config: config_arc,
            certificate_service: Arc::new(CertificateService::new(
                repository,
                publisher,
            )),
        }
    }
}

pub fn build_router(config: AppConfig, db_pool: PgPool, nats_client: async_nats::Client) -> Router {
    let state = AppState::new(config, db_pool, nats_client);
    routes::build_router(state)
}

pub use shared::tls::{serve_tls, load_certs, load_key};
