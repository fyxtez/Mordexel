use reqwest::header::{HeaderMap, HeaderValue};

use crate::{
    error::{BinanceApiError, BinanceError},
    ws::types::ListenKeyResponse,
};

fn api_headers(api_key: &str) -> Result<HeaderMap, BinanceError> {
    if api_key.trim().is_empty() {
        return Err(BinanceError::InvalidInput(
            "Binance API key is empty".to_string(),
        ));
    }

    let mut headers = HeaderMap::new();

    let header_value = HeaderValue::from_str(api_key).map_err(|e| {
        BinanceError::InvalidInput(format!("invalid Binance API key header value: {e}"))
    })?;

    headers.insert("X-MBX-APIKEY", header_value);

    Ok(headers)
}

pub async fn create_listen_key(
    client: &reqwest::Client,
    rest_base: &str,
    api_key: &str,
) -> Result<String, BinanceError> {
    let url = format!("{rest_base}/fapi/v1/listenKey");

    let response = client
        .post(&url)
        .headers(api_headers(api_key)?)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        let api_error = serde_json::from_str::<BinanceApiError>(&text).unwrap_or(BinanceApiError {
            code: status.as_u16() as i64,
            msg: text,
        });

        return Err(BinanceError::Api(api_error));
    }

    let parsed: ListenKeyResponse = serde_json::from_str(&text)?;

    Ok(parsed.listen_key)
}

pub async fn keepalive_listen_key(
    client: &reqwest::Client,
    rest_base: &str,
    api_key: &str,
) -> Result<(), BinanceError> {
    let url = format!("{rest_base}/fapi/v1/listenKey");

    let response = client
        .put(&url)
        .headers(api_headers(api_key)?)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        let api_error = serde_json::from_str::<BinanceApiError>(&text).unwrap_or(BinanceApiError {
            code: status.as_u16() as i64,
            msg: text,
        });

        return Err(BinanceError::Api(api_error));
    }

    if !text.trim().is_empty() {
        tracing::info!(
            response = %mask_middle(&text),
            "listen key keepalive response"
        );
    }

    Ok(())
}

fn mask_middle(s: &str) -> String {
    if s.len() <= 10 {
        return "***".to_string();
    }
    format!("{}...{}", &s[..6], &s[s.len() - 6..])
}

#[allow(dead_code)]
pub async fn close_listen_key(
    client: &reqwest::Client,
    rest_base: &str,
    api_key: &str,
    listen_key: &str,
) -> Result<(), BinanceError> {
    if listen_key.trim().is_empty() {
        return Err(BinanceError::InvalidInput(
            "listen key is empty".to_string(),
        ));
    }

    let url = format!(
        "{rest_base}/fapi/v1/listenKey?listenKey={}",
        urlencoding::encode(listen_key)
    );

    let response = client
        .delete(&url)
        .headers(api_headers(api_key)?)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        let api_error = serde_json::from_str::<BinanceApiError>(&text).unwrap_or(BinanceApiError {
            code: status.as_u16() as i64,
            msg: text,
        });

        return Err(BinanceError::Api(api_error));
    }

    tracing::info!("listen key closed");

    if !text.trim().is_empty() {
        tracing::info!(response = %text, "close listen key response");
    }

    Ok(())
}
