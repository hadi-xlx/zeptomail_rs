use std::fmt;
use std::error::Error as StdError;
use serde::{Deserialize, Serialize};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    pub target: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<ApiErrorDetail>>,
    pub request_id: Option<String>,
}

#[derive(Debug)]
pub enum ZeptoMailError {
    ApiError(ApiError),
    NetworkError(ReqwestError),
    SerializationError(SerdeJsonError),
    UnexpectedResponse(String),
}

impl fmt::Display for ZeptoMailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZeptoMailError::ApiError(api_error) => write!(f, "API Error: {}", api_error.message),
            ZeptoMailError::NetworkError(err) => write!(f, "Network Error: {}", err),
            ZeptoMailError::SerializationError(err) => write!(f, "Serialization Error: {}", err),
            ZeptoMailError::UnexpectedResponse(msg) => write!(f, "Unexpected Response: {}", msg),
        }
    }
}

impl StdError for ZeptoMailError {}

impl From<ReqwestError> for ZeptoMailError {
    fn from(error: ReqwestError) -> Self {
        ZeptoMailError::NetworkError(error)
    }
}

impl From<serde_json::Error> for ZeptoMailError {
    fn from(error: SerdeJsonError) -> Self {
        ZeptoMailError::SerializationError(error)
    }
}
