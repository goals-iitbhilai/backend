use serde::Serialize;

// List of strings for the `available` column that are considered `true`.
const POSTIVIES: &[&str] = &["true", "yes", "1"];

#[derive(Debug, Clone, Serialize)]
pub struct LibraryItem {
    pub item: String,
    pub author: String,
    pub available: bool,
}

impl LibraryItem {
    pub fn from_cells(
        item: &str,
        author: Option<&serde_json::Value>,
        available: Option<&serde_json::Value>,
    ) -> Self {
        // Parse the `author` column as string, and replace unknown values with an empty string.
        let author = author.and_then(|v| v.as_str()).unwrap_or("").to_owned();

        // Parse the `available` column as boolean, and replace unknown values with false.
        let available = available
            .and_then(|v| v.as_str())
            .map_or(false, |v| POSTIVIES.contains(&v.to_lowercase().as_str()));

        Self {
            item: item.to_owned(),
            author,
            available,
        }
    }
}
