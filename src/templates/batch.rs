//! ## Send Batch Email with Template
//! This module contains the implementation for sending batch template emails using the ZeptoMail API.

use reqwest::{Response, StatusCode};
use crate::client::ZeptoMailClient;
use crate::models::template::BatchTemplateEmailRequest;
use crate::models::api_success::ApiResponse;
use crate::models::api_failure::{ZeptoMailError, ApiError};

impl ZeptoMailClient {
    /// Sends a batch email using a template via the ZeptoMail API.
    ///
    /// This function allows you to send an email to multiple recipients using a template. The batch email
    /// uses the same template for all recipients, with the ability to use merge fields to personalize each email.
    ///
    /// # Arguments
    ///
    /// * `batch_template_email_request` - A `BatchTemplateEmailRequest` struct containing the details of the email,
    ///   including sender, recipients, template key, and other optional parameters.
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
    /// use zeptomail_rs::{ZeptoMailClient, BatchTemplateEmailRequest, ApiResponse, EmailAddress, Recipient};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client = ZeptoMailClient::new("your_api_key")?;
    ///
    ///     let batch_template_email_request = BatchTemplateEmailRequest {
    ///         template_key: "your_template_key".to_string(),
    ///         bounce_address: Some("bounce@example.com".to_string()),
    ///         sender: EmailAddress::new("sender@example.com".to_string()),
    ///         recipients: vec![Recipient::new("recipient1@example.com".to_string()), Recipient::new("recipient2@example.com".to_string())],
    ///         reply_to: Some(vec![EmailAddress::new("replyto@example.com".to_string())]),
    ///         track_clicks: Some(true),
    ///         track_opens: Some(true),
    ///         client_reference: Some("client_ref".to_string()),
    ///         mime_headers: None,
    ///         attachments: None,
    ///     };
    ///
    ///     match client.send_batch_template_email(batch_template_email_request).await {
    ///         Ok(response) => println!("Batch email sent successfully: {:?}", response),
    ///         Err(e) => eprintln!("Failed to send batch email: {}", e),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_batch_template_email(
        &self,
        batch_template_email_request: BatchTemplateEmailRequest
    ) -> Result<ApiResponse, ZeptoMailError> {
        let endpoint: &str = "email/template/batch";
        let url: String = format!("{}/{}", self.base_url, endpoint);

        let response: Response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Zoho-enczapikey {}", self.api_key))
            .json(&batch_template_email_request)
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