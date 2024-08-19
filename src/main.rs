use zeptomail_rs::ZeptoMailClient;
use zeptomail_rs::{EmailRequest, Recipient};
use zeptomail_rs::EmailAddress;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new ZeptoMailClient instance
    let client = ZeptoMailClient::new("your_api_key")?;

    // Define the 'sender' email address
    let sender: EmailAddress = EmailAddress {
        address: "accounts@info.zylker.com".to_string(),
        name: Some("Paula".to_string()),
    };

    // Define the 'to' email recipient
    let recipients: Vec<Recipient> = vec![Recipient {
        email_address: EmailAddress {
            address: "rudra.d@zylker.com".to_string(),
            name: Some("Rudra".to_string()),
        },
        merge_info: None,
    }];

    // Create the email request object
    let email_request = EmailRequest {
        bounce_address: Some("bounces@info.zylker.com".to_string()),
        sender,
        recipients,
        reply_to: None,
        subject: "Account Confirmation".to_string(),
        htmlbody: Some("<div><b> Kindly click on Verify Account to confirm your account </b></div>".to_string()),
        textbody: None,
        carbon_copy: None,
        blind_carbon_copy: None,
        track_clicks: Some(true),
        track_opens: Some(true),
        client_reference: None,
        mime_headers: None,
        attachments: None,
        inline_images: None,
    };

    // Send the email using the ZeptoMailClient instance
    match client.send_email(email_request).await {
        Ok(response) => println!("Email sent successfully: {:?}", response),
        Err(e) => eprintln!("Failed to send email: {}", e),
    }

    Ok(())
}
