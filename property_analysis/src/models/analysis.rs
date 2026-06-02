use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuburbAggregateAnalysis {
    pub suburb_name: String,
    pub avg_price: Decimal,
    pub avg_floor_size: Option<Decimal>,
    pub avg_erf_size: Option<Decimal>,
    pub total_properties: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuburbTrendAnalysis {
    pub suburb_name: String,
    pub sales: Vec<Sales>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sales {
    pub year: u16,
    pub avg_price: Decimal,
    pub volume: u32,
}