use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    app::AppState,
    dto::certificate::{
        CertificateListQuery, CertificateListResponse, CertificateResponse, IssueCertificateRequest, IssueCertificateResponse,
    },
    error::AppResult,
};

pub async fn list_certificates(
    State(state): State<AppState>,
    Query(query): Query<CertificateListQuery>,
) -> AppResult<Json<CertificateListResponse>> {
    let response = state.certificate_service.list_certificates(query).await?;
    Ok(Json(response))
}

pub async fn issue_certificate(
    State(state): State<AppState>,
    Json(request): Json<IssueCertificateRequest>,
) -> AppResult<Json<IssueCertificateResponse>> {
    let response = state.certificate_service.issue_certificate(request).await?;
    Ok(Json(response))
}

pub async fn get_certificate(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<CertificateResponse>> {
    let response = state.certificate_service.get_certificate(id).await?;
    Ok(Json(response))
}
