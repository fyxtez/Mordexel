use crate::{transport::Transport, types::BinanceConfig};

#[derive(Clone, Debug)]
pub struct BinanceClient {
    pub request_client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub is_test: bool,
}

impl BinanceClient {
    pub fn new(client: reqwest::Client, config: BinanceConfig) -> Self {
        Self {
            request_client: client,
            base_url: config.base_url.to_string(),
            api_key: config.api_key.to_string(),
            api_secret: config.api_secret.to_string(),
            is_test: config.is_test,
        }
    }

    pub(crate) fn transport(&self) -> Transport<'_> {
        Transport {
            client: &self.request_client,
            base_url: &self.base_url,
            api_key: &self.api_key,
            api_secret: &self.api_secret,
        }
    }
}
