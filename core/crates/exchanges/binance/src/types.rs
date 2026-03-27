pub struct BinanceConfig {
    pub api_key: String,
    pub api_secret: String,
    pub base_url: String,
    pub is_test: bool,
}

#[derive(Debug, Clone)]
pub enum BinanceNetwork {
    Testnet,
    Mainnet,
    Unknown,
}
