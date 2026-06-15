use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ApiError {
    #[error("resource not found")]
    NotFound(Option<String>),
    #[error("failed to parse data")]
    ParseError(Option<String>),
    #[error("lock has been poisoned")]
    PoisonedLock,
    #[error("one or more request values are invalid")]
    BadRequest,
    #[error("unauthorized request")]
    Unauthorized,
    #[error("validation error")]
    ValidationError(Option<String>),
}

impl ApiError {
    fn create_json_response(error: String, detail: Option<String>) -> Json<serde_json::Value> {
        if detail.is_none() {
            return json!({"error" : error}).into();
        }

        json!({
            "error": error,
            "detail": detail.unwrap()
        })
        .into()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let error_msg = self.to_string();
        let (status, context) = match self {
            Self::NotFound(ctx) => (StatusCode::NOT_FOUND, ctx),
            Self::ParseError(ctx) => (StatusCode::INTERNAL_SERVER_ERROR, ctx),
            Self::PoisonedLock => (StatusCode::INTERNAL_SERVER_ERROR, None),
            Self::BadRequest => (StatusCode::BAD_REQUEST, None),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, None),
            Self::ValidationError(ctx) => (StatusCode::BAD_REQUEST, ctx),
        };

        (status, ApiError::create_json_response(error_msg, context)).into_response()
    }
}
