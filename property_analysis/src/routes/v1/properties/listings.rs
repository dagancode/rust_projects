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
    cursor::{
        build_listings_query, encode_cursor, resolve_pagination, CursorError, CursorPayload,
        PaginationParams, SortDirection, SortField, SortValue,
    },
    db::PropertyListingRow,
    domain::{PropertyListing, PropertyType},
    error::ApiError,
};

#[derive(Deserialize, Serialize)]
pub struct ListingsFilters {
    pub suburb: Option<String>,
    pub property_type: Option<String>,
}

#[derive(Deserialize)]
pub struct ListingsQuery {
    pub suburb: Option<String>,
    pub property_type: Option<String>,
    pub sort: Option<SortField>,
    pub order: Option<SortDirection>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

// GET /v1/listings?suburb={suburb}&property_type={type}
#[axum::debug_handler]
pub async fn get_listings(
    Query(query): Query<ListingsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<PropertyListing>>>, ApiError> {
    let filters = ListingsFilters {
        suburb: query.suburb,
        property_type: query.property_type,
    };
    let pagination = PaginationParams {
        sort: query.sort,
        order: query.order,
        cursor: query.cursor,
        limit: query.limit,
    };

    tracing::debug!("{:?}", filters.suburb);
    if let Some(ref property_type_query) = filters.property_type {
        PropertyType::from(property_type_query.as_str()).validate_property_type_query()?
    }

    pagination.validate()?;

    let resolved = resolve_pagination(&pagination, state.cursor_secret.as_bytes())
        .map_err(|e| {
            match e {
                CursorError::MalformedEncoding | CursorError::SignatureMismatch => {
                    tracing::warn!("cursor rejected: {:?}", e);
                    ApiError::ValidationError(Some("invalid or expired cursor".to_string()))
                }
                CursorError::QueryShapeMismatch => ApiError::ValidationError(Some(
                    "cursor does not match the requested sort/order — start a new pagination sequence or omit sort/order when using a cursor".to_string()
                )),
            }
        })?;

    let limit = pagination.limit.unwrap_or(20);

    let mut qb = build_listings_query(&filters, &resolved, limit);
    let rows: Vec<PropertyListingRow> = qb.build_query_as().fetch_all(&state.db).await?;

    let next_cursor = if rows.len() as u32 == limit {
        rows.last().map(|last_row| {
            let last_value = match resolved.sort {
                SortField::Price => SortValue::Price(last_row.price),
                SortField::ListedDate => SortValue::ListedDate(last_row.listing_date),
                SortField::Sqm => SortValue::Sqm(last_row.floor_size_m2),
            };

            let payload = CursorPayload {
                sort_field: resolved.sort.clone(),
                direction: resolved.direction.clone(),
                last_value,
                last_id: last_row.id,
            };

            encode_cursor(&payload, state.cursor_secret.as_bytes())
        })
    } else {
        None
    };

    let result: Vec<PropertyListing> = rows.into_iter().map(PropertyListing::from).collect();
    let count = result.len() as u32;

    Ok(Json(ApiResponse {
        data: result,
        meta: Some(MetaData { count, next_cursor }),
    }))
}
