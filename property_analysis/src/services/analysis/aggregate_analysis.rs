use rust_decimal::Decimal;

use crate::models::{analysis::SuburbAggregateAnalysis, domain::PropertyListing};

pub fn suburb_aggregate_analysis(
    suburb: &str,
    properties: &[PropertyListing],
) -> Option<SuburbAggregateAnalysis> {
    let filtered_properties: Vec<&PropertyListing> = properties
        .iter()
        .filter(|p| {
            p.address
                .to_ascii_lowercase()
                .contains(&suburb.to_lowercase())
        })
        .collect();

    if filtered_properties.is_empty() {
        return None;
    }

    let total_properties = filtered_properties.len() as u16;

    let avg_price = (filtered_properties.iter().map(|p| p.price).sum::<Decimal>()
        / Decimal::from(total_properties))
    .round_dp(2);

    let floor_sizes: Vec<Decimal> = filtered_properties
        .iter()
        .filter_map(|p| p.floor_size_m2.map(Decimal::from))
        .collect();
    let erf_sizes: Vec<Decimal> = filtered_properties
        .iter()
        .filter_map(|p| p.erf_size_m2.map(Decimal::from))
        .collect();

    let avg_floor_size = if floor_sizes.is_empty() {
        None
    } else {
        Some(((floor_sizes.iter().sum::<Decimal>()) / Decimal::from(floor_sizes.len())).round_dp(2))
    };

    let avg_erf_size = if erf_sizes.is_empty() {
        None
    } else {
        Some(((erf_sizes.iter().sum::<Decimal>()) / Decimal::from(erf_sizes.len())).round_dp(2))
    };

    Some(SuburbAggregateAnalysis {
        suburb_name: suburb.to_string(),
        avg_price,
        avg_floor_size,
        avg_erf_size,
        total_properties,
    })
}
