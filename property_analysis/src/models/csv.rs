use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesRecord {
    pub street_number: String,
    pub street_name: String,
    pub price: Decimal,
    pub year: u16,
    pub neighbourhood: String,
    pub suburb: String,
    pub city: String,
    pub province: String,
    pub source_url: String,
}
