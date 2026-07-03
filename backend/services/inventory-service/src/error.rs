use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("request validation failed: {0}")]
    Validation(String),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("database operation failed")]
    Database(#[from] sqlx::Error),

    #[error("stored data is invalid: {0}")]
    DataIntegrity(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

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
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DataIntegrity(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::Validation(_) => "validation_error",
            Self::NotFound(_) => "not_found",
            Self::Database(_) => "database_error",
            Self::DataIntegrity(_) => "data_integrity_error",
            Self::Unauthorized(_) => "unauthorized",
            Self::Forbidden(_) => "forbidden",
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
