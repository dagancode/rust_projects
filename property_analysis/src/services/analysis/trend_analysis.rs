use std::collections::HashMap;

use rust_decimal::prelude::Decimal;

use crate::models::{
    analysis::{Sales, SuburbTrendAnalysis},
    domain::PropertyDetail,
    error::ApiError,
    filters::RangeFilter,
    filters::RangeQuery,
};

pub fn suburb_trend_analysis(
    suburb: &str,
    properties: &[PropertyDetail],
    range: RangeQuery,
) -> Result<SuburbTrendAnalysis, ApiError> {
    let mut sales_map: HashMap<u16, Vec<Decimal>> = HashMap::new();

    let filtered_properties: Vec<PropertyDetail> = properties
        .iter()
        .filter(|p| p.property.location.suburb.eq_ignore_ascii_case(suburb))
        .cloned()
        .collect();

    if filtered_properties.is_empty() {
        return Err(ApiError::NotFound);
    }

    let filtered_properties = filtered_properties.apply_range_filter(range);

    if filtered_properties.is_empty() {
        return Err(ApiError::NotFound);
    }

    for property in filtered_properties.iter() {
        let sales = &property.sales_history;

        for sale in sales {
            sales_map.entry(sale.year).or_default().push(sale.price);
        }
    }

    let mut sales: Vec<Sales> = sales_map
        .into_iter()
        .map(|(year, prices)| {
            let volume: u32 = prices.len() as u32;
            let avg_price = ((prices.iter().sum::<Decimal>()) / Decimal::from(volume)).round_dp(2);

            Sales {
                year,
                avg_price,
                volume,
            }
        })
        .collect();

    sales.sort();

    Ok(SuburbTrendAnalysis {
        suburb_name: suburb.to_string(),
        sales,
    })
}
