use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("External API error: {0}")]
    ExternalApi(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            ApiError::ExternalApi(ref msg) => {
                tracing::error!("External API error: {}", msg);
                (StatusCode::BAD_GATEWAY, msg.as_str())
            }
            ApiError::NotFound(ref msg) => {
                tracing::error!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, msg.as_str())
            }
            ApiError::InvalidInput(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
