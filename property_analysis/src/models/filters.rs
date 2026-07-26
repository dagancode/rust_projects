use chrono::Datelike;
use serde::{Deserialize, Serialize};

use crate::models::error::ApiError;

#[derive(Serialize, Deserialize, Clone)]
pub struct RangeQuery {
    #[serde(rename = "from")]
    pub from_year: Option<u16>,
    #[serde(rename = "to")]
    pub to_year: Option<u16>,
}

#[must_use]
pub trait RangeFilter {
    fn apply_range_filter(self, range: RangeQuery) -> Self;
}

impl RangeQuery {
    pub fn validate_range_query(&self) -> Result<(), ApiError> {
        if let Some(from_year_query) = self.from_year {
            if from_year_query < 1900 || from_year_query > chrono::Utc::now().year() as u16 {
                return Err(ApiError::ValidationError(Some(format!(
                    "from value {} must be greater than 1900 and less than or equal to {}",
                    from_year_query,
                    chrono::Utc::now().year()
                ))));
            }

            if let Some(to_year_query) = self.to_year {
                if from_year_query > to_year_query {
                    return Err(ApiError::ValidationError(Some(format!(
                        "'from' value {} must less that 'to' value {}",
                        from_year_query, to_year_query
                    ))));
                }
            }
        }

        if let Some(to_year_query) = self.to_year {
            if to_year_query < 1900 {
                return Err(ApiError::ValidationError(Some(format!(
                    "to value {} must be greater than 1900",
                    to_year_query
                ))));
            }
        }

        Ok(())
    }
}
