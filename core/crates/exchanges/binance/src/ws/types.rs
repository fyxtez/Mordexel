use serde::Deserialize;
use serde_json::Value;

use crate::types::BinanceNetwork;

#[derive(Debug, Deserialize)]
pub struct ListenKeyResponse {
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}

// TODO: Binance sometimes sends timestamps ("E", "T") as strings instead of numbers.
// We normalize both string/number → u64 via custom deserializer to avoid runtime decode errors.
#[derive(Debug, Deserialize)]
pub struct UserStreamEnvelope {
    #[serde(rename = "e")]
    pub event_type: Option<String>,

    #[serde(rename = "E")]
    pub event_time: Option<u64>,

    #[serde(rename = "T")]
    pub transaction_time: Option<u64>,

    #[serde(rename = "o")]
    pub order: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct FuturesOrderUpdate {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub client_order_id: String,

    #[serde(rename = "S")]
    pub side: String,

    #[serde(rename = "ps")]
    pub position_side: Option<String>,

    #[serde(rename = "o")]
    pub order_type: String,

    #[serde(rename = "ot")]
    pub original_order_type: Option<String>,

    #[serde(rename = "f")]
    pub time_in_force: Option<String>,

    #[serde(rename = "q")]
    pub original_qty: String,

    #[serde(rename = "p")]
    pub original_price: String,

    #[serde(rename = "ap")]
    pub average_price: Option<String>,

    #[serde(rename = "sp")]
    pub stop_price: Option<String>,

    #[serde(rename = "x")]
    pub execution_type: String,

    #[serde(rename = "X")]
    pub order_status: String,

    #[serde(rename = "i")]
    pub order_id: Option<u64>,

    #[serde(rename = "l")]
    pub last_filled_qty: Option<String>,

    #[serde(rename = "z")]
    pub accumulated_filled_qty: Option<String>,

    #[serde(rename = "L")]
    pub last_filled_price: Option<String>,

    #[serde(rename = "rp")]
    pub realized_pnl: Option<String>,

    #[serde(rename = "wt")]
    pub working_type: Option<String>,

    #[serde(rename = "R")]
    pub reduce_only: Option<bool>,

    #[serde(rename = "cp")]
    pub close_position: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct SelectedStreamConfig {
    pub rest_base: &'static str,
    pub ws_base: &'static str,
    pub api_key: String,
    pub network: BinanceNetwork,
}

#[derive(Debug, Deserialize)]
pub struct RawOrderTradeUpdate {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub client_order_id: Option<String>,

    #[serde(rename = "S")]
    pub side: Option<String>,

    #[serde(rename = "ps")]
    pub position_side: Option<String>,

    #[serde(rename = "o")]
    pub order_type: Option<String>,

    #[serde(rename = "ot")]
    pub original_order_type: Option<String>,

    #[serde(rename = "f")]
    pub time_in_force: Option<String>,

    #[serde(rename = "q")]
    pub original_qty: Option<String>,

    #[serde(rename = "p")]
    pub original_price: Option<String>,

    #[serde(rename = "ap")]
    pub average_price: Option<String>,

    #[serde(rename = "sp")]
    pub stop_price: Option<String>,

    #[serde(rename = "x")]
    pub execution_type: Option<String>,

    #[serde(rename = "X")]
    pub order_status: Option<String>,

    #[serde(rename = "i")]
    pub order_id: Option<u64>,

    #[serde(rename = "l")]
    pub last_filled_qty: Option<String>,

    #[serde(rename = "z")]
    pub accumulated_filled_qty: Option<String>,

    #[serde(rename = "L")]
    pub last_filled_price: Option<String>,

    #[serde(rename = "rp")]
    pub realized_pnl: Option<String>,

    #[serde(rename = "wt")]
    pub working_type: Option<String>,

    #[serde(rename = "R")]
    pub reduce_only: Option<bool>,

    #[serde(rename = "cp")]
    pub close_position: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
pub enum WsEventKind {
    OrderPlaced,
    PositionOpened,
    TakeProfitHit,
    StopLossHit,
    ReducedOrClosed,
    Cancelled,
    LeverageChanged,
    UnknownOrderUpdate,
}

#[derive(Debug, Clone)]
pub struct WsEvent {
    pub kind: WsEventKind,
    pub symbol: Option<String>,
    pub title: String,
    pub message: String,
    pub timestamp: u64,
}
