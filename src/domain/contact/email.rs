use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct ContactEmail(String);

impl TryFrom<String> for ContactEmail {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if !validator::validate_email(&value) {
            return Err("Invalid email".to_string());
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for ContactEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
