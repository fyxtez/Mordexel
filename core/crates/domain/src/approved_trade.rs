use crate::trade_intent::TradeIntent;

#[derive(Debug, Clone)]
pub struct ApprovedTrade {
    pub trade_intent: TradeIntent,
}
