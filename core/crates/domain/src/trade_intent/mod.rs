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

use std::fmt;

impl fmt::Display for TradeIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TradeIntent [{}]\n  Symbol: {}\n  Side: {}\n  Entry: {}\n  Stop Loss: {}\n  Targets: [{}]\n  Timeframe: {}",
            self.intent_id,
            self.symbol,
            self.side,
            self.entry,
            self.stop_loss,
            self.targets
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.timeframe
        )
    }
}