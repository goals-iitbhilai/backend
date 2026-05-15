use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct ContactName(String);

impl TryFrom<String> for ContactName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        Ok(ContactName(value))
    }
}

impl AsRef<str> for ContactName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
