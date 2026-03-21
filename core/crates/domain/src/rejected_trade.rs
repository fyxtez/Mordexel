use crate::trade_intent::TradeIntent;

#[derive(Debug, Clone)]
pub struct RejectedTrade {
    pub trade_intent: TradeIntent,
    pub reason: RejectionReason,
}

#[derive(Debug, Clone)]
pub enum RejectionReason {
    ExecutionPolicyDenied,
}

// TODO: Later...
// pub enum RejectionReason {
//     ExecutionPolicyDenied,
//     DuplicateSignal,
//     MaxRiskExceeded,
//     CooldownActive,
//     ExchangeUnavailable,
// }
