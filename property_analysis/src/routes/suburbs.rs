use axum::{
    extract::{Path, State},
    Json,
};

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
                return Err(ApiError::NotFound);
            }

            Ok(Json(result))
        }

        Err(_) => Err(ApiError::PoisonedLock),
    }
}
