use axum::{http::StatusCode, response::IntoResponse};

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
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            Self::ParseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
            Self::PoisonedLock => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }
        }
    }
}
