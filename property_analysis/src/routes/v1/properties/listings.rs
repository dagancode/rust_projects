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
        api::{ApiResponse, MetaData},
        app::AppState,
        domain::{PropertyListing, PropertyType},
        error::ApiError,
    },
    routes::v1::utils::read_lock_handler,
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

    let lock = state.data.property_listings.clone();
    let guard = read_lock_handler(&lock);

    let result: Vec<PropertyListing> = guard
        .iter()
        .filter(|p| {
            let suburb_match = query.suburb.as_ref().map_or(true, |s| {
                p.address
                    .to_ascii_lowercase()
                    .contains(&s.to_ascii_lowercase())
            });
            let type_match = query.property_type.as_ref().map_or(true, |t| {
                p.property_type
                    .eq(&PropertyType::from(t.to_ascii_lowercase().as_str())) // FIX: PROP-7
            });
            suburb_match && type_match
        })
        .cloned()
        .collect();

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
