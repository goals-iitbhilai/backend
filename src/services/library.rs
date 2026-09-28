use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use tokio::sync::RwLock;
use tracing::info;

use crate::domain::library::LibraryItem;
use crate::repository::library::LibraryRepo;

/// The range of cells to fetch from the Google Sheet.
///
/// Using the named range `books` fetches all cells in that sheet, which is
/// appropriate here since we want almost all of the data.
const RANGE: &str = "books";

/// How long cached results are served before a fresh fetch is triggered.
const CACHE_TTL: Duration = Duration::from_mins(10);

struct Cache {
    items: Vec<LibraryItem>,
    fetched_at: Instant,
}

static CACHE: LazyLock<RwLock<Option<Cache>>> = LazyLock::new(|| RwLock::new(None));

/// Returns the full library catalogue, serving from the in-process cache when
/// the last fetch is within [`CACHE_TTL`].
#[tracing::instrument(skip(repo), err)]
pub async fn get_library_items<R: LibraryRepo>(repo: &R) -> Result<Vec<LibraryItem>, Error> {
    {
        let guard = CACHE.read().await;

        if let Some(entry) = &*guard
            && entry.fetched_at.elapsed() < CACHE_TTL
        {
            info!("serving {} items from cache", entry.items.len());
            return Ok(entry.items.clone());
        }
    }

    info!("fetching items from repository");
    let rows = repo.fetch_rows(RANGE).await?;
    let items = parse_rows(&rows)?;

    {
        let mut guard = CACHE.write().await;
        *guard = Some(Cache {
            items: items.clone(),
            fetched_at: Instant::now(),
        });
    }

    Ok(items)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("repository error: {0}")]
    Repository(#[from] crate::repository::library::Error),

    #[error("missing '{0}' column in sheet")]
    MissingColumn(String),
}

fn parse_rows(rows: &[Vec<serde_json::Value>]) -> Result<Vec<LibraryItem>, Error> {
    let headers = match rows.first() {
        Some(h) => h,
        None => return Ok(vec![]),
    };

    // Look up column indices by header name so the code is resilient to
    // changes in the column order of the sheet.
    let find_col = |name: &str| {
        headers
            .iter()
            .position(|h| h.as_str().is_some_and(|s| s == name))
            .ok_or_else(|| Error::MissingColumn(name.into()))
    };

    let item_idx = find_col("item")?;
    let author_idx = find_col("author")?;
    let available_idx = find_col("available")?;

    let items: Vec<_> = rows[1..]
        .iter()
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

    info!("parsed {} items", items.len());

    Ok(items)
}
