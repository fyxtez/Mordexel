use uuid::Uuid;

use crate::{
    side::Side,
    symbol::Symbol,
    timeframe::Timeframe,
    trade_intent::{TradeIntent, error::TradeIntentError},
};

pub struct TradeIntentBuilder {
    symbol: Symbol,
    side: Option<Side>,
    entry: Option<f64>,
    targets: Option<Vec<f64>>,
    timeframe: Option<Timeframe>,
    stop_loss: Option<f64>,
}
impl TradeIntent {
    pub fn builder(symbol: Symbol) -> TradeIntentBuilder {
        TradeIntentBuilder {
            symbol,
            side: None,
            entry: None,
            targets: None,
            timeframe: None,
            stop_loss: None,
        }
    }
}

impl TradeIntentBuilder {
    pub fn side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn entry(mut self, entry: f64) -> Self {
        self.entry = Some(entry);
        self
    }

    pub fn stop_loss(mut self, stop_loss: f64) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    pub fn targets(mut self, targets: &[f64]) -> Self {
        self.targets = Some(targets.to_vec());
        self
    }

    pub fn timeframe(mut self, timeframe: Timeframe) -> Self {
        self.timeframe = Some(timeframe);
        self
    }

    pub fn build(self) -> Result<TradeIntent, TradeIntentError> {
        Ok(TradeIntent {
            intent_id: Uuid::new_v4(),
            symbol: self.symbol,
            side: self.side.ok_or(TradeIntentError::MissingSide)?,
            entry: self.entry.ok_or(TradeIntentError::MissingEntry)?,
            targets: self.targets.ok_or(TradeIntentError::MissingTargets)?,
            timeframe: self.timeframe.ok_or(TradeIntentError::MissingTimeframe)?,
            stop_loss: self.stop_loss.ok_or(TradeIntentError::MissingStopLoss)?,
        })
    }
}
