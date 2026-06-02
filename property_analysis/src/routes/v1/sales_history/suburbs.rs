use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use tracing::debug;

use crate::{
    models::{app::AppState, domain::PropertyDetail, error::ApiError, helpers::RangeQuery},
    routes::v1::utils::{apply_sales_history_range_query, read_lock_handler},
};

// GET /sales-history/suburb/{suburb}?from=2018&to=2024
pub async fn get_suburb_sales_history(
    Path(suburb): Path<String>,
    Query(range): Query<RangeQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PropertyDetail>>, ApiError> {
    let lock = state.sales_history.clone();
    let guard = read_lock_handler(&lock);

    let mut result: Vec<PropertyDetail> = guard
        .iter()
        .filter(|val| val.property.location.suburb.eq_ignore_ascii_case(&suburb))
        .cloned()
        .collect();

    result = apply_sales_history_range_query(result, range);

    if result.is_empty() {
        debug!(
            "GET /sales-history/suburb/{suburb} -> {}",
            StatusCode::NOT_FOUND
        );
        return Err(ApiError::NotFound);
    }

    debug!("GET /sales-history/suburbs/{suburb} -> {}", StatusCode::OK);
    Ok(Json(result))
}
