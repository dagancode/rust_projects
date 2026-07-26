use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{error::ApiError, filters::RangeFilter, filters::RangeQuery};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PropertyListing {
    pub source_url: String,
    pub title: String,
    pub price: Decimal,
    pub address: String,
    pub property_type: PropertyType,
    pub listing_date: ListingDate,
    pub erf_size_m2: Option<u32>,
    pub floor_size_m2: Option<u32>,
    pub price_per_m2: Option<Decimal>,
    pub levies: Option<Decimal>,
    pub rates_and_taxes: Option<Decimal>,
    pub bedrooms: Option<u8>,
    pub bedroom_detail: Option<String>,
    pub bathrooms: Option<u8>,
    pub kitchens: Option<u8>,
    pub lounges: Option<u8>,
    pub dining_rooms: Option<u8>,
    pub parking: Option<u8>,
    pub garage: Option<u8>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyType {
    Apartment,
    Commercial,
    Industrial,
    House,
    Townhouse,
    VacantLand,
    Farm,
    Unknown(String),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListingDate {
    pub year: u16,
    pub month: u16,
    pub day: u16,
}

impl From<&str> for PropertyType {
    fn from(value: &str) -> PropertyType {
        match value.to_ascii_lowercase().as_str() {
            "apartment" | "flat" | "apartment / flat" => Self::Apartment,
            "commercial" | "commercial property" => Self::Commercial,
            "industrial" | "industrial property" => Self::Industrial,
            "house" => Self::House,
            "townhouse" => Self::Townhouse,
            "vacant" | "land" | "plot" | "vacant land / plot" => Self::VacantLand,
            "farm" => Self::Farm,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

impl PropertyType {
    pub fn validate_property_type_query(self) -> Result<(), ApiError> {
        match self {
            Self::Unknown(property_type) => Err(ApiError::ValidationError(Some(format!("invalid property_type '{}' - valid values: apartment, house, townhouse, commercial, industrial, plot, farm", property_type))).into()),
            _ => Ok(())
        }
    }
}

impl std::fmt::Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Apartment => write!(f, "Apartment / Flat"),
            Self::Commercial => write!(f, "Commercial Property"),
            Self::Industrial => write!(f, "Industrial Property"),
            Self::House => write!(f, "House"),
            Self::Townhouse => write!(f, "Townhouse"),
            Self::VacantLand => write!(f, "Vacant Land / Plot"),
            Self::Farm => write!(f, "Farm"),
            Self::Unknown(property_type) => write!(f, "Unknown: {property_type}",),
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

impl From<NaiveDate> for ListingDate {
    fn from(value: NaiveDate) -> Self {
        Self {
            year: value.year() as u16,
            month: value.month() as u16,
            day: value.day() as u16,
        }
    }
}

impl RangeFilter for Vec<PropertyListing> {
    fn apply_range_filter(mut self, range: RangeQuery) -> Self {
        match range {
            RangeQuery {
                from_year: Some(from_year_query),
                to_year: Some(to_year_query),
            } => {
                self.retain(|p| {
                    p.listing_date.year >= from_year_query && p.listing_date.year <= to_year_query
                });

                self
            }
            RangeQuery {
                from_year: Some(from_year_query),
                to_year: None,
            } => {
                self.retain(|p| p.listing_date.year >= from_year_query);

                self
            }
            RangeQuery {
                from_year: None,
                to_year: Some(to_year_query),
            } => {
                self.retain(|p| p.listing_date.year <= to_year_query);

                self
            }
            _ => self,
        }
    }
}

impl RangeFilter for Vec<PropertyDetail> {
    fn apply_range_filter(mut self, range: RangeQuery) -> Self {
        match range {
            RangeQuery {
                from_year: Some(from_year_query),
                to_year: Some(to_year_query),
            } => {
                self.retain(|p| {
                    p.sales_history
                        .iter()
                        .all(|s| s.year >= from_year_query && s.year <= to_year_query)
                });

                self
            }
            RangeQuery {
                from_year: Some(from_year_query),
                to_year: None,
            } => {
                self.retain(|p| p.sales_history.iter().all(|s| s.year >= from_year_query));

                self
            }
            RangeQuery {
                from_year: None,
                to_year: Some(to_year_query),
            } => {
                self.retain(|p| p.sales_history.iter().all(|s| s.year <= to_year_query));

                self
            }
            _ => self,
        }
    }
}

impl TryFrom<ListingDate> for NaiveDate {
    type Error = String;

    fn try_from(value: ListingDate) -> Result<Self, Self::Error> {
        chrono::NaiveDate::from_ymd_opt(value.year as i32, value.month as u32, value.day as u32)
            .ok_or(format!("Unable to parse {:?} as NativeDate", value))
    }
}
