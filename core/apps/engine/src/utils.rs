use adapter_http::start_api_server;
use domain::ingress_events::IngressEvent;
use tokio::sync::mpsc::Sender;
use tracing::error;

pub fn get_build_version() -> &'static str {
    option_env!("BUILD_VERSION").unwrap_or("dev")
}

pub fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(concat!(
                "info,",
                "grammers=warn,",
                "grammers_client=warn,",
                "grammers_session=warn,",
                "grammers_mtproto=warn,",
                "grammers_mtsender=warn,",
                "grammers_tl_types=warn,",
                "hyper=warn,",
                "reqwest=warn,",
                "teloxide=warn"
            ))
        }))
        .without_time()
        .try_init();
}

pub fn create_reqwest_client() -> reqwest::Client {
    use std::time::Duration;

    reqwest::Client::builder()
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
        .tcp_nodelay(true)
        // Keep connections alive
        // Sends periodic “I’m alive” signal on TCP connection.
        .build()
        .unwrap() //Allow unwrap cause request client must exist on startup. 
}

pub async fn start_server(tx: Sender<IngressEvent>, is_test: bool) {
    let address = if cfg!(feature = "production") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    let port = 8656;

    match start_api_server(address, port, tx, is_test).await {
        Ok(_) => {}
        Err(error) => {
            error!(error = %error, "Failed starting api server.");
        }
    }
}
