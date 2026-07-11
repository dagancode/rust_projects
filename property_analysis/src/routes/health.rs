use axum::{http::StatusCode, Json};
use serde_json::json;
use tracing::debug;

pub async fn get_health() -> (StatusCode, Json<serde_json::Value>) {
    debug!("GET /health -> {}", StatusCode::OK);
    (
        StatusCode::OK,
        Json(json!({
            "state": "healthy!"
        })),
    )
}
