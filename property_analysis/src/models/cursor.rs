use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::error::ApiError;

#[derive(Serialize, Deserialize)]
pub struct CursorPayload {
    pub sort_field: SortField,
    pub direction: SortDirection,
    pub last_value: SortValue,
    pub last_id: Uuid,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    Price,
    ListedDate,
    Sqm,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize)]
pub enum SortValue {
    Price(Decimal),
    ListedDate(DateTime<Utc>),
    Sqm(i32),
}

pub enum CursorError {
    /// base64 decode failed, or JSON deserialize failed
    MalformedEncoding,
    /// HMAC verify failed — possible tampering    
    SignatureMismatch,
    /// cursor's sort_field/direction don't match the incoming request  
    QueryShapeMismatch,
}

pub fn encode_cursor(payload: &CursorPayload, secret: &[u8]) -> String {
    let data = serde_json::to_vec(payload).expect("should be able to encode any size");
    let signature = crate::crypto::hmac::sign(data.as_slice(), secret);

    let mut combined = signature;
    combined.extend_from_slice(&data);

    URL_SAFE_NO_PAD.encode(combined)
}

pub fn decode_cursor(cursor: &str, secret: &[u8]) -> Result<CursorPayload, CursorError> {
    let decoded = URL_SAFE_NO_PAD
        .decode(cursor)
        .map_err(|_| CursorError::MalformedEncoding)?;

    if decoded.len() < 32 {
        return Err(CursorError::MalformedEncoding);
    }
    let (signature, data) = decoded.split_at(32);

    if crate::crypto::hmac::verify(data, secret, signature) {
        let result: CursorPayload =
            serde_json::from_slice(data).map_err(|_| CursorError::MalformedEncoding)?;
        Ok(result)
    } else {
        Err(CursorError::SignatureMismatch)
    }
}

#[derive(Deserialize, Serialize)]
pub struct PaginationParams {
    pub sort: Option<SortField>,
    pub order: Option<SortDirection>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

impl PaginationParams {
    fn validate(&self) -> Result<(), ApiError> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 75 {
                return Err(ApiError::ValidationError(Some(format!(
                    "requested limit {} must be between 1 and 75",
                    limit,
                ))));
            }
        }

        Ok(())
    }
}

pub struct ResolvedPagination {
    pub sort: SortField,
    pub direction: SortDirection,
    pub cursor_bound: Option<(SortValue, Uuid)>, // None = first page, Some = next page
}

pub fn resolve_pagination(
    params: &PaginationParams,
    secret: &[u8],
) -> Result<ResolvedPagination, CursorError> {
    match &params.cursor {
        Some(cursor_str) => {
            let cursor = decode_cursor(cursor_str, secret)?;

            if let Some(requested_sort) = &params.sort {
                if requested_sort != &cursor.sort_field {
                    return Err(CursorError::QueryShapeMismatch);
                }
            }

            if let Some(requested_order) = &params.order {
                if requested_order != &cursor.direction {
                    return Err(CursorError::QueryShapeMismatch);
                }
            }

            Ok(ResolvedPagination {
                sort: cursor.sort_field,
                direction: cursor.direction,
                cursor_bound: Some((cursor.last_value, cursor.last_id)),
            })
        }
        None => {
            let sort = params.sort.clone().unwrap_or(SortField::ListedDate);
            let direction = params.order.clone().unwrap_or(SortDirection::Desc);
            Ok(ResolvedPagination {
                sort,
                direction,
                cursor_bound: None,
            })
        }
    }
}
