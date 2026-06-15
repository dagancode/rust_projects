use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::models::{
    api::{ApiResponse, MetaData},
    app::AppState,
    domain::PropertyDetail,
    error::ApiError,
};
use crate::routes::v1::utils::read_lock_handler;

#[derive(Clone, Serialize, Deserialize)]
pub struct PropertyRequest {
    pub suburb: String,
    pub street: String,
    pub number: String,
}

// GET /sales-history/properties?suburb:suburb&street:street&number:number
#[axum::debug_handler]
pub async fn get_property_sales_history(
    Query(property_request): Query<PropertyRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let suburb = property_request.suburb.to_lowercase();
    let street = property_request.street.to_lowercase();
    let number = property_request.number.to_lowercase();

    let lock = state.data.sales_history.clone();
    let guard = read_lock_handler(&lock);

    let result: Vec<PropertyDetail> = guard
        .iter()
        .filter(|val| {
            val.property.location.suburb.eq(&suburb)
                && val.property.location.street_name.contains(&street)
                && val.property.location.street_number.eq(&number)
        })
        .cloned()
        .collect();

    if result.is_empty() {
        debug!(
            "GET /sales-history/suburb/{suburb} -> {}",
            StatusCode::NOT_FOUND
        );
        return Err(ApiError::NotFound(Some(format!(
            "property not found: {} {} {}",
            number, street, suburb
        ))));
    }

    debug!(
        "GET /sales-history/properties?suburb:{suburb}&street:{street}&number:{number} -> {}",
        StatusCode::OK
    );
    let count = result.len() as u32;

    Ok(Json(ApiResponse {
        data: result,
        meta: Some(MetaData { count }),
    }))
}
