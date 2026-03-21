pub mod client;
mod constants;
mod endpoints;
mod error;
mod transport;
pub mod types;
pub mod utils;

use async_trait::async_trait;
use domain::{side::Side, symbol::Symbol};
use execution::{error::ExchangeError, exchange::Exchange, types::AccountInfo};
use reqwest::Method;

use crate::{
    client::BinanceClient, endpoints::ACCOUNT_INFO, error::BinanceError, types::FuturesAccountInfo,
};

pub struct Binance {
    pub client: BinanceClient,
}

#[async_trait]
impl Exchange for Binance {
    async fn account_info(&self) -> Result<AccountInfo, ExchangeError> {
        let raw = self.try_account_info().await?;
        Ok(raw.into())
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
        self.try_place_limit_order(symbol, side, quantity, price)
            .await?;
        Ok(())
    }
}

impl Binance {
    async fn try_account_info(&self) -> Result<FuturesAccountInfo, BinanceError> {
        self.client
            .transport()
            .signed(Method::GET, ACCOUNT_INFO, String::new())
            .await
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
