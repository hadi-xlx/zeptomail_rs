use reqwest::{Response, StatusCode};

use crate::{
    ZeptoMailClient,
    EmailRequest,
    ApiResponse,
    ZeptoMailError,
    ApiError,
};

impl ZeptoMailClient {
    pub async fn send_email(
        &self,
        email_request: EmailRequest
    ) -> Result<ApiResponse, ZeptoMailError> {
        let endpoint: &str = "email";
        let url: String = format!("{}/{}", self.base_url, endpoint);

        let response: Response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Zoho-enczapikey {}", self.api_key))
            .json(&email_request)
            .send()
            .await?;

        let status: StatusCode = response.status();

        if status.is_success() {
            let response_text: String = response.text().await?;
            let success_response: ApiResponse = serde_json::from_str(&response_text)?;
            Ok(success_response)
        } else {
            let response_text: String = response.text().await?;
            let api_error: ApiError = serde_json::from_str(&response_text)?;
            Err(ZeptoMailError::ApiError(api_error))
        }
    }
}
