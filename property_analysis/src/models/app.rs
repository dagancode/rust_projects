use std::sync::{Arc, RwLock};

use crate::models::domain::PropertyDetail;

#[derive(Clone, Default)]
pub struct AppState {
    pub sales_history: Arc<RwLock<Vec<PropertyDetail>>>,
    // more to come :)
}
