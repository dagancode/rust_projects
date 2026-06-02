use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use tracing::debug;

use crate::{
    models::{analysis::SuburbTrendAnalysis, app::AppState, error::ApiError, helpers::RangeQuery},
    routes::v1::utils::read_lock_handler,
    services::analysis::trend_analysis::suburb_trend_analysis,
};

// GET /analysis/suburbs/{suburb}/trends?from=2018&to=2024
pub async fn get_suburb_trend_analysis(
    Path(suburb): Path<String>,
    Query(range): Query<RangeQuery>,
    State(state): State<AppState>,
) -> Result<Json<SuburbTrendAnalysis>, ApiError> {
    let lock = state.sales_history.clone();
    let guard = read_lock_handler(&lock);

    let result = suburb_trend_analysis(suburb.as_str(), &guard, range);

    debug!(
        "GET /analysis/suburbs/{suburb}/trends -> {}",
        StatusCode::OK
    );
    Ok(Json(result))
}
