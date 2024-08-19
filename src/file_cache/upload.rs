//! ## File Upload to Cache
//! This module contains the implementation for uploading files to the cache using the ZeptoMail API.

use reqwest::multipart::{Form, Part};
use reqwest::{Response, StatusCode};

use crate::{
    ZeptoMailClient,
    FileUploadRequest,
    FileUploadResponse,
    ZeptoMailError,
    ApiError,
};

impl ZeptoMailClient {
    /// Uploads a file to the cache using the ZeptoMail API.
    ///
    /// This function uploads a file to the ZeptoMail cache. It constructs a multipart form with the file data
    /// and sends the request to the API.
    ///
    /// # Arguments
    ///
    /// * `file_upload_request` - A `FileUploadRequest` struct containing the details of the file to be uploaded, 
    ///   including the file name, content type, and file data.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`:
    /// * `Ok(FileUploadResponse)` - If the file is successfully uploaded, containing the API response.
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
    /// use zeptomail_rs::{ZeptoMailClient,FileUploadRequest, FileUploadResponse};
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let client = ZeptoMailClient::new("your_api_key")?;
    ///
    ///     let file_upload_request = FileUploadRequest {
    ///         name: "example.txt".to_string(),
    ///         content_type: "text/plain".to_string(),
    ///         data: b"Hello, world!".to_vec(),
    ///     };
    ///
    ///     match client.upload_file_to_cache(file_upload_request).await {
    ///         Ok(response) => println!("File uploaded successfully: {:?}", response),
    ///         Err(e) => eprintln!("Failed to upload file: {}", e),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn upload_file_to_cache(
        &self,
        file_upload_request: FileUploadRequest
    ) -> Result<FileUploadResponse, ZeptoMailError> {
        let endpoint: &str = "files";
        let url: String = format!("{}/{}", self.base_url, endpoint);

        // Create the multipart form with the file data
        let form = Form::new()
            .text("name", file_upload_request.name)
            .text("content_type", file_upload_request.content_type)
            .part("data", Part::bytes(file_upload_request.data).file_name("upload"));

        let response: Response = self.client
            .post(&url)
            .header("Authorization", format!("Zoho-enczapikey {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        let status: StatusCode = response.status();

        if status.is_success() {
            let response_text: String = response.text().await?;
            let success_response: FileUploadResponse = serde_json::from_str(&response_text)?;
            Ok(success_response)
        } else {
            let response_text: String = response.text().await?;
            let api_error: ApiError = serde_json::from_str(&response_text)?;
            Err(ZeptoMailError::ApiError(api_error))
        }
    }
}