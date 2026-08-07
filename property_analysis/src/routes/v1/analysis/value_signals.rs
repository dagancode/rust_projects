use crate::{
    models::{
        analysis::ValueSignal, api::{ApiResponse, MetaData}, app::AppState, db::PropertyValueSignalRow, error::ApiError,
    }
};
use axum::{
    extract::{Path, State},
    Json,
};

// GET /v1/analysis/suburbs/{suburb}/value-signals
#[axum::debug_handler]
pub async fn get_suburb_value_signals(
    Path(suburb): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ValueSignal>>>, ApiError> {
    let rows: Vec<PropertyValueSignalRow> = sqlx::query_as(r#"
    WITH suburb_avg AS (
        SELECT AVG(price) as avg_price
        FROM property_listings
        WHERE address ILIKE $1
    )
    SELECT 
        source_url,
        title,
        price,
        address,
        property_type,
        listing_date,
        erf_size_m2,
        floor_size_m2,
        price_per_m2,
        levies,
        rates_and_taxes,
        bedrooms,
        bedroom_detail,
        bathrooms,
        kitchens,
        lounges,
        dining_rooms,
        parking,
        garage,
        pool,
        garden,
        pet_friendly,
        facing,
        roof,
        wall,
        floor,
        internet_access,
        key_features,
        suburb_avg.avg_price
    FROM property_listings, suburb_avg
    WHERE address ILIKE $1
        AND price <= suburb_avg.avg_price
    "#)
        .bind(format!("%{}%", suburb))
        .fetch_all(&state.db)
        .await?;

    let mut result: Vec<ValueSignal> = rows.into_iter().map(ValueSignal::from).collect();

    result.sort_by(|a, b| b.discount_percentage.cmp(&a.discount_percentage));
    let count = result.iter().count() as u32;

    Ok(Json(ApiResponse {
        data: result,
        meta: Some(MetaData { count, next_cursor: None }),
    }))
}
