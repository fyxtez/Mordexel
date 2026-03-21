use thiserror::Error;

#[derive(Debug, Error)]
pub enum BinanceError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Missing field in response: {0}")]
    MissingField(&'static str),

    #[error("Binance API error {0}")]
    Api(BinanceApiError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

// Implemented here so execution does not depend on exchanges, but rather specific exchange converts its error to ExchangeError
use execution::error::ExchangeError;

impl From<BinanceError> for ExchangeError {
    fn from(value: BinanceError) -> Self {
        match value {
            BinanceError::Http(err) => ExchangeError::Network {
                message: err.to_string(),
            },
            BinanceError::Json(err) => ExchangeError::Internal {
                message: err.to_string(),
            },
            BinanceError::MissingField(field) => ExchangeError::Internal {
                message: format!("missing field: {field}"),
            },
            BinanceError::Api(api_err) => ExchangeError::Rejected {
                message: api_err.to_string(),
            },
            BinanceError::InvalidInput(msg) => ExchangeError::InvalidRequest { message: msg },
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct BinanceApiError {
    pub code: i64,
    pub msg: String,
}

impl std::fmt::Display for BinanceApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}): {}", self.code, self.msg)
    }
}
