use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TradeRequest {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct SignalIngressRequest {
    pub source: String,
    pub external_id: Option<String>,
    pub text: String,
}
