use std::collections::HashMap;

use rust_decimal::prelude::Decimal;

use tracing::debug;

use crate::{models::{
    analysis::{Sales, SuburbTrendAnalysis},
    domain::PropertyDetail,
    helpers::RangeQuery,
}, routes::v1::utils::apply_sales_history_range_query};

pub fn suburb_trend_analysis(
    suburb: &str,
    properties: &[PropertyDetail],
    range: RangeQuery,
) -> SuburbTrendAnalysis {
    let mut sales_map: HashMap<u16, Vec<Decimal>> = HashMap::new();

    let mut filtered_properties: Vec<PropertyDetail> = properties
        .iter()
        .filter(|p| p.property.location.suburb.eq_ignore_ascii_case(suburb))
        .cloned()
        .collect();

    filtered_properties = apply_sales_history_range_query(filtered_properties, range);

    debug!("Filtered properties: {:?}", filtered_properties.len());

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

    SuburbTrendAnalysis {
        suburb_name: suburb.to_string(),
        sales,
    }
}
