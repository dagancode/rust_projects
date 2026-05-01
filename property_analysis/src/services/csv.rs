use std::collections::HashMap;

use csv::Reader;
use uuid::Uuid;

use crate::models::domain::*;
use crate::models::csv::SalesRecord;

pub fn load_sales_history(file_path: &str) -> Result<Vec<PropertyDetail>, Box<dyn std::error::Error>> {
    let mut property_sales: HashMap<Location, Vec<PropertySale>> = HashMap::new();

    let mut reader = Reader::from_path(file_path).expect("Should have been able to read the CSV");

    for row in reader.deserialize() {
        let result: SalesRecord = row?;

        let sale = PropertySale {
            id: Uuid::new_v4(),
            year: result.year,
            price: result.price,
        };

        let location = Location {
            street_number: result.street_number,
            street_name: result.street_name,
            neighbourhood: result.neighbourhood,
            suburb: result.suburb,
            city: result.city,
            province: result.province,
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

    Ok(results)
}
