use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUploadRequest {
    pub name: String,
    pub content_type: String,
    pub data: Vec<u8>, // Binary data of the file
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileUploadResponse {
    pub file_cache_key: String,
    pub message: String,
    pub code: String,
}
