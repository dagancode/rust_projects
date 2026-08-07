use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<MetaData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub count: u32,
    pub next_cursor: Option<String>,
}
