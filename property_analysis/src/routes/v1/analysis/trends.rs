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
        filters::RangeQuery,
    },
    routes::v1::utils::read_lock_handler,
    services::analysis::trend_analysis::suburb_trend_analysis,
};

// GET /analysis/suburbs/{suburb}/trends?from=2018&to=2024
pub async fn get_suburb_trend_analysis(
    Path(suburb): Path<String>,
    Query(range): Query<RangeQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let lock = state.sales_history.clone();
    let guard = read_lock_handler(&lock);

    match suburb_trend_analysis(suburb.as_str(), &guard, range) {
        Ok(result) => {
            let count = result.sales.len() as u32;

            debug!(
                "GET /analysis/suburbs/{suburb}/trends -> {}",
                StatusCode::OK
            );
            Ok(Json(ApiResponse {
                data: result,
                meta: Some(MetaData { count }),
            }))
        }
        Err(e) => Err(e),
    }
}
