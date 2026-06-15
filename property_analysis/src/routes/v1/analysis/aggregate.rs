use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::debug;

use crate::{
    models::{api::ApiResponse, app::AppState, error::ApiError},
    routes::v1::utils::read_lock_handler,
    services::analysis::aggregate_analysis::suburb_aggregate_analysis,
};

// GET /v1/analysis/suburbs/{suburb}/aggregate
pub async fn get_suburb_aggregate_analysis(
    Path(suburb): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let lock = state.data.property_listings.clone();
    let guard = read_lock_handler(&lock);

    match suburb_aggregate_analysis(&suburb, &guard) {
        Some(result) => {
            debug!(
                "GET /analysis/suburbs/{suburb}/aggregate -> {}",
                StatusCode::OK
            );
            Ok(Json(ApiResponse {
                data: result,
                meta: None,
            }))
        }
        None => Err(ApiError::NotFound(Some(format!(
            "no properties found in suburb: {}",
            suburb
        )))),
    }
}
