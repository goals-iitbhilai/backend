use chrono::Utc;
use webhook::client::WebhookClient;

use crate::domain::book_request::{
    BookRequestId, BookRequestName, BookRequestPhone, BookRequestTitles,
};

#[tracing::instrument(ret, err)]
pub async fn submit(
    name: &BookRequestName,
    titles: &BookRequestTitles,
    id: &BookRequestId,
    phone: &BookRequestPhone,
) -> Result<(), Error> {
    let url = std::env::var("BOOK_REQUEST_WEBHOOK")?;

    WebhookClient::new(&url)
        .send(|msg| {
            msg.username("Book Request Form").embed(|embed| {
                embed
                    .title("New Book Request")
                    .description(titles.as_ref())
                    .timestamp(&Utc::now().to_rfc3339())
                    .author(name.as_ref(), None, None)
                    .field("ID", id.as_ref(), false)
                    .field("Phone", phone.as_ref(), false)
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
