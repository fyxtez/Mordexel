mod constants;
mod error;
mod transport;
pub mod utils;
pub mod types;
pub mod client;

use async_trait::async_trait;
use domain::{side::Side, symbol::Symbol};
use execution::{error::ExchangeError, exchange::Exchange};

use crate::{client::BinanceClient, error::BinanceError};

pub struct Binance{
    pub client: BinanceClient
}

#[async_trait]
impl Exchange for Binance {
    async fn account_info(&self) -> Result<(), ExchangeError> {
        self.try_account_info().await?;
        Ok(())
    }

    async fn place_market_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
    ) -> Result<(), ExchangeError> {
        self.try_place_market_order(symbol, side, quantity).await?;
        Ok(())
    }

    async fn place_limit_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
        price: f64,
    ) -> Result<(), ExchangeError> {
        self.try_place_limit_order(symbol, side, quantity, price).await?;
        Ok(())
    }
}

impl Binance {
    async fn try_account_info(&self) -> Result<(), BinanceError> {
        Ok(())
    }

    async fn try_place_market_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
    ) -> Result<(), BinanceError> {
        let _ = (symbol, side, quantity);
        todo!()
    }

    async fn try_place_limit_order(
        &self,
        symbol: Symbol,
        side: Side,
        quantity: f64,
        price: f64,
    ) -> Result<(), BinanceError> {
        let _ = (symbol, side, quantity, price);
        todo!()
    }
}