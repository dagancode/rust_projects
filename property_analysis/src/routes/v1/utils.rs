use std::sync::{RwLock, RwLockReadGuard};

use tracing::{debug, warn};

use crate::models::{
    domain::{PropertyDetail, PropertyListing},
    helpers::RangeQuery,
};

pub fn read_lock_handler<'a, T>(read_lock: &'a RwLock<Vec<T>>) -> RwLockReadGuard<'a, Vec<T>> {
    let guard = match read_lock.read() {
        Ok(lock) => lock,
        Err(poison_error) => {
            let lock = poison_error.into_inner();
            warn!(
                "Lock was poisoned - recovering from last stable state ({} items)",
                lock.len()
            );

            lock
        }
    };
    guard
}

pub fn apply_sales_history_range_query(
    mut sales: Vec<PropertyDetail>,
    range: RangeQuery,
) -> Vec<PropertyDetail> {
    debug!("1: {}", sales.len());
    if let Some(from_year_query) = range.from_year {
        sales.retain(|p| p.sales_history.iter().all(|s| s.year >= from_year_query));
        debug!("2: {}", sales.len());
    };

    if let Some(to_year_query) = range.to_year {
        sales.retain(|p| p.sales_history.iter().all(|s| s.year <= to_year_query));
        debug!("3: {}", sales.len());
    };

    sales
}


pub fn apply_listings_range_query(
    mut sales: Vec<PropertyListing>,
    range: RangeQuery,
) -> Vec<PropertyListing> {
    debug!("1: {}", sales.len());
    if let Some(from_year_query) = range.from_year {
        sales.retain(|p| p.listing_date.year >= from_year_query);
        debug!("2: {}", sales.len());
    };

    if let Some(to_year_query) = range.to_year {
        sales.retain(|p| p.listing_date.year <= to_year_query);
        debug!("3: {}", sales.len());
    };

    sales
}