use serde::{Serialize, Deserialize};
use crate::models::common::{EmailAddress, Attachment, MimeHeaders};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRequest {
    pub from: EmailAddress,
    pub to: Vec<Recipient>,
    pub subject: String,
    pub htmlbody: Option<String>,
    pub textbody: Option<String>,
    pub cc: Option<Vec<Recipient>>,
    pub bcc: Option<Vec<Recipient>>,
    pub track_clicks: Option<bool>,
    pub track_opens: Option<bool>,
    pub client_reference: Option<String>,
    pub mime_headers: Option<MimeHeaders>,
    pub attachments: Option<Vec<Attachment>>,
    pub inline_images: Option<Vec<InlineImage>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchEmailRequest {
    pub from: EmailAddress,
    pub to: Vec<Recipient>,
    pub subject: String,
    pub htmlbody: Option<String>,
    pub textbody: Option<String>,
    pub cc: Option<Vec<Recipient>>,
    pub bcc: Option<Vec<Recipient>>,
    pub track_clicks: Option<bool>,
    pub track_opens: Option<bool>,
    pub client_reference: Option<String>,
    pub mime_headers: Option<MimeHeaders>,
    pub attachments: Option<Vec<Attachment>>,
    pub inline_images: Option<Vec<InlineImage>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub email_address: EmailAddress,
    pub merge_info: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineImage {
    pub mime_type: String,
    pub content: String,    // Base64 encoded content
    pub cid: String,        // Content ID used in the HTML body
}
