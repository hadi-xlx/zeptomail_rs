//! ## Send Email with Template
//! This module contains the implementation for sending template emails using the ZeptoMail API.

use reqwest::{Response, StatusCode};

use crate::{
    client::ZeptoMailClient,
    TemplateEmailRequest,
    ApiResponse,
    ZeptoMailError,
    ApiError,
};

impl ZeptoMailClient {
    /// Sends a template email using the ZeptoMail API.
    ///
    /// This function sends a template email using the ZeptoMail API. It constructs the request with the necessary
    /// headers and sends it to the API endpoint.
    ///
    /// # Arguments
    ///
    /// * `template_email_request` - A `TemplateEmailRequest` struct containing the details of the email to be sent,
    ///   including the template ID, recipient details, and any template variables.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`:
    /// * `Ok(ApiResponse)` - If the email is successfully sent, containing the API response.
    /// * `Err(ZeptoMailError)` - If an error occurs, containing the error details.
    ///
    /// # Errors
    ///
    /// This function can return the following errors:
    /// * `ZeptoMailError::ApiError` - If the API returns an error response.
    /// * `ZeptoMailError::NetworkError` - If there is a network issue while sending the request.
    /// * `ZeptoMailError::SerializationError` - If there is an issue serializing or deserializing the request/response.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use zeptomail_rs::{ZeptoMailClient, TemplateEmailRequest, ApiResponse, EmailAddress, Recipient};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client = ZeptoMailClient::new("your_api_key")?;
    ///
    ///     let template_email_request = TemplateEmailRequest {
    ///         template_key: "your_template_key".to_string(),
    ///         bounce_address: Some("bounce@example.com".to_string()),
    ///         sender: EmailAddress::new("sender@example.com".to_string()),
    ///         recipients: vec![Recipient::new("recipient@example.com".to_string())],
    ///         reply_to: Some(vec![EmailAddress::new("replyto@example.com".to_string())]),
    ///         track_clicks: Some(true),
    ///         track_opens: Some(true),
    ///         client_reference: Some("client_ref".to_string()),
    ///         mime_headers: None,
    ///         attachments: None,
    ///         merge_info: Some(std::collections::HashMap::new()),
    ///     };
    ///
    ///     match client.send_template_email(template_email_request).await {
    ///         Ok(response) => println!("Email sent successfully: {:?}", response),
    ///         Err(e) => eprintln!("Failed to send email: {}", e),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_template_email(
        &self,
        template_email_request: TemplateEmailRequest
    ) -> Result<ApiResponse, ZeptoMailError> {
        let endpoint: &str = "email/template";
        let url: String = format!("{}/{}", self.base_url, endpoint);

        let response: Response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Zoho-enczapikey {}", self.api_key))
            .json(&template_email_request)
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