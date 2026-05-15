use std::collections::HashMap;

use csv::Reader;
use uuid::Uuid;

use crate::models::csv::SalesRecord;
use crate::models::domain::*;

pub fn load_sales_history(
    file_path: &str,
) -> Result<Vec<PropertyDetail>, Box<dyn std::error::Error>> {
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

    Ok(results)
}
