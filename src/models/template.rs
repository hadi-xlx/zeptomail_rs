use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Attachment, EmailAddress, MimeHeaders, Recipient};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateEmailRequest {
    pub template_key: String,
    pub bounce_address: Option<String>,
    pub sender: EmailAddress,
    pub recipients: Vec<Recipient>,
    pub reply_to: Option<Vec<EmailAddress>>,
    pub track_clicks: Option<bool>,
    pub track_opens: Option<bool>,
    pub client_reference: Option<String>,
    pub mime_headers: Option<MimeHeaders>,
    pub attachments: Option<Vec<Attachment>>,
    pub merge_info: Option<HashMap<String, String>>, 

}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchTemplateEmailRequest {
    pub template_key: String,
    pub bounce_address: Option<String>,
    pub sender: EmailAddress,
    pub recipients: Vec<Recipient>,
    pub reply_to: Option<Vec<EmailAddress>>,
    pub track_clicks: Option<bool>,
    pub track_opens: Option<bool>,
    pub client_reference: Option<String>,
    pub mime_headers: Option<MimeHeaders>,
    pub attachments: Option<Vec<Attachment>>,
}
