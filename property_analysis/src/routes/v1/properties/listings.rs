use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    models::{
        api::{ApiResponse, MetaData}, app::AppState, db::PropertyListingRow, domain::{PropertyListing, PropertyType}, error::ApiError,
    }
};

#[derive(Deserialize, Serialize)]
pub struct ListingsQuery {
    pub suburb: Option<String>,
    pub property_type: Option<String>,
}

// GET /v1/listings?suburb={suburb}&property_type={type}
#[axum::debug_handler]
pub async fn get_listings(
    Query(query): Query<ListingsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(ref property_type_query) = query.property_type {
        PropertyType::from(property_type_query.as_str()).validate_property_type_query()?
    }

    let rows: Vec<PropertyListingRow> = sqlx::query_as(r#"
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
        key_features
    FROM property_listings
    WHERE ($1 IS NULL OR address ILIKE $1)
        AND ($2 IS NULL OR property_type ILIKE $2)
    "#)
    .bind(query.suburb.map(|s| format!("%{s}%")))
    .bind(query.property_type)
    .fetch_all(&state.db)
    .await?;

    let result: Vec<PropertyListing> = rows.into_iter().map(PropertyListing::from).collect();

    if result.is_empty() {
        debug!("/v1/listings -> {}", StatusCode::NOT_FOUND);
        return Err(ApiError::NotFound(Some(format!(
            "no property listings found"
        ))));
    }

    debug!("/v1/listings -> {}", StatusCode::OK);
    let count = result.len() as u32;

    Ok(Json(ApiResponse {
        data: result,
        meta: Some(MetaData { count }),
    }))
}
