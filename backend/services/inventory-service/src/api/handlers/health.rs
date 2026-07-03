use axum::{Json, extract::State};
use serde::Serialize;

use crate::{app::AppState, error::AppResult};

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: String,
    version: String,
}

pub async fn health_check(State(state): State<AppState>) -> AppResult<Json<HealthResponse>> {
    Ok(Json(HealthResponse {
        status: "ok",
        service: state.config.service.name.clone(),
        version: state.config.service.version.clone(),
    }))
}
