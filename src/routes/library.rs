use http::header;
use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};
use tracing::info;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use google_sheets4::hyper_util::{client::legacy::Client, rt::TokioExecutor};
use google_sheets4::yup_oauth2::ServiceAccountAuthenticator;
use google_sheets4::{Sheets, hyper_rustls::HttpsConnectorBuilder};
use tokio::sync::RwLock;

use crate::domain::library::LibraryItem;

// The range of cells to fetch from the Google Sheet.
//
// The range `books` fetches ALL cells in the `books` sheet. This is fine
// for our use case, since we want almost all the data anyways.
const RANGE: &str = "books";

// The time-to-live (TTL) for the cache.
//
// This is the maximum amount of time that cached results will be returned
// before they are refreshed.
const CACHE_TTL: Duration = Duration::from_mins(10);

struct Cache {
    items: Vec<LibraryItem>,
    fetched_at: Instant,
}

// Store the global cache in a lazy lock-protected RwLock.
static CACHE: LazyLock<RwLock<Option<Cache>>> = LazyLock::new(|| RwLock::new(None));

#[tracing::instrument]
pub async fn handler() -> Result<impl IntoResponse, Error> {
    let headers = [(
        header::CACHE_CONTROL,
        format!("public, max-age={}", CACHE_TTL.as_secs()),
    )];

    // If the last call to the Sheets API was within the TTL deadline,
    // return the cached results.
    //
    // The scoped block ensures we release the lock as soon as we're done with it.
    {
        let guard = CACHE.read().await;

        if let Some(entry) = &*guard
            && entry.fetched_at.elapsed() < CACHE_TTL
        {
            info!("serving {} items from cache", entry.items.len());
            return Ok((headers, Json(entry.items.clone())));
        }
    }

    info!("fetching items from Google Sheets");
    let items = fetch_library_items().await?;

    // Store the new results in the cache.
    {
        let mut guard = CACHE.write().await;

        *guard = Some(Cache {
            items: items.clone(),
            fetched_at: Instant::now(),
        });
    }

    Ok((headers, Json(items)))
}

#[tracing::instrument(err)]
async fn fetch_library_items() -> Result<Vec<LibraryItem>, Error> {
    // Load environment variables.
    let key_json_base64 = std::env::var("GOOGLE_SERVICE_ACCOUNT_KEY")?;
    let sheet_id = std::env::var("GOOGLE_SHEET_ID")?;

    // Create an authenticator using the stored service account key.
    let auth = {
        let bytes = STANDARD.decode(key_json_base64.trim()).unwrap();
        let key_json = String::from_utf8(bytes).unwrap();
        let sa_key: yup_oauth2::ServiceAccountKey =
            serde_json::from_str(&key_json).map_err(Error::SaKeyParsing)?;

        ServiceAccountAuthenticator::builder(sa_key)
            .build()
            .await
            .map_err(Error::AuthError)?
    };

    // Create an HTTP client.
    let client = {
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .map_err(Error::HttpError)?
            .https_or_http()
            .enable_http2()
            .build();

        Client::builder(TokioExecutor::new()).build(connector)
    };

    // Fetch library items from the Google Sheets API.
    let (_, value_range) = Sheets::new(client, auth)
        .spreadsheets()
        .values_get(&sheet_id, RANGE)
        .doit()
        .await
        .map_err(Box::new)
        .map_err(Error::SheetsError)?;

    // Parse the fetched rows into `LibraryItem` instances.
    parse_rows(&value_range.values.unwrap_or_default())
}

fn parse_rows(rows: &[Vec<serde_json::Value>]) -> Result<Vec<LibraryItem>, Error> {
    // Extract the table headers from the first row.
    let headers = match rows.first() {
        Some(headers) => headers,
        None => return Ok(vec![]),
    };

    // Helper function to find indices of columns by name.
    //
    // This allows us to modify the overall layout of the sheet
    // without needing to update the code.
    let find_col = |name: &str| {
        headers
            .iter()
            .position(|h| h.as_str().is_some_and(|s| s == name))
            .ok_or_else(|| Error::MissingColumn(name.into()))
    };

    let item_idx = find_col("item")?;
    let author_idx = find_col("author")?;
    let available_idx = find_col("available")?;

    // Map the rows into `LibraryItem` instances.
    let items: Vec<_> = rows[1..]
        .iter()
        // We only return items that have a valid `item` value.
        .filter_map(|row| {
            Some(LibraryItem::from_cells(
                row.get(item_idx)
                    .and_then(|v| v.as_str())
                    .filter(|v| !v.is_empty())?,
                row.get(author_idx),
                row.get(available_idx),
            ))
        })
        .collect();

    info!("fetched {} items", items.len());

    Ok(items)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("environment variables not present")]
    EnvironmentError(#[from] std::env::VarError),

    #[error("failed to parse service account key: {0}")]
    SaKeyParsing(#[source] serde_json::Error),

    #[error("failed to create authenticator: {0}")]
    AuthError(#[source] std::io::Error),

    #[error("http client error: {0}")]
    HttpError(#[source] std::io::Error),

    #[error("sheets error: {0}")]
    SheetsError(#[from] Box<google_sheets4::Error>),

    #[error("missing '{0}' column from sheet")]
    MissingColumn(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
