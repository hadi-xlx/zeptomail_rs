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
