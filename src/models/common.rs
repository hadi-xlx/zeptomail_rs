use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    pub address: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub content: Option<String>,       // Base64 encoded content
    pub mime_type: Option<String>,
    pub file_cache_key: Option<String>, // File cache key for an uploaded file
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MimeHeaders {
    pub headers: std::collections::HashMap<String, String>,
}
