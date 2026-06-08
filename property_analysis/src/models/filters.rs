use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

