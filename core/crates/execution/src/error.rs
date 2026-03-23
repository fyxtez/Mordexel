use domain::symbol::Symbol;
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

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("exchange error: {0}")]
    Exchange(#[from] ExchangeError),

    #[error("unsupported symbol: {0}")]
    UnsupportedSymbol(Symbol),

    #[error("invalid scheduled entry: {message}")]
    InvalidScheduledEntry { message: String },

    #[error("internal execution error: {message}")]
    Internal { message: String },
}
