use crate::trade_intent::TradeIntent;

#[derive(Debug, Clone)]
pub struct RejectedTrade {
    pub trade_intent: TradeIntent,
    pub reason: RejectionReason,
}

#[derive(Debug, Clone)]
pub enum RejectionReason {
    SymbolNotAllowed,
    BlockedSession,
    BlockedWeekday,
}
