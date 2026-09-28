use std::time::Duration;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http::header;

use crate::repository::library::SheetsLibraryRepo;
use crate::services::library as service;

const CACHE_AGE: Duration = Duration::from_mins(30);

#[tracing::instrument(err)]
pub async fn handler() -> Result<impl IntoResponse, Error> {
    let headers = [(
        header::CACHE_CONTROL,
        format!("public, max-age={}", CACHE_AGE.as_secs()),
    )];

    let repo = SheetsLibraryRepo::from_env()?;
    let items = service::get_library_items(&repo).await?;

    Ok((headers, Json(items)))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Repository(#[from] crate::repository::library::Error),

    #[error(transparent)]
    Service(#[from] service::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
