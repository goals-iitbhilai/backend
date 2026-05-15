use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub enum ContactSubject {
    Membership,
    Event,
    Collaboration,
    Feedback,
    Other,
}

impl TryFrom<String> for ContactSubject {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "membership" => Ok(ContactSubject::Membership),
            "event" => Ok(ContactSubject::Event),
            "collaboration" => Ok(ContactSubject::Collaboration),
            "feedback" => Ok(ContactSubject::Feedback),
            "other" => Ok(ContactSubject::Other),
            _ => Err("Invalid subject".to_string()),
        }
    }
}

impl AsRef<str> for ContactSubject {
    fn as_ref(&self) -> &str {
        match self {
            ContactSubject::Membership => "membership",
            ContactSubject::Event => "event",
            ContactSubject::Collaboration => "collaboration",
            ContactSubject::Feedback => "feedback",
            ContactSubject::Other => "other",
        }
    }
}
