use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use serde::Deserialize;
use webhook::client::WebhookClient;

use crate::domain::book_request::{
    BookRequestId, BookRequestName, BookRequestPhone, BookRequestTitles,
};

#[derive(Deserialize, Debug)]
pub struct Body {
    name: BookRequestName,
    titles: BookRequestTitles,
    id: BookRequestId,
    phone: BookRequestPhone,
}

#[tracing::instrument(ret, err)]
pub async fn handler(Json(body): Json<Body>) -> Result<StatusCode, Error> {
    let url = std::env::var("BOOK_REQUEST_WEBHOOK")?;

    WebhookClient::new(&url)
        .send(|msg| {
            msg.username("Book Request Form").embed(|embed| {
                embed
                    .title("New Book Request")
                    .description(body.titles.as_ref())
                    .timestamp(&Utc::now().to_rfc3339())
                    .author(body.name.as_ref(), None, None)
                    .field("ID", body.id.as_ref(), false)
                    .field("Phone", body.phone.as_ref(), false)
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
