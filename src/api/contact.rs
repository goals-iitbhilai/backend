use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use serde::Deserialize;
use webhook::client::WebhookClient;

use crate::domain::contact::{ContactEmail, ContactMessage, ContactName, ContactSubject};

#[derive(Deserialize)]
pub struct Body {
    name: ContactName,
    email: ContactEmail,
    subject: ContactSubject,
    message: ContactMessage,
}

pub async fn handler(Json(body): Json<Body>) -> Result<StatusCode, Error> {
    let url = std::env::var("WEBHOOK_URL")?;

    WebhookClient::new(&url)
        .send(|msg| {
            msg.username("Contact Form").embed(|embed| {
                embed
                    .title("New Message")
                    .description(body.message.as_ref())
                    .timestamp(&Utc::now().to_rfc3339())
                    .author(body.name.as_ref(), None, None)
                    .field("subject", body.subject.as_ref(), false)
                    .field("email", body.email.as_ref(), false)
            })
        })
        .await?;

    Ok(StatusCode::OK)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("webhook error")]
    WebhookError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("environment error")]
    EnvironmentError(#[from] std::env::VarError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
