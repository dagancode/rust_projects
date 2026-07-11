use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use tracing::debug;

use crate::models::{
    analysis::SuburbAggregateAnalysis, api::ApiResponse, app::AppState, db::SuburbAggregateRow,
    error::ApiError,
};

// GET /v1/analysis/suburbs/{suburb}/aggregate
pub async fn get_suburb_aggregate_analysis(
    Path(suburb): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SuburbAggregateAnalysis>>, ApiError> {
    let result: SuburbAggregateRow = sqlx::query_as(
        r#"
        SELECT 
            AVG(price) AS avg_price,
            AVG(floor_size_m2) AS avg_floor_size,
            AVG(erf_size_m2) AS avg_erf_size,
            COUNT(source_url) AS total_properties
        FROM property_listings
        WHERE ($1 IS NULL OR address ILIKE $1)
    "#,
    )
    .bind(format!("%{suburb}%"))
    .fetch_one(&state.db)
    .await?;

    let data = SuburbAggregateAnalysis {
        suburb_name: suburb.clone(),
        avg_price: result.avg_price.map_or(Decimal::from(0), |p| p.round_dp(2)),
        avg_floor_size: result.avg_floor_size.map(|s| s.round_dp(2)),
        avg_erf_size: result.avg_erf_size.map(|s| s.round_dp(2)),
        total_properties: result.total_properties as u16,
    };

    debug!(
        "GET /analysis/suburbs/{suburb}/aggregate -> {}",
        StatusCode::OK
    );
    Ok(Json(ApiResponse { data, meta: None }))
}
