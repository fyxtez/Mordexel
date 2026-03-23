pub mod client;
mod constants;
mod endpoints;
mod error;
mod response_types;
mod transport;
pub mod types;
pub mod utils;

use async_trait::async_trait;
use domain::{side::Side, symbol::Symbol};
use execution::{
    error::ExchangeError,
    exchange::Exchange,
    types::{AccountInfo, SetLeverageResponse as ExchangeSetLeverageResponse},
};
use reqwest::Method;
use serde_json::Value;

use crate::{
    client::BinanceClient,
    endpoints::{ACCOUNT_INFO, ALGO_ORDER, LEVERAGE, ORDER},
    error::BinanceError,
    response_types::{BinanceSetLeverageResponse, FuturesAccountInfo},
    utils::build_query,
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

    async fn set_leverage(
        &self,
        symbol: &Symbol,
        leverage: u32,
    ) -> Result<ExchangeSetLeverageResponse, ExchangeError> {
        let response = self.try_set_leverage(symbol, leverage).await?;

        Ok(ExchangeSetLeverageResponse {
            leverage: response.leverage,
            symbol: response.symbol,
        })
    }

    //TODO: Type
    async fn place_market_order(
        &self,
        symbol: &Symbol,
        side: Side,
        quantity: f64,
    ) -> Result<(), ExchangeError> {
        self.try_place_market_order(symbol, side, quantity).await?;
        Ok(())
    }

    async fn place_stop_loss_order(
        &self,
        symbol: &Symbol,
        side: Side, // opposite of position side
        stop_price: f64,
    ) -> Result<(), ExchangeError> {
        self.try_place_stop_loss_order(symbol, side, stop_price)
            .await?;
        Ok(())
    }
}

impl Binance {
    async fn try_set_leverage(
        &self,
        symbol: &Symbol,
        leverage: u32,
    ) -> Result<BinanceSetLeverageResponse, BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("leverage", leverage.to_string()),
        ]);

        self.client
            .transport()
            .signed(Method::POST, LEVERAGE, query)
            .await
    }

    async fn try_account_info(&self) -> Result<FuturesAccountInfo, BinanceError> {
        self.client
            .transport()
            .signed(Method::GET, ACCOUNT_INFO, String::new())
            .await
    }

    async fn try_place_market_order(
        &self,
        symbol: &Symbol,
        side: Side,
        quantity: f64,
        //TODO: Result<MarketOrderResponse, BinanceError>
    ) -> Result<(), BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "MARKET".to_string()),
            ("quantity", quantity.to_string()),
            ("newOrderRespType", "RESULT".to_string()),
        ]);

        let _: Value = self
            .client
            .transport()
            .signed(Method::POST, ORDER, query)
            .await?;

        Ok(())
    }

    async fn try_place_stop_loss_order(
        &self,
        symbol: &Symbol,
        side: Side,
        stop_price: f64,
    ) -> Result<(), BinanceError> {
        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("algoType", "CONDITIONAL".to_string()),
            ("type", "STOP_MARKET".to_string()),
            ("closePosition", "true".to_string()),
            ("triggerPrice", stop_price.to_string()),
            ("workingType", "MARK_PRICE".to_string()),
        ]);

        let _: Value = self
            .client
            .transport()
            .signed(Method::POST, ALGO_ORDER, query)
            .await?;

        Ok(())
    }
}
