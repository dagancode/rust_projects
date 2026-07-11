use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{
    analysis::ValueSignal,
    domain::{ListingDate, PropertyListing, PropertyType},
};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct PropertySaleRow {
    pub property_id: Uuid,
    pub street_number: String,
    pub street_name: String,
    pub neighbourhood: String,
    pub suburb: String,
    pub city: String,
    pub province: String,
    pub source_url: String,
    pub sale_id: Uuid,
    pub year: i32,
    pub price: Decimal,
}

#[derive(sqlx::FromRow)]
pub struct PropertyListingRow {
    pub source_url: String,
    pub title: String,
    pub price: Decimal,
    pub address: String,
    pub property_type: String,
    pub listing_date: NaiveDate,
    pub erf_size_m2: Option<i32>,
    pub floor_size_m2: Option<i32>,
    pub price_per_m2: Option<Decimal>,
    pub levies: Option<Decimal>,
    pub rates_and_taxes: Option<Decimal>,
    pub bedrooms: Option<i16>,
    pub bedroom_detail: Option<String>,
    pub bathrooms: Option<i16>,
    pub kitchens: Option<i16>,
    pub lounges: Option<i16>,
    pub dining_rooms: Option<i16>,
    pub parking: Option<i16>,
    pub garage: Option<i16>,
    pub pool: Option<bool>,
    pub garden: Option<bool>,
    pub pet_friendly: Option<bool>,
    pub facing: Option<String>,
    pub roof: Option<String>,
    pub wall: Option<String>,
    pub floor: Option<String>,
    pub internet_access: Option<String>,
    pub key_features: Option<String>,
}

impl From<PropertyListingRow> for PropertyListing {
    fn from(row: PropertyListingRow) -> Self {
        Self {
            source_url: row.source_url,
            title: row.title,
            price: row.price,
            address: row.address,
            property_type: PropertyType::from(row.property_type.as_str()),
            listing_date: ListingDate::from(row.listing_date),
            erf_size_m2: row.erf_size_m2.map(|v| v as u32),
            floor_size_m2: row.floor_size_m2.map(|v| v as u32),
            price_per_m2: row.price_per_m2,
            levies: row.levies,
            rates_and_taxes: row.rates_and_taxes,
            bedrooms: row.bedrooms.map(|v| v as u8),
            bedroom_detail: row.bedroom_detail,
            bathrooms: row.bathrooms.map(|v| v as u8),
            kitchens: row.kitchens.map(|v| v as u8),
            lounges: row.lounges.map(|v| v as u8),
            dining_rooms: row.dining_rooms.map(|v| v as u8),
            parking: row.parking.map(|v| v as u8),
            garage: row.garage.map(|v| v as u8),
            pool: row.pool,
            garden: row.garden,
            pet_friendly: row.pet_friendly,
            facing: row.facing,
            roof: row.roof,
            wall: row.wall,
            floor: row.floor,
            internet_access: row.internet_access,
            key_features: row.key_features,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct PropertyValueSignalRow {
    pub source_url: String,
    pub title: String,
    pub price: Decimal,
    pub address: String,
    pub property_type: String,
    pub listing_date: NaiveDate,
    pub erf_size_m2: Option<i32>,
    pub floor_size_m2: Option<i32>,
    pub price_per_m2: Option<Decimal>,
    pub levies: Option<Decimal>,
    pub rates_and_taxes: Option<Decimal>,
    pub bedrooms: Option<i16>,
    pub bedroom_detail: Option<String>,
    pub bathrooms: Option<i16>,
    pub kitchens: Option<i16>,
    pub lounges: Option<i16>,
    pub dining_rooms: Option<i16>,
    pub parking: Option<i16>,
    pub garage: Option<i16>,
    pub pool: Option<bool>,
    pub garden: Option<bool>,
    pub pet_friendly: Option<bool>,
    pub facing: Option<String>,
    pub roof: Option<String>,
    pub wall: Option<String>,
    pub floor: Option<String>,
    pub internet_access: Option<String>,
    pub key_features: Option<String>,
    pub avg_price: Decimal,
}

impl From<PropertyValueSignalRow> for ValueSignal {
    fn from(row: PropertyValueSignalRow) -> Self {
        Self {
            listing: PropertyListing {
                source_url: row.source_url,
                title: row.title,
                price: row.price,
                address: row.address,
                property_type: PropertyType::from(row.property_type.as_str()),
                listing_date: ListingDate::from(row.listing_date),
                erf_size_m2: row.erf_size_m2.map(|v| v as u32),
                floor_size_m2: row.floor_size_m2.map(|v| v as u32),
                price_per_m2: row.price_per_m2,
                levies: row.levies,
                rates_and_taxes: row.rates_and_taxes,
                bedrooms: row.bedrooms.map(|v| v as u8),
                bedroom_detail: row.bedroom_detail,
                bathrooms: row.bathrooms.map(|v| v as u8),
                kitchens: row.kitchens.map(|v| v as u8),
                lounges: row.lounges.map(|v| v as u8),
                dining_rooms: row.dining_rooms.map(|v| v as u8),
                parking: row.parking.map(|v| v as u8),
                garage: row.garage.map(|v| v as u8),
                pool: row.pool,
                garden: row.garden,
                pet_friendly: row.pet_friendly,
                facing: row.facing,
                roof: row.roof,
                wall: row.wall,
                floor: row.floor,
                internet_access: row.internet_access,
                key_features: row.key_features,
            },
            suburb_avg_price: row.avg_price,
            discount_amount: Decimal::from(row.avg_price - row.price),
            discount_percentage: (Decimal::from(100)
                - ((row.price / row.avg_price) * Decimal::from(100)))
            .round_dp(2),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct SalesTrendRow {
    pub suburb: String,
    pub year: i32,
    pub avg_price: Decimal,
    pub volume: i64,
}

#[derive(sqlx::FromRow)]
pub struct SuburbAggregateRow {
    pub avg_price: Option<Decimal>,
    pub avg_floor_size: Option<Decimal>,
    pub avg_erf_size: Option<Decimal>,
    pub total_properties: i64,
}