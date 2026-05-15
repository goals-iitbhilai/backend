use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct ContactMessage(String);

impl TryFrom<String> for ContactMessage {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Message cannot be empty".to_string());
        }

        Ok(ContactMessage(value))
    }
}

impl AsRef<str> for ContactMessage {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
