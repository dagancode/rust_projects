use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::debug;

use crate::{
    models::{
        api::{ApiResponse, MetaData},
        app::AppState,
        domain::PropertyDetail,
        error::ApiError,
        filters::{RangeFilter, RangeQuery},
    },
    routes::v1::utils::read_lock_handler,
};

// GET /sales-history/suburb/{suburb}?from=2018&to=2024
pub async fn get_suburb_sales_history(
    Path(suburb): Path<String>,
    Query(range): Query<RangeQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    range.validate_range_query()?;

    let lock = state.data.sales_history.clone();
    let guard = read_lock_handler(&lock);

    let result: Vec<PropertyDetail> = guard
        .iter()
        .filter(|val| val.property.location.suburb.eq_ignore_ascii_case(&suburb))
        .cloned()
        .collect();

    if result.is_empty() {
        debug!(
            "GET /sales-history/suburb/{suburb} -> {}",
            StatusCode::NOT_FOUND
        );
        return Err(ApiError::NotFound(Some(format!(
            "no properties found in suburb: {}",
            suburb
        ))));
    }

    let result = result.apply_range_filter(range.clone());

    if result.is_empty() {
        debug!(
            "GET /sales-history/suburb/{suburb} -> {}",
            StatusCode::NOT_FOUND
        );
        return Err(ApiError::NotFound(Some(format!(
            "no properties found in suburb: {} for the selected range ({:?} - {:?})",
            suburb, range.from_year, range.to_year
        ))));
    }

    debug!("GET /sales-history/suburbs/{suburb} -> {}", StatusCode::OK);
    let count = result.len() as u32;

    Ok(Json(ApiResponse {
        data: result,
        meta: Some(MetaData { count }),
    }))
}
