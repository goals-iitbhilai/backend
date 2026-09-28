use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct BookRequestId(String);

impl TryFrom<String> for BookRequestId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("ID cannot be empty".to_string());
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for BookRequestId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
