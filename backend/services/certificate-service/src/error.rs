use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

use ca_engine::CertificateGenerationError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("request validation failed: {0}")]
    Validation(String),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("certificate generation failed")]
    CertificateGeneration(#[from] CertificateGenerationError),

    #[error("database operation failed")]
    Database(#[from] sqlx::Error),

    #[error("stored data is invalid: {0}")]
    DataIntegrity(String),

    #[error("internal server error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::CertificateGeneration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DataIntegrity(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Validation(_) => "validation_error",
            Self::NotFound(_) => "not_found",
            Self::CertificateGeneration(_) => "certificate_generation_error",
            Self::Database(_) => "database_error",
            Self::DataIntegrity(_) => "data_integrity_error",
            Self::Internal(_) => "internal_error",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = ErrorResponse {
            error: ErrorBody {
                code: self.code(),
                message: self.to_string(),
            },
        };

        (status, Json(body)).into_response()
    }
}
