use chrono::Utc;
use webhook::client::WebhookClient;

use crate::domain::contact::{ContactEmail, ContactMessage, ContactName, ContactSubject};

#[tracing::instrument(ret, err)]
pub async fn submit(
    name: &ContactName,
    email: &ContactEmail,
    subject: &ContactSubject,
    message: &ContactMessage,
) -> Result<(), Error> {
    let url = std::env::var("CONTACT_WEBHOOK")?;

    WebhookClient::new(&url)
        .send(|msg| {
            msg.username("Contact Form").embed(|embed| {
                embed
                    .title("New Message")
                    .description(message.as_ref())
                    .timestamp(&Utc::now().to_rfc3339())
                    .author(name.as_ref(), None, None)
                    .field("subject", subject.as_ref(), false)
                    .field("email", email.as_ref(), false)
            })
        })
        .await?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("webhook error")]
    WebhookError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("environment error")]
    EnvironmentError(#[from] std::env::VarError),
}
