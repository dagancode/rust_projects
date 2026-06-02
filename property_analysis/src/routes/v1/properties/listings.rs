use axum::{
    Json, extract::{State, Query}, http::StatusCode
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    models::{
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
) -> Result<Json<Vec<PropertyListing>>, ApiError> {
    let lock = state.property_listings.clone();
    let guard = read_lock_handler(&lock);

    let mut result = guard.clone();

    if let Some(suburb_query) = &query.suburb {
        result.retain(|p| p.address.to_ascii_lowercase().contains(&suburb_query.to_ascii_lowercase()));
    }

    if let Some(property_type_query) = &query.property_type {
        result.retain(|p| p.property_type.eq(&PropertyType::from(property_type_query.to_ascii_lowercase().as_str())));
    }

    if result.is_empty() {
        debug!("/v1/listings -> {}", StatusCode::NOT_FOUND);
        return Err(ApiError::NotFound);
    }

    debug!("/v1/listings -> {}", StatusCode::OK);

    Ok(Json(result))
}
