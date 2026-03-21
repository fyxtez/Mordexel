use async_trait::async_trait;
use domain::{side::Side, symbol::Symbol};

use crate::error::ExchangeError;

#[async_trait]
pub trait Exchange: Send + Sync {
    async fn place_market_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
    ) -> Result<(), ExchangeError>;

    async fn place_limit_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
        price: f64,
    ) -> Result<(), ExchangeError>;
}
