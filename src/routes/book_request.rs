use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::domain::book_request::{
    BookRequestId, BookRequestName, BookRequestPhone, BookRequestTitles,
};
use crate::services::book_request as service;

#[derive(Deserialize, Debug)]
pub struct Body {
    name: BookRequestName,
    titles: BookRequestTitles,
    id: BookRequestId,
    phone: BookRequestPhone,
}

#[tracing::instrument(ret, err)]
pub async fn handler(Json(body): Json<Body>) -> Result<StatusCode, Error> {
    service::submit(&body.name, &body.titles, &body.id, &body.phone).await?;
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
