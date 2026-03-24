use execution::types::AccountInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceSetLeverageResponse {
    pub leverage: u32,
    pub _max_notional_value: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAsset {
    pub asset: String,

    #[serde(default)]
    pub wallet_balance: Option<String>,

    #[serde(default)]
    pub unrealized_profit: Option<String>,

    #[serde(flatten)]
    pub _extra: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesPosition {
    pub symbol: String,

    #[serde(default)]
    pub position_amt: Option<String>,

    #[serde(default)]
    pub entry_price: Option<String>,

    #[serde(default)]
    pub unrealized_profit: Option<String>,

    #[serde(flatten)]
    pub _extra: serde_json::Value,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturesAccountInfo {
    pub total_wallet_balance: String,

    #[serde(default)]
    pub available_balance: Option<String>,

    #[serde(default)]
    pub assets: Vec<FuturesAsset>,

    #[serde(default)]
    pub positions: Vec<FuturesPosition>,

    #[serde(flatten)]
    pub _extra: serde_json::Value,
}

impl From<FuturesAccountInfo> for AccountInfo {
    fn from(value: FuturesAccountInfo) -> Self {
        AccountInfo {
            balance: value.total_wallet_balance.parse().unwrap_or(0.0),
            available_balance: match value.available_balance {
                Some(value) => value.parse().unwrap_or(0.0),
                None => 0.0,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracketResponse {
    pub symbol: String,
    pub brackets: Vec<LeverageBracket>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    pub bracket: u32,
    pub initial_leverage: u32,
    pub notional_cap: f64,
    pub notional_floor: f64,
    pub maint_margin_ratio: f64,
    pub cum: f64,
}
