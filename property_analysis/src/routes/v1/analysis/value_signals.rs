use crate::{
    models::{
        analysis::ValueSignal,
        api::{ApiResponse, MetaData},
        app::AppState,
        domain::PropertyListing,
        error::ApiError,
    },
    routes::v1::utils::read_lock_handler,
    services::analysis::aggregate_analysis::suburb_aggregate_analysis,
};
use axum::{
    extract::{Path, State},
    Json,
};
use rust_decimal::Decimal;

// GET /v1/analysis/suburbs/{suburb}/value-signals
#[axum::debug_handler]
pub async fn get_suburb_value_signals(
    Path(suburb): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ValueSignal>>>, ApiError> {
    let lock = state.data.property_listings.clone();
    let guard = read_lock_handler(&lock);

    let analysis = suburb_aggregate_analysis(&suburb, &guard).ok_or(ApiError::NotFound(Some(
        format!("no properties found in suburb: {}", suburb),
    )))?;

    let listings: Vec<PropertyListing> = guard
        .iter()
        .filter(|p| p.address.contains(&suburb) && p.price < analysis.avg_price)
        .cloned()
        .collect();

    let mut result: Vec<_> = listings
        .iter()
        .map(|l| ValueSignal {
            listing: l.clone(),
            suburb_avg_price: analysis.avg_price,
            discount_amount: analysis.avg_price - l.price,
            discount_percentage: (Decimal::from(100)
                - ((l.price / analysis.avg_price) * Decimal::from(100)))
            .round_dp(2),
        })
        .collect();

    result.sort_by(|a, b| b.discount_percentage.cmp(&a.discount_percentage));
    let count = result.iter().count() as u32;

    Ok(Json(ApiResponse {
        data: result,
        meta: Some(MetaData { count }),
    }))
}
