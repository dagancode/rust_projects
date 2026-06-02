use std::sync::{Arc, RwLock};

use crate::models::domain::{PropertyDetail, PropertyListing};

#[derive(Clone, Default)]
pub struct AppState {
    pub sales_history: Arc<RwLock<Vec<PropertyDetail>>>,
    pub property_listings: Arc<RwLock<Vec<PropertyListing>>>
    // more to come :)
}
