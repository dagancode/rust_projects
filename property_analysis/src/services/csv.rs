use std::collections::HashMap;

use csv::Reader;
use rust_decimal::Decimal;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::csv::{ListingsRecord, SalesRecord};
use crate::models::domain::*;

pub fn load_sales_history(
    file_path: &str,
) -> Result<Vec<PropertyDetail>, Box<dyn std::error::Error>> {
    let mut property_sales: HashMap<Location, Vec<PropertySale>> = HashMap::new();

    let mut reader = Reader::from_path(file_path)?;

    for row in reader.deserialize() {
        let result: SalesRecord = row?;

        let sale = PropertySale {
            id: Uuid::new_v4(),
            year: result.year,
            price: result.price,
        };

        let location = Location {
            street_number: result.street_number.to_lowercase(),
            street_name: result.street_name.to_lowercase(),
            neighbourhood: result.neighbourhood.to_lowercase(),
            suburb: result.suburb.to_lowercase(),
            city: result.city.to_lowercase(),
            province: result.province.to_lowercase(),
            source_url: result.source_url,
        };

        property_sales
            .entry(location)
            .or_insert_with(|| Vec::new())
            .push(sale);
    }

    let results: Vec<PropertyDetail> = property_sales
        .into_iter()
        .map(|(k, v)| {
            let property = Property {
                id: Uuid::new_v4(),
                location: k,
            };
            PropertyDetail {
                property,
                sales_history: v,
            }
        })
        .collect();

    info!("Loaded {} property sales.", results.len());
    Ok(results)
}

pub fn load_listings(file_path: &str) -> Result<Vec<PropertyListing>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    let mut reader = Reader::from_path(file_path)?;

    for row in reader.deserialize() {
        results.push(parse_listings_record(row?));
    }

    info!("Loaded {} property listings.", results.len());
    Ok(results)
}

/// This function does not return any parsing errors, it will return default values if parsing fails.
pub fn parse_listings_record(record: ListingsRecord) -> PropertyListing {
    PropertyListing {
        source_url: record.source_url,
        title: record.title.to_lowercase(),
        price: record.price,
        address: record.address.to_lowercase(),
        //listing_number: record.listing_number,
        property_type: PropertyType::from(record.property_type.to_ascii_lowercase().as_str()),
        listing_date: ListingDate::from(record.listing_date.as_str()),
        erf_size_m2: record.erf_size_m2.and_then(|v| v.parse::<u32>().ok()),
        floor_size_m2: record.floor_size_m2.and_then(|v| v.parse::<u32>().ok()),
        price_per_m2: record.price_per_m2.and_then(|v| v.parse::<Decimal>().ok()),
        levies: record.levies.and_then(|v| v.parse::<Decimal>().ok()),
        rates_and_taxes: record
            .rates_and_taxes
            .and_then(|v| v.parse::<Decimal>().ok()),
        bedrooms: record.bedrooms,
        bedroom_detail: record.bedroom_detail,
        bathrooms: record.bathrooms.and_then(|v| v.parse::<u8>().ok()),
        kitchens: record.kitchens.and_then(|v| v.parse::<u8>().ok()),
        lounges: record.lounges.and_then(|v| v.parse::<u8>().ok()),
        dining_rooms: record.dining_rooms.and_then(|v| v.parse::<u8>().ok()),
        parking: record.parking.and_then(|v| v.parse::<u8>().ok()),
        garage: record.garage.and_then(|v| v.parse::<u8>().ok()),
        pool: record
            .pool
            .map_or(None, |v| Some(v.eq_ignore_ascii_case("yes"))),
        garden: record
            .garden
            .map_or(None, |v| Some(v.eq_ignore_ascii_case("yes"))),
        pet_friendly: record
            .pet_friendly
            .map_or(None, |v| Some(v.eq_ignore_ascii_case("yes"))),
        facing: record.facing,
        roof: record.roof,
        wall: record.wall,
        floor: record.floor,
        internet_access: record.internet_access,
        key_features: record.key_features,
    }
}

pub fn load_sales_history_directory(
    path: &str,
) -> Result<Vec<PropertyDetail>, Box<dyn std::error::Error>> {
    match std::fs::read_dir(path) {
        Ok(mut f) => {
            let csv_files: Vec<_> = f
                .by_ref()
                .filter(|d| match d {
                    Ok(entry) => entry
                        .path()
                        .extension()
                        .unwrap_or_default()
                        .eq_ignore_ascii_case("csv"),
                    Err(_) => false,
                })
                .collect();

            info!("Found {} CSV file(s).", csv_files.len());

            let mut results: Vec<PropertyDetail> = Vec::new();

            for entry in csv_files {
                let dir = entry?;

                results.extend_from_slice(&load_sales_history(&dir.path().to_string_lossy())?);
            }

            Ok(results)
        }
        Err(e) => {
            error!("Unable to load any CSV files in the path: {path}");
            Err(Box::new(e))
        }
    }
}

pub fn load_csv_data(
) -> Result<(Vec<PropertyDetail>, Vec<PropertyListing>), Box<dyn std::error::Error>> {
    let sales_history_path = std::env::var("SALES_HISTORY_PATH")
        .expect("Failed to load path. SALES_HISTORY_PATH must be set in .env ");
    let property_listings_path = std::env::var("PROPERTY_LISTINGS_PATH")
        .expect("Failed to load path. PROPERTY_LISTINGS_PATH must be set in .env ");
    Ok((
        load_sales_history_directory(&sales_history_path)?,
        load_listings(&property_listings_path)?,
    ))
}
