use thiserror::Error;

#[derive(Debug, Error)]
pub enum TelegramError {
    #[error("telegram invocation error: {0}")]
    Invocation(#[from] grammers_client::InvocationError),

    // NOTE:
    // `SignInError` from `grammers_client` is very large (~300+ bytes),
    // which triggers `clippy::large_enum_variant` and would bloat the size
    // of the entire `TelegramError` enum.
    //
    // We box it to keep the enum small and cheap to pass around.
    // Because of that, we implement `From<SignInError>` manually below
    // to preserve ergonomic `.into()` / `?` usage because of thiserror's [from] implementation.
    #[error("telegram sign-in error: {0}")]
    SignIn(Box<grammers_client::SignInError>),

    #[error("io error: {0}")]
    StdIO(#[from] std::io::Error),

    #[error("environment variable '{name}' error: {source}")]
    EnvVar {
        name: String,
        #[source]
        source: std::env::VarError,
    },

    #[error("failed to parse integer for '{name}': {source}")]
    ParseInt {
        name: String,
        #[source]
        source: std::num::ParseIntError,
    },

    #[error("other error: {0}")]
    Other(String),
}

impl From<grammers_client::SignInError> for TelegramError {
    fn from(err: grammers_client::SignInError) -> Self {
        TelegramError::SignIn(Box::new(err))
    }
}
