use std::env;

use adapter_telegram::types::{TelegramConfig, TelegramCredentials};
use binance::types::BinanceConfig;

pub fn get_build_version() -> &'static str {
    option_env!("BUILD_VERSION").unwrap_or("dev")
}

pub fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new("engine=info,adapter_telegram=info,adapter_http=info")
        }))
        .without_time()
        .init();
}

pub fn create_reqwest_client() -> reqwest::Client {
    use std::time::Duration;

    let client = reqwest::Client::builder()
        .user_agent("Pulsgram/1.0")
        .connect_timeout(Duration::from_secs(5))
        // Network safety
        // How long is the waiting to establish TCP connection to Binance.
        // Includes DNS resolution, TCP & TLS handshake
        // Not having this makes request freeze, potentially for a long time.
        .timeout(Duration::from_secs(20))
        // Prevents long requests
        .pool_idle_timeout(Duration::from_secs(30))
        // Pool tuning
        // Stops connection leak
        // How long unused connections stay alive in the pool.
        // HTTP clients reuse TCP connections for speed.
        // This cleans those up after 30 seconds if they are idle.
        .pool_max_idle_per_host(10) // Cleans dead sockets
        .tcp_keepalive(Duration::from_secs(60))
        // Keep connections alive
        // Sends periodic “I’m alive” signal on TCP connection.
        .build().unwrap(); //Allow unwrap cause request client must exist on startup. 

    client
}
