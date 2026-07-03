use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    app::AppState,
    dto::inventory::{
        InventoryListQuery, InventoryListResponse, DashboardStatsResponse, InventorySummaryResponse,
    },
    error::AppResult,
};

pub async fn list_certificates(
    State(state): State<AppState>,
    Query(query): Query<InventoryListQuery>,
) -> AppResult<Json<InventoryListResponse>> {
    let response = state.inventory_service.list_certificates(query).await?;
    Ok(Json(response))
}

pub async fn get_certificate(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<InventorySummaryResponse>> {
    let response = state.inventory_service.get_certificate(id).await?;
    Ok(Json(response))
}

pub async fn get_dashboard_stats(
    State(state): State<AppState>,
) -> AppResult<Json<DashboardStatsResponse>> {
    let response = state.inventory_service.get_dashboard_stats().await?;
    Ok(Json(response))
}
