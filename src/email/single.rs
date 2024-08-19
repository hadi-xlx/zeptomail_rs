//! ## Send Email
//! This module contains the implementation for sending an email using the ZeptoMail API.

use reqwest::{Response, StatusCode};

use crate::{
    ZeptoMailClient,
    EmailRequest,
    ApiResponse,
    ZeptoMailError,
    ApiError,
};

impl ZeptoMailClient {
    /// Sends an email using the ZeptoMail API.
    ///
    /// This function sends an email to recipients using the ZeptoMail API. It constructs the request
    /// with the necessary headers and payload, and handles the response.
    ///
    /// # Arguments
    ///
    /// * `email_request` - An `EmailRequest` struct containing the details of the email, 
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
    /// use zeptomail_rs::{ZeptoMailClient, EmailRequest, EmailAddress, Recipient};
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
    ///     let recipient = Recipient {
    ///         email_address: EmailAddress {
    ///             address: "recipient@example.com".to_string(),
    ///             name: Some("Recipient Name".to_string()),
    ///         },
    ///         merge_info: None,
    ///     };
    ///
    ///     let email_request = EmailRequest {
    ///         bounce_address: None,
    ///         sender,
    ///         recipients: vec![recipient],
    ///         reply_to: None,
    ///         subject: "Test Email".to_string(),
    ///         htmlbody: Some("<div>Email Content</div>".to_string()),
    ///         textbody: Some("Email Content".to_string()),
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
    ///     match client.send_email(email_request).await {
    ///         Ok(response) => println!("Email sent successfully: {:?}", response),
    ///         Err(e) => eprintln!("Failed to send email: {}", e),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
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