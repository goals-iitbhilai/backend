use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::domain::contact::{ContactEmail, ContactMessage, ContactName, ContactSubject};
use crate::services::contact as service;

#[derive(Deserialize, Debug)]
pub struct Body {
    name: ContactName,
    email: ContactEmail,
    subject: ContactSubject,
    message: ContactMessage,
}

#[tracing::instrument(ret, err)]
pub async fn handler(Json(body): Json<Body>) -> Result<StatusCode, Error> {
    service::submit(&body.name, &body.email, &body.subject, &body.message).await?;
    Ok(StatusCode::OK)
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] service::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
