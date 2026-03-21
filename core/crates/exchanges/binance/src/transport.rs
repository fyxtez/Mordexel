use reqwest::Method;
use serde::de::DeserializeOwned;

use crate::{error::BinanceError, utils::send_signed_request};

pub struct Transport<'a> {
    pub client: &'a reqwest::Client,
    pub base_url: &'a str,
    pub api_key: &'a str,
    pub api_secret: &'a str,
}

impl<'a> Transport<'a> {
    pub async fn signed<T: DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        query: String,
    ) -> Result<T, BinanceError> {
        let response = send_signed_request(
            self.client,
            method,
            self.base_url,
            endpoint,
            self.api_key,
            self.api_secret,
            query,
        )
        .await?;

        let text = response.text().await?;
        parse_binance_json::<T>(&text)
    }
}

/// Detects `{ code: <0, msg: ... }` and returns BinanceError::Api
fn parse_binance_json<T: DeserializeOwned>(raw: &str) -> Result<T, BinanceError> {
    let value: serde_json::Value = serde_json::from_str(raw)?;

    if let Some(code) = value.get("code").and_then(|c| c.as_i64())
        && code < 0
    {
        let api_err = serde_json::from_value(value)?;
        return Err(BinanceError::Api(api_err));
    }

    Ok(serde_json::from_value(value)?)
}
