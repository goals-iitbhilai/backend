use base64::{Engine, engine::general_purpose::STANDARD};
use google_sheets4::{
    Sheets,
    hyper_rustls::HttpsConnectorBuilder,
    hyper_util::{client::legacy::Client, rt::TokioExecutor},
    yup_oauth2::ServiceAccountAuthenticator,
};
use serde_json::Value;

pub trait LibraryRepo {
    fn fetch_rows(
        &self,
        range: &str,
    ) -> impl Future<Output = Result<Vec<Vec<Value>>, Error>> + Send;
}

pub struct SheetsLibraryRepo {
    key_json_base64: String,
    sheet_id: String,
}

impl SheetsLibraryRepo {
    pub fn from_env() -> Result<Self, Error> {
        let key_json_base64 = std::env::var("GOOGLE_SERVICE_ACCOUNT_KEY")?;
        let sheet_id = std::env::var("GOOGLE_SHEET_ID")?;

        Ok(Self {
            key_json_base64,
            sheet_id,
        })
    }
}

impl LibraryRepo for SheetsLibraryRepo {
    #[tracing::instrument(skip(self), err)]
    async fn fetch_rows(&self, range: &str) -> Result<Vec<Vec<Value>>, Error> {
        let auth = {
            let bytes = STANDARD.decode(self.key_json_base64.trim()).unwrap();
            let key_json = String::from_utf8(bytes).unwrap();
            let sa_key: yup_oauth2::ServiceAccountKey =
                serde_json::from_str(&key_json).map_err(Error::SaKeyParsing)?;

            ServiceAccountAuthenticator::builder(sa_key)
                .build()
                .await
                .map_err(Error::AuthError)?
        };

        let client = {
            let connector = HttpsConnectorBuilder::new()
                .with_native_roots()
                .map_err(Error::HttpError)?
                .https_or_http()
                .enable_http2()
                .build();

            Client::builder(TokioExecutor::new()).build(connector)
        };

        let (_, value_range) = Sheets::new(client, auth)
            .spreadsheets()
            .values_get(&self.sheet_id, range)
            .doit()
            .await
            .map_err(Box::new)
            .map_err(Error::SheetsError)?;

        Ok(value_range.values.unwrap_or_default())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("required environment variable not set")]
    EnvironmentError(#[from] std::env::VarError),

    #[error("failed to parse service account key: {0}")]
    SaKeyParsing(#[source] serde_json::Error),

    #[error("failed to build authenticator: {0}")]
    AuthError(#[source] std::io::Error),

    #[error("failed to build HTTP client: {0}")]
    HttpError(#[source] std::io::Error),

    #[error("Google Sheets API error: {0}")]
    SheetsError(#[from] Box<google_sheets4::Error>),
}
