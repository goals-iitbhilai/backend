use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct BookRequestName(String);

impl TryFrom<String> for BookRequestName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for BookRequestName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
