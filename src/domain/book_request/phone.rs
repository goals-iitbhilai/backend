use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct BookRequestPhone(String);

impl TryFrom<String> for BookRequestPhone {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Phone number cannot be empty".to_string());
        }

        if !value
            .chars()
            .all(|c| c.is_ascii_digit() || c == '+' || c == ' ')
        {
            return Err("Phone number contains invalid characters".to_string());
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for BookRequestPhone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
