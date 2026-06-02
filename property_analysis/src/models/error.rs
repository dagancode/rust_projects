use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum ApiError {
    #[error("resource not found")]
    NotFound,
    #[error("failed to parse data")]
    ParseError,
    #[error("lock has been poisoned")]
    PoisonedLock,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::ParseError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PoisonedLock => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(json!({"error": self.to_string()}))).into_response()
    }
}
