pub mod builder;
pub mod error;

use uuid::Uuid;

use crate::{side::Side, symbol::Symbol, timeframe::Timeframe};

#[derive(Debug, Clone)]
pub struct TradeIntent {
    pub intent_id: Uuid,
    pub symbol: Symbol,
    pub side: Side,
    pub entry: f64,
    pub targets: Vec<f64>,
    pub timeframe: Timeframe,
    pub stop_loss: f64,
}
