use std::env;

use hmac::{Hmac, Mac};
use reqwest::Method;
use sha2::Sha256;

use crate::{constants::{FUTURES, TESTNET_FUTURES}, error::{BinanceApiError, BinanceError}, types::BinanceConfig};

const DEFAULT_RECV_WINDOW: u64 = 5000;

fn get_timestamp() -> u128 {
    // We expect system time to always be after UNIX_EPOCH (1970-01-01).
    // If this fails, the machine's clock is misconfigured, which is a fatal
    // environment issue — not something the application should recover from.
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock is before UNIX_EPOCH — check system time configuration")
        .as_millis()
}

fn create_signature(query_string: &str, secret: &str) -> String {
    // HMAC accepts keys of any size. If this fails, something is
    // fundamentally wrong with the crypto configuration.
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("Failed to create HMAC-SHA256 instance — invalid secret key");

    mac.update(query_string.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

pub async fn send_signed_request(
    client: &reqwest::Client,
    method: Method,
    base_url: &str,
    endpoint: &str,
    api_key: &str,
    api_secret: &str,
    mut query_string: String,
) -> Result<reqwest::Response, BinanceError> {
    let timestamp = get_timestamp();

    if !query_string.is_empty() {
        query_string.push('&');
    }

    // recvWindow=5000
    // Allows up to 5 seconds difference between request timestamp
    // and Binance server time.
    // Prevents -1021 timestamp errors caused by latency or small clock drift.
    query_string.push_str(&format!("recvWindow={}", DEFAULT_RECV_WINDOW));
    query_string.push('&');
    query_string.push_str(&format!("timestamp={}", timestamp));

    let signature = create_signature(&query_string, api_secret);

    let url = format!(
        "{}/{}?{}&signature={}",
        base_url, endpoint, query_string, signature
    );

    let response = client
        .request(method, url)
        .header("X-MBX-APIKEY", api_key)
        .send()
        .await?;

    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?;

        // Try parsing structured Binance error
        if let Ok(api_err) = serde_json::from_str::<BinanceApiError>(&body) {
            return Err(BinanceError::Api(api_err));
        }

        // Fallback if Binance returns something unexpected
        return Err(BinanceError::Api(BinanceApiError {
            code: -1,
            msg: body,
        }));
    }

    Ok(response)
}

pub fn build_query(params: &[(&str, String)]) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}


pub fn load_binance_config(is_test: bool) -> BinanceConfig {
    let (key_name, secret_name, base_url) = if is_test {
        (
            "BINANCE_API_KEY_TEST",
            "BINANCE_API_SECRET_TEST",
            TESTNET_FUTURES,
        )
    } else {
        (
            "BINANCE_API_KEY",
            "BINANCE_API_SECRET",
            FUTURES,
        )
    };

    let api_key = env::var(key_name)
        .unwrap_or_else(|_| panic!("{key_name} must be set"));

    let api_secret = env::var(secret_name)
        .unwrap_or_else(|_| panic!("{secret_name} must be set"));

    BinanceConfig {
        api_key,
        api_secret,
        base_url: base_url.to_string(),
    }
}


#[cfg(test)]
mod tests_timestamp {
    use super::*;

    #[test]
    fn test_timestamp_is_positive() {
        let ts = get_timestamp();
        assert!(ts > 0);
    }

    #[test]
    fn test_timestamp_is_reasonable() {
        // Assert timestamp is after 2020-01-01 and before 2100-01-01
        let ts = get_timestamp();
        assert!(ts > 1_577_836_800_000); // 2020-01-01 in ms
        assert!(ts < 4_102_444_800_000); // 2100-01-01 in ms
    }

    #[test]
    fn test_timestamps_are_monotonic() {
        let ts1 = get_timestamp();
        let ts2 = get_timestamp();
        assert!(ts2 >= ts1);
    }

    #[test]
    fn test_timestamps_increase_over_time() {
        let ts1 = get_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let ts2 = get_timestamp();
        assert!(ts2 > ts1);
    }
}

#[cfg(test)]
mod tests_signature {
    use super::*;

    #[test]
    fn test_signature_is_valid_hex() {
        let sig = create_signature("foo=bar&baz=qux", "secret");
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_signature_is_correct_length() {
        // HMAC-SHA256 produces 32 bytes = 64 hex characters
        let sig = create_signature("foo=bar", "secret");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_signature_is_deterministic() {
        let sig1 = create_signature("foo=bar&baz=qux", "secret");
        let sig2 = create_signature("foo=bar&baz=qux", "secret");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_different_query_strings_produce_different_signatures() {
        let sig1 = create_signature("foo=bar", "secret");
        let sig2 = create_signature("foo=baz", "secret");
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_different_secrets_produce_different_signatures() {
        let sig1 = create_signature("foo=bar", "secret1");
        let sig2 = create_signature("foo=bar", "secret2");
        assert_ne!(sig1, sig2);
    }

    #[test]
    fn test_known_value() {
        // Pre-computed: echo -n "symbol=BTCUSDT&side=BUY" | openssl dgst -sha256 -hmac "mysecret"
        let sig = create_signature("symbol=BTCUSDT&side=BUY", "mysecret");
        assert_eq!(
            sig,
            "f0fe50c8f82b55b3da13325f82379ff550b523c5853d73595f2a848688bd3434"
        );
    }

    #[test]
    fn test_empty_query_string() {
        let sig = create_signature("", "secret");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_empty_secret() {
        let sig = create_signature("foo=bar", "");
        assert_eq!(sig.len(), 64);
    }

    #[test]
    fn test_special_characters_in_query() {
        // URL-encoded values should be treated as plain strings
        let sig1 = create_signature("foo=hello%20world", "secret");
        let sig2 = create_signature("foo=hello%20world", "secret");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_parameter_order_matters() {
        let sig1 = create_signature("a=1&b=2", "secret");
        let sig2 = create_signature("b=2&a=1", "secret");
        assert_ne!(sig1, sig2);
    }
}
