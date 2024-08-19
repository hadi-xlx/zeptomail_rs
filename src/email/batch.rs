//! ## Send Batch Email
//! This module contains the implementation for sending batch emails using the ZeptoMail API.

use reqwest::{Response, StatusCode};

use crate::{
    ZeptoMailClient,
    BatchEmailRequest,
    ApiResponse,
    ZeptoMailError,
    ApiError,
};

impl ZeptoMailClient {
    /// Sends a batch email using the ZeptoMail API.
    ///
    /// This function sends an email to multiple recipients using a single API request. The batch email 
    /// allows you to send the same email content to different recipients, with the ability to use merge fields 
    /// to personalize each recipient's email.
    ///
    /// # Arguments
    ///
    /// * `batch_email_request` - A `BatchEmailRequest` struct containing the details of the email, 
    ///   including sender, recipients, subject, body content, and other optional parameters.
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
    /// use zeptomail_rs::{ZeptoMailClient,BatchEmailRequest, EmailAddress, Recipient};
    /// use std::collections::HashMap;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client = ZeptoMailClient::new("your_api_key")?;
    ///
    ///     let sender = EmailAddress {
    ///         address: "sender@example.com".to_string(),
    ///         name: Some("Sender Name".to_string()),
    ///     };
    ///
    ///     let recipients = vec![
    ///         Recipient {
    ///             email_address: EmailAddress {
    ///                 address: "recipient1@example.com".to_string(),
    ///                 name: Some("Recipient One".to_string()),
    ///             },
    ///             merge_info: None,
    ///         },
    ///         Recipient {
    ///             email_address: EmailAddress {
    ///                 address: "recipient2@example.com".to_string(),
    ///                 name: Some("Recipient Two".to_string()),
    ///             },
    ///             merge_info: None,
    ///         },
    ///     ];
    ///
    ///     let batch_email_request = BatchEmailRequest {
    ///         sender,
    ///         recipients,
    ///         subject: "Batch Email Test".to_string(),
    ///         htmlbody: Some("<div>Batch Email Content</div>".to_string()),
    ///         textbody: Some("Batch Email Content".to_string()),
    ///         carbon_copy: None,
    ///         blind_carbon_copy: None,
    ///         track_clicks: Some(true),
    ///         track_opens: Some(true),
    ///         client_reference: None,
    ///         mime_headers: None,
    ///         attachments: None,
    ///         inline_images: None,
    ///     };
    ///
    ///     match client.send_batch_email(batch_email_request).await {
    ///         Ok(response) => println!("Email sent successfully: {:?}", response),
    ///         Err(e) => eprintln!("Failed to send email: {}", e),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_batch_email(
        &self,
        batch_email_request: BatchEmailRequest
    ) -> Result<ApiResponse, ZeptoMailError> {
        let endpoint: &str = "email/batch";
        let url: String = format!("{}/{}", self.base_url, endpoint);

        let response: Response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Zoho-enczapikey {}", self.api_key))
            .json(&batch_email_request)
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
