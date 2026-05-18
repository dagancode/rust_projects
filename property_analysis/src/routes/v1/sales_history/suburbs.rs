use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::{debug, warn};

use crate::models::{app::AppState, domain::PropertyDetail, error::ApiError};

pub async fn get_suburb_sales_history(
    Path(suburb): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PropertyDetail>>, ApiError> {
    let guard = match state.sales_history.read() {
        Ok(lock) => lock,
        Err(poison_error) => {
            let lock = poison_error.into_inner();
            warn!(
                "Lock was poisoned - recovering from last stable state ({} items)",
                lock.len()
            );

            lock
        }
    };

    let result: Vec<PropertyDetail> = guard
        .iter()
        .filter(|val| val.property.location.suburb.contains(&suburb))
        .cloned()
        .collect();

    if result.is_empty() {
        debug!(
            "GET /sales-history/suburb/{suburb} -> {}",
            StatusCode::NOT_FOUND
        );
        return Err(ApiError::NotFound);
    }

    debug!("GET /sales-history/suburb/{suburb} -> {}", StatusCode::OK);
    Ok(Json(result))
}
