use axum::{
    Json, extract::{Path, State}, http::StatusCode
};
use tracing::debug;

use crate::models::{app::AppState, domain::PropertyDetail, error::ApiError};

pub async fn get_suburb_sales_history(
    Path(suburb): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<PropertyDetail>>, ApiError> {
    match state.sales_history.read() {
        Ok(lock) => {
            let result: Vec<PropertyDetail> = lock
                .iter()
                .filter(|val| val.property.location.suburb.contains(&suburb))
                .cloned()
                .collect();

            if result.is_empty() {
                debug!("{} -> {}", format!("GET /sales-history/suburb/{suburb}"), StatusCode::NOT_FOUND);
                return Err(ApiError::NotFound);
            }

            debug!("{} -> {}", format!("GET /sales-history/suburb/{suburb}"), StatusCode::OK);
            Ok(Json(result))
        }

        Err(_) => Err(ApiError::PoisonedLock),
    }
}
