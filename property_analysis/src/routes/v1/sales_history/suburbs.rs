use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::debug;

use crate::{
    models::{
        api::{ApiResponse, MetaData},
        app::AppState,
        db::PropertySaleRow,
        domain::{Location, Property, PropertyDetail, PropertySale},
        error::ApiError,
        filters::{RangeQuery},
    },
};

// GET /sales-history/suburb/{suburb}?from=2018&to=2024
pub async fn get_suburb_sales_history(
    Path(suburb): Path<String>,
    Query(range): Query<RangeQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    range.validate_range_query()?;

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
    JOIN property_sales_history psh 
        ON psh.property_id = pd.id
    WHERE pd.suburb ILIKE $1
        AND ($2::int IS NULL OR psh.year >= $2)
        AND ($3::int IS NULL OR psh.year <= $3)
    "#,
    )
    .bind(suburb.clone())
    .bind(range.from_year.map(|y| y as i32))
    .bind(range.to_year.map(|y| y as i32))
    .fetch_all(&state.db)
    .await?;

    if rows.is_empty() {
        debug!(
            "GET /sales-history/suburb/{suburb} -> {}",
            StatusCode::NOT_FOUND
        );
        return Err(ApiError::NotFound(Some(format!(
            "no properties found in suburb: {}",
            suburb
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

    debug!("GET /sales-history/suburbs/{suburb} -> {}", StatusCode::OK);
    let count = rows.len() as u32;

    Ok(Json(ApiResponse {
        data: results,
        meta: Some(MetaData { count }),
    }))
}
