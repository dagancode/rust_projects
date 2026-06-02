use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::helpers::RangeQuery;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyListing {
    pub source_url: String,
    pub title: String,
    pub price: Decimal,
    pub address: String,
    //pub listing_number: String,
    pub property_type: PropertyType,
    pub listing_date: ListingDate,
    pub erf_size_m2: Option<u32>,
    pub floor_size_m2: Option<u32>,
    pub price_per_m2: Option<Decimal>,
    pub levies: Option<Decimal>,
    pub rates_and_taxes: Option<Decimal>,
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyType {
    Apartment,
    Commercial,
    Industrial,
    House,
    Townhouse,
    VacantLand,
    Farm,
    Unknown,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListingDate {
    pub year: u16,
    pub month: u16,
    pub day: u16,
}

impl From<&str> for PropertyType {
    fn from(value: &str) -> PropertyType {
        match value {
            "apartment" | "flat" => Self::Apartment,
            "commercial" => Self::Commercial,
            "industrial" => Self::Industrial,
            "house" => Self::House,
            "townhouse" => Self::Townhouse,
            "vacant" | "land" | "plot" => Self::VacantLand,
            "farm" => Self::Farm,
            _ => Self::Unknown,
        }
    }
}

impl std::fmt::Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Apartment => write!(f, "Apartment / Flat"),
            Self::Commercial => write!(f, "Commercial Property"),
            Self::Industrial => write!(f, "Industrial Property"),
            Self::House => write!(f, "House"),
            Self::Townhouse => write!(f, "Townhouse"),
            Self::VacantLand => write!(f, "Vacant Land / Plot"),
            Self::Farm => write!(f, "Farm"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<&str> for ListingDate {
    fn from(raw_date: &str) -> Self {
        let parts: Vec<u16> = raw_date
            .split("-")
            .map(|p| p.parse::<u16>().unwrap_or(0))
            .collect();
        Self {
            year: parts[0],
            month: parts[1],
            day: parts[2],
        }
    }
}

impl PropertyDetail {
    fn apply_range_query(
        mut sales: Vec<Self>,
        range: RangeQuery,
    ) -> Vec<PropertyDetail> {
        if let Some(from_year_query) = range.from_year {
            sales.retain(|p| p.sales_history.iter().all(|s| s.year >= from_year_query));
        };

        if let Some(to_year_query) = range.to_year {
            sales.retain(|p| p.sales_history.iter().all(|s| s.year <= to_year_query));
        };

        sales
    }
}
