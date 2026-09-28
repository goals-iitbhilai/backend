use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct BookRequestTitles(String);

impl TryFrom<String> for BookRequestTitles {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Titles cannot be empty".to_string());
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for BookRequestTitles {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
