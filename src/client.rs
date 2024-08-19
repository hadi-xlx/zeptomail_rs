use std::time::Duration;
use reqwest::Client;
use crate::ZeptoMailError;

pub struct ZeptoMailClient {
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
}

impl ZeptoMailClient {
    pub fn new(
        api_key: &str
    ) -> Result<Self, ZeptoMailError> {
        Ok(ZeptoMailClient {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(ZeptoMailError::NetworkError)?,
            api_key: api_key.to_string(),
            base_url: "https://api.zeptomail.eu/v1.1".to_string(),
        })
    }
}
