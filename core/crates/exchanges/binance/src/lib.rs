use async_trait::async_trait;
use domain::{side::Side, symbol::Symbol};
use execution::{error::ExchangeError, exchange::Exchange};

pub struct Binance;

#[async_trait]
impl Exchange for Binance {
    async fn place_market_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
    ) -> Result<(), ExchangeError> {
        let _ = (symbol, side, quantity);
        todo!()
    }

    async fn place_limit_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
        price: f64,
    ) -> Result<(), ExchangeError> {
        let _ = (symbol, side, quantity, price);
        todo!()
    }
}
