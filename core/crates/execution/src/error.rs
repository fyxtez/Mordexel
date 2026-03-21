use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExchangeError {
    #[error("network error: {message}")]
    Network { message: String },

    #[error("authentication error: {message}")]
    Authentication { message: String },

    #[error("invalid request: {message}")]
    InvalidRequest { message: String },

    #[error("exchange rejected request: {message}")]
    Rejected { message: String },

    #[error("rate limited: {message}")]
    RateLimited { message: String },

    #[error("internal exchange error: {message}")]
    Internal { message: String },
}