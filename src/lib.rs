//! # ZeptoMail SDK for Rust
//!
//! This is an unofficial Rust SDK for [ZeptoMail](https://www.zoho.com/zeptomail/), since there is no official SDK for Rust yet.
//!     
//! ## Features
//! - [**`Send Email`**](#send-email): An abstraction for operations over the [ZeptoMail Email Sending API](https://www.zoho.com/zeptomail/help/api/email-sending.html)
//! - [**`Send Batch Email`**](#send-batch-email): An abstraction for operations over the [ZeptoMail Batch Email Sending API](https://www.zoho.com/zeptomail/help/api/batch-email-sending.html)
//! - [**`Send Email with Template`**](#send-email-with-template): An abstraction for operations over the [ZeptoMail Templates API - Single Email](https://www.zoho.com/zeptomail/help/api/email-templates.html)
//! - [**`Send Batch Email with Template`**](#send-batch-email-with-template): An abstraction for operations over the [ZeptoMail Templates API - Batch Email](https://www.zoho.com/zeptomail/help/api/batch-email-templates.html)
//! - [**File Upload to Cache**](#file-upload-to-cache): An abstraction for operations over the [ZeptoMail File Cache Upload API](https://www.zoho.com/zeptomail/help/api/file-upload.html)
//! 
//! ## Usage
//! 
//! First make sure you initialize the client with your API key:
//! 
//! ```rust
//! let client = ZeptoMailClient::new("your_api_key");
//! ```
//! 
//! ### Send Email
//! 
//! ```rust
//! use zeptomail_sdk::{ZeptoMailClient, EmailRequest};
//! 
//! #[tokio::main]
//! async fn main() {
//!     let client = ZeptoMailClient::new("your_api_key");
//!     let email_request = EmailRequest {
//!         from: "sender@example.com".to_string(),
//!         to: vec!["recipient@example.com".to_string()],
//!         subject: "Test Email".to_string(),
//!         content: "This is a test email.".to_string(),
//!     };
//! 
//!     match client.send_email(email_request).await {
//!         Ok(response) => println!("Email sent successfully: {:?}", response),
//!         Err(e) => eprintln!("Error sending email: {:?}", e),
//!     }
//! }
//! ```
//! 
//! ### Send Batch Email
//! 
//! ```rust
//! use zeptomail_sdk::{ZeptoMailClient, BatchEmailRequest, EmailAddress, Recipient};
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let client = ZeptoMailClient::new("your_api_key");
//! 
//!     let sender = EmailAddress {
//!         address: "sender@example.com".to_string(),
//!         name: Some("Sender Name".to_string()),
//!     };
//! 
//!     let recipients = vec![
//!         Recipient {
//!             email_address: EmailAddress {
//!                 address: "recipient1@example.com".to_string(),
//!                 name: Some("Recipient One".to_string()),
//!             },
//!             merge_info: None,
//!         },
//!         Recipient {
//!             email_address: EmailAddress {
//!                 address: "recipient2@example.com".to_string(),
//!                 name: Some("Recipient Two".to_string()),
//!             },
//!             merge_info: None,
//!         },
//!     ];
//! 
//!     let batch_email_request = BatchEmailRequest {
//!         sender,
//!         recipients,
//!         subject: "Batch Email Test".to_string(),
//!         htmlbody: Some("<div>Batch Email Content</div>".to_string()),
//!         textbody: Some("Batch Email Content".to_string()),
//!         carbon_copy: None,
//!         blind_carbon_copy: None,
//!         track_clicks: Some(true),
//!         track_opens: Some(true),
//!         client_reference: None,
//!         mime_headers: None,
//!         attachments: None,
//!         inline_images: None,
//!     };
//! 
//!     match client.send_batch_email(batch_email_request).await {
//!         Ok(response) => println!("Email sent successfully: {:?}", response),
//!         Err(e) => eprintln!("Failed to send email: {}", e),
//!     }
//! 
//!     Ok(())
//! }
//! ```
//!
//! ### Send Email with Template
//! 
//! ```rust
//! use zeptomail_sdk::{ZeptoMailClient, TemplateEmailRequest, EmailAddress, Recipient};
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let client = ZeptoMailClient::new("your_api_key");
//! 
//!     let template_email_request = TemplateEmailRequest {
//!         template_key: "your_template_key".to_string(),
//!         bounce_address: Some("bounce@example.com".to_string()),
//!         sender: EmailAddress::new("sender@example.com".to_string()),
//!         recipients: vec![Recipient::new("recipient@example.com".to_string())],
//!         reply_to: Some(vec![EmailAddress::new("replyto@example.com".to_string())]),
//!         track_clicks: Some(true),
//!         track_opens: Some(true),
//!         client_reference: Some("client_ref".to_string()),
//!         mime_headers: None,
//!         attachments: None,
//!         merge_info: Some(std::collections::HashMap::new()),
//!     };
//! 
//!     match client.send_template_email(template_email_request).await {
//!         Ok(response) => println!("Email sent successfully: {:?}", response),
//!         Err(e) => eprintln!("Failed to send email: {}", e),
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! ### Send Batch Email with Template
//! 
//! ```rust
//! use zeptomail_sdk::{ZeptoMailClient, BatchTemplateEmailRequest, EmailAddress, Recipient};
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let client = ZeptoMailClient::new("your_api_key");
//! 
//!     let batch_template_email_request = BatchTemplateEmailRequest {
//!         template_key: "your_template_key".to_string(),
//!         bounce_address: Some("bounce@example.com".to_string()),
//!         sender: EmailAddress::new("sender@example.com".to_string()),
//!         recipients: vec![
//!             Recipient::new("recipient1@example.com".to_string()),
//!             Recipient::new("recipient2@example.com".to_string())
//!         ],
//!         reply_to: Some(vec![EmailAddress::new("replyto@example.com".to_string())]),
//!         track_clicks: Some(true),
//!         track_opens: Some(true),
//!         client_reference: Some("client_ref".to_string()),
//!         mime_headers: None,
//!         attachments: None,
//!     };
//! 
//!     match client.send_batch_template_email(batch_template_email_request).await {
//!         Ok(response) => println!("Batch email sent successfully: {:?}", response),
//!         Err(e) => eprintln!("Failed to send batch email: {}", e),
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! ### File Upload to Cache
//! 
//! ```rust
//! use zeptomail_sdk::{ZeptoMailClient, FileUploadRequest};
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let client = ZeptoMailClient::new("your_api_key");
//! 
//!     let file_upload_request = FileUploadRequest {
//!         name: "example.txt".to_string(),
//!         content_type: "text/plain".to_string(),
//!         data: b"Hello, world!".to_vec(),
//!     };
//! 
//!     match client.upload_file_to_cache(file_upload_request).await {
//!         Ok(response) => println!("File uploaded successfully: {:?}", response),
//!         Err(e) => eprintln!("Failed to upload file: {}", e),
//!     }
//! 
//!     Ok(())
//! }
//! ```
//!
//! ## Different Operations
//! 
//! ## [Send email](./email/single/index.html)
//! 
//! ## [Send Batch Email](./email/batch/index.html)
//! 
//! ## [Send Email with Template](./templates/single/index.html)
//! 
//! ## [Send Batch Email with Template](./templates/batch/index.html)
//! 
//! ## [File Upload to Cache](./file_cache/upload/index.html)


pub mod email;
pub mod file_cache;
pub mod models;
pub mod templates;
pub mod client;

pub use client::ZeptoMailClient;
pub use models::{
    api_failure::{ApiErrorDetail, ApiError, ZeptoMailError},
    api_success::{SuccessData, ApiResponse},
    common::{EmailAddress, Attachment, MimeHeaders},
    email::{EmailRequest, BatchEmailRequest, Recipient},
    file_cache::{FileUploadRequest, FileUploadResponse},
    template::{TemplateEmailRequest, BatchTemplateEmailRequest},
    email::InlineImage, 
};