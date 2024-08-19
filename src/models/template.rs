use serde::{Serialize, Deserialize};
use crate::models::common::{EmailAddress, Attachment, MimeHeaders};
use crate::models::email::Recipient;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateEmailRequest {
    pub template_key: String,
    pub bounce_address: Option<String>,
    pub from: EmailAddress,
    pub to: Vec<Recipient>,
    pub reply_to: Option<Vec<EmailAddress>>,
    pub track_clicks: Option<bool>,
    pub track_opens: Option<bool>,
    pub client_reference: Option<String>,
    pub mime_headers: Option<MimeHeaders>,
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchTemplateEmailRequest {
    pub template_key: String,
    pub bounce_address: Option<String>,
    pub from: EmailAddress,
    pub to: Vec<Recipient>,
    pub reply_to: Option<Vec<EmailAddress>>,
    pub track_clicks: Option<bool>,
    pub track_opens: Option<bool>,
    pub client_reference: Option<String>,
    pub mime_headers: Option<MimeHeaders>,
    pub attachments: Option<Vec<Attachment>>,
}
