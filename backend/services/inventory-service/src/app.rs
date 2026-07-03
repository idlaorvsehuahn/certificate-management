use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;

use crate::{
    api::routes,
    config::AppConfig,
    infrastructure::database::postgres::PostgresInventoryRepository,
    application::services::inventory::InventoryService,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub inventory_service: Arc<InventoryService>,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: PgPool) -> Self {
        let repository = PostgresInventoryRepository::new(db_pool);
        let config_arc = Arc::new(config);

        Self {
            config: config_arc,
            inventory_service: Arc::new(InventoryService::new(
                repository,
            )),
        }
    }
}

pub fn build_router(config: AppConfig, db_pool: PgPool) -> Router {
    let state = AppState::new(config, db_pool);
    routes::build_router(state)
}

pub use shared::tls::{serve_tls, load_certs, load_key};
