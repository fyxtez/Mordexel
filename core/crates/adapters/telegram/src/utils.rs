use std::env;

use crate::types::{TelegramConfig, TelegramCredentials};

pub fn load_telegram_config() -> TelegramConfig {
    let api_id = env::var("TELEGRAM_API_ID").expect("TELEGRAM_API_ID must be set");

    let api_hash = env::var("TELEGRAM_API_HASH").expect("TELEGRAM_API_HASH must be set");

    let phone_number =
        env::var("TELEGRAM_PHONE_NUMBER").expect("TELEGRAM_PHONE_NUMBER must be set");

    let password = env::var("TELEGRAM_PASSWORD").expect("TELEGRAM_PASSWORD must be set");

    let signals_id = env::var("LC_SIGNALS_ID").expect("LC_SIGNALS_ID must be set");

    let signal_source_id = signals_id
        .parse::<i64>()
        .expect("Could not parse LC_SIGNALS_ID");

    let api_id = api_id
        .parse::<i32>()
        .expect("Could not parse TELEGRAM_API_ID");

    TelegramConfig {
        signal_source_id,
        session_path: "mordexel.session".into(),
        credentials: TelegramCredentials {
            api_id,
            api_hash,
            phone_number,
            password,
        },
    }
}
