use serde::Deserialize;

#[derive(Deserialize)]
pub struct TradeRequest {
    pub text: String,
}
