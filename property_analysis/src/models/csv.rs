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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingsRecord {
    pub source_url: String,
    pub title: String,
    pub price: Decimal,
    pub address: String,
    //pub listing_number: String,
    pub property_type: String,
    pub listing_date: String, // change to date
    pub erf_size_m2: Option<String>,
    pub floor_size_m2: Option<String>,
    pub price_per_m2: Option<String>,
    pub levies: Option<String>,
    pub rates_and_taxes: Option<String>,
    pub bedrooms: Option<u16>,
    pub bedroom_detail: Option<String>,
    pub bathrooms: Option<String>,
    pub kitchens: Option<String>,
    pub lounges: Option<String>,
    pub dining_rooms: Option<String>,
    pub parking: Option<String>,
    pub garage: Option<String>,
    pub pool: Option<String>,
    pub garden: Option<String>,
    pub pet_friendly: Option<String>,
    pub facing: Option<String>,
    pub roof: Option<String>,
    pub wall: Option<String>,
    pub floor: Option<String>,
    pub internet_access: Option<String>,
    pub key_features: Option<String>,
}
