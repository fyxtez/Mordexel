use std::fmt;

use crate::trade_intent::TradeIntent;

#[derive(Debug, Clone)]
pub struct ApprovedTrade {
    pub trade_intent: TradeIntent,
}

impl fmt::Display for ApprovedTrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApprovedTrade:\n{}", self.trade_intent)
    }
}