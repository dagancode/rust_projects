use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tracing::debug;

use crate::{
    models::app::AppState, routes::v1::utils::read_lock_handler,
    services::analysis::aggregate_analysis::suburb_aggregate_analysis,
};

// GET /v1/analysis/suburbs/{suburb}/aggregate
pub async fn get_suburb_aggregate_analysis(
    Path(suburb): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let lock = state.property_listings.clone();
    let guard = read_lock_handler(&lock);

    match suburb_aggregate_analysis(&suburb, &guard) {
        Some(result) => {
            debug!(
                "GET /analysis/trends/suburbs/{suburb} -> {}",
                StatusCode::OK
            );
            Ok(Json(result))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "suburb not found"
            })),
        )),
    }
}
