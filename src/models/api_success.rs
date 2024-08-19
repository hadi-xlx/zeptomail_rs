use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessData {
    pub code: String,
    pub additional_info: Option<Vec<String>>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<SuccessData>,
    pub message: String,
    pub request_id: String,
    pub object: Option<String>, // Optional, since not all responses may have an object
}
