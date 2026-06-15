use axum::{http::StatusCode, Json};
use serde_json::json;

pub async fn get_health() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "state": "healthy!"
        })),
    )
}
