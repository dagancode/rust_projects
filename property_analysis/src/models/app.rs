use std::sync::{Arc, RwLock};

use crate::models::domain::{PropertyDetail, PropertyListing};

#[derive(Clone)]
pub struct AppState {
    pub data: AppData,
    pub encoding_key: jsonwebtoken::EncodingKey,
    pub decoding_key: jsonwebtoken::DecodingKey,
    pub jwt_secret: String,
}

#[derive(Clone)]
pub struct AppData {
    pub sales_history: Arc<RwLock<Vec<PropertyDetail>>>,
    pub property_listings: Arc<RwLock<Vec<PropertyListing>>>,
}
