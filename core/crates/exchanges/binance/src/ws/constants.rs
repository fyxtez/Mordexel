pub const TESTNET_REST_BASE: &str = "https://demo-fapi.binance.com";
pub const TESTNET_WS_BASE: &str = "wss://fstream.binancefuture.com";

pub const MAINNET_REST_BASE: &str = "https://fapi.binance.com";
pub const MAINNET_WS_BASE: &str = "wss://fstream.binance.com";

// Binance docs: listenKey valid 60 minutes, PUT extends by 60 minutes.
// Refreshing every 30 minutes is a safe margin.
pub const LISTENKEY_KEEPALIVE_SECS: u64 = 30 * 60;
pub const _RECV_WINDOW: u64 = 5_000;
