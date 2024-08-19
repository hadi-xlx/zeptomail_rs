use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Vec<T>,
    pub message: String,
    pub request_id: String,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<ApiErrorDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    pub target: Option<String>,
}
