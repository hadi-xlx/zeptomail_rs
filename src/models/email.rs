use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{EmailAddress, Attachment, MimeHeaders};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRequest {
    //A valid bounce email address as configured in your Mail Agent.
    pub bounce_address: Option<String>, 
    //A valid sender email address with "address" and "name" key-value pairs.
    pub sender: EmailAddress,           
    pub recipients: Vec<Recipient>,
    pub reply_to: Option<Vec<EmailAddress>>,
    //The subject of the email to be sent.
    pub subject: String, 
    //The HTML body of the email to be sent.
    pub htmlbody: Option<String>, 
    pub textbody: Option<String>,
    pub carbon_copy: Option<Vec<Recipient>>,
    pub blind_carbon_copy: Option<Vec<Recipient>>,
    //You can also enable email click tracking in your Mail Agent under Email Tracking section.
    //Note: The API setting will override the Mail Agent settings in your ZeptoMail account. 
    pub track_clicks: Option<bool>, 
    //You can also enable email open tracking in your Mail Agent under Email Tracking section.
    //Note: The API setting will override the Mail Agent settings in your ZeptoMail account. 
    pub track_opens: Option<bool>,
    //An identifier set by the user to track a particular transaction.
    pub client_reference: Option<String>, 
    pub mime_headers: Option<MimeHeaders>, 
    pub attachments: Option<Vec<Attachment>>,
    pub inline_images: Option<Vec<InlineImage>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchEmailRequest {
    pub sender: EmailAddress,
    pub recipients: Vec<Recipient>,
    pub subject: String,
    pub htmlbody: Option<String>,
    pub textbody: Option<String>,
    pub carbon_copy: Option<Vec<Recipient>>,
    pub blind_carbon_copy: Option<Vec<Recipient>>,
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
    pub merge_info: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineImage {
    pub mime_type: String,
    pub content: String,    // Base64 encoded content
    pub content_id: String,        // Content ID used in the HTML body
}