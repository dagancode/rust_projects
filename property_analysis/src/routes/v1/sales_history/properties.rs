use std::collections::HashMap;

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
    db::PropertySaleRow,
    domain::{Location, Property, PropertyDetail, PropertySale},
    error::ApiError,
};

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

    let rows: Vec<PropertySaleRow> = sqlx::query_as(
        r#"
        SELECT 
            pd.id as property_id,
            pd.street_number,
            pd.street_name,
            pd.neighbourhood,
            pd.suburb,
            pd.city,
            pd.province,
            pd.source_url,
            psh.id as sale_id,
            psh.year,
            psh.price
        FROM property_detail pd
        JOIN property_sales_history psh ON psh.property_id = pd.id
        WHERE pd.suburb ILIKE $1
            AND pd.street_name ILIKE $2
            AND pd.street_number ILIKE $3
    "#,
    )
    .bind(format!("%{}%", suburb))
    .bind(format!("%{}%", street))
    .bind(format!("%{}%", number))
    .fetch_all(&state.db)
    .await?;

    if rows.is_empty() {
        debug!(
            "GET /sales-history/suburb/{suburb} -> {}",
            StatusCode::NOT_FOUND
        );
        return Err(ApiError::NotFound(Some(format!(
            "property not found: {} {} {}",
            number, street, suburb
        ))));
    }

    let mut property_map = HashMap::new();

    for row in &rows {
        let property_sale = PropertySale {
            id: row.sale_id,
            year: row.year as u16,
            price: row.price,
        };

        let property_detail = PropertyDetail {
            property: Property {
                id: row.property_id,
                location: Location {
                    street_number: row.street_number.clone(),
                    street_name: row.street_name.clone(),
                    neighbourhood: row.neighbourhood.clone(),
                    suburb: row.suburb.clone(),
                    city: row.city.clone(),
                    province: row.province.clone(),
                    source_url: row.source_url.clone(),
                },
            },
            sales_history: vec![property_sale],
        };

        property_map
            .entry(row.property_id)
            .and_modify(|pd: &mut PropertyDetail| pd.sales_history.push(property_sale))
            .or_insert_with(|| property_detail);
    }

    let results: Vec<PropertyDetail> = property_map.into_values().collect();

    debug!(
        "GET /sales-history/properties?suburb:{suburb}&street:{street}&number:{number} -> {}",
        StatusCode::OK
    );
    let count = results.len() as u32;

    Ok(Json(ApiResponse {
        data: results,
        meta: Some(MetaData { count, next_cursor: None }),
    }))
}
