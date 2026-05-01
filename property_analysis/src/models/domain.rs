use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub id: Uuid,
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDetail {
    pub property: Property,
    pub sales_history: Vec<PropertySale>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Location {
    pub street_number: String,
    pub street_name: String,
    pub neighbourhood: String,
    pub suburb: String,
    pub city: String,
    pub province: String,
    pub source_url: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PropertySale {
    pub id: Uuid,
    pub year: u16,
    pub price: Decimal,
}
