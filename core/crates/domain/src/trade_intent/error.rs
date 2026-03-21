use std::fmt;

#[derive(Debug)]
pub enum TradeIntentError {
    MissingSide,
    MissingEntry,
    MissingTargets,
    MissingTimeframe,
    MissingStopLoss,
}

impl fmt::Display for TradeIntentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeIntentError::MissingSide => write!(f, "Side is missing"),
            TradeIntentError::MissingEntry => write!(f, "Entry price is missing"),
            TradeIntentError::MissingTargets => write!(f, "Targets are missing"),
            TradeIntentError::MissingTimeframe => write!(f, "Timeframe is missing"),
            TradeIntentError::MissingStopLoss => write!(f, "Stop loss is missing"),
        }
    }
}
