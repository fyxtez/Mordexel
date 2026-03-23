pub mod client;
mod constants;
mod endpoints;
mod error;
mod futures_endpoints;
mod response_types;
mod transport;
pub mod types;
pub mod utils;

use async_trait::async_trait;
use domain::{side::Side, symbol::Symbol};
use execution::{
    error::ExchangeError,
    exchange::Exchange,
    types::{AccountInfo, SetLeverageResponse as ExchangeSetLeverageResponse, SymbolFilters},
};
use reqwest::Method;
use serde_json::Value;
use tracing::{debug, info};

use crate::{
    client::BinanceClient,
    constants::binance_symbol_filters,
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

    fn symbol_filters(&self, symbol: &Symbol) -> Result<&SymbolFilters, ExchangeError> {
        binance_symbol_filters()
            .get(symbol)
            .ok_or_else(|| ExchangeError::InvalidRequest {
                message: format!("no symbol filters configured for {}", symbol),
            })
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
    async fn place_take_profit_order(
        &self,
        symbol: &Symbol,
        side: Side,
        quantity: f64,
        trigger_price: f64,
    ) -> Result<(), ExchangeError> {
        self.try_place_take_profit_order(symbol, side, quantity, trigger_price)
            .await?;
        Ok(())
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
        debug!(
            symbol = %symbol,
            leverage = leverage,
            "setting leverage on binance"
        );

        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("leverage", leverage.to_string()),
        ]);

        let response: BinanceSetLeverageResponse = self
            .client
            .transport()
            .signed(Method::POST, LEVERAGE, query)
            .await?;

        info!(
            symbol = %symbol,
            requested_leverage = leverage,
            applied_leverage = response.leverage,
            "binance leverage set successfully"
        );

        Ok(response)
    }

    async fn try_account_info(&self) -> Result<FuturesAccountInfo, BinanceError> {
        debug!("fetching futures account info from binance");

        let response: FuturesAccountInfo = self
            .client
            .transport()
            .signed(Method::GET, ACCOUNT_INFO, String::new())
            .await?;

        debug!(
            total_wallet_balance = response.total_wallet_balance,
            "fetched futures account info successfully"
        );

        Ok(response)
    }

    async fn try_place_take_profit_order(
        &self,
        symbol: &Symbol,
        side: Side,
        quantity: f64,
        price: f64,
    ) -> Result<(), BinanceError> {
        debug!(
            symbol = %symbol,
            side = %side,
            quantity = quantity,
            price = price,
            order_type = "LIMIT",
            reduce_only = true,
            "placing take profit order on binance"
        );

        let query = build_query(&[
            ("symbol", symbol.to_string()),
            ("side", side.to_string()),
            ("type", "LIMIT".to_string()),
            ("quantity", quantity.to_string()),
            ("price", price.to_string()),
            ("timeInForce", "GTC".to_string()),
            ("reduceOnly", "true".to_string()),
            ("newOrderRespType", "RESULT".to_string()),
        ]);

        let _: Value = self
            .client
            .transport()
            .signed(Method::POST, ORDER, query)
            .await?;

        info!(
            symbol = %symbol,
            side = %side,
            quantity = quantity,
            price = price,
            order_type = "LIMIT",
            reduce_only = true,
            "take profit order placed successfully"
        );

        Ok(())
    }

    async fn try_place_market_order(
        &self,
        symbol: &Symbol,
        side: Side,
        quantity: f64,
    ) -> Result<(), BinanceError> {
        debug!(
            symbol = %symbol,
            side = %side,
            quantity = quantity,
            order_type = "MARKET",
            "placing market order on binance"
        );

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

        info!(
            symbol = %symbol,
            side = %side,
            quantity = quantity,
            order_type = "MARKET",
            "market order placed successfully"
        );

        Ok(())
    }

    async fn try_place_stop_loss_order(
        &self,
        symbol: &Symbol,
        side: Side,
        stop_price: f64,
    ) -> Result<(), BinanceError> {
        debug!(
            symbol = %symbol,
            side = %side,
            stop_price = stop_price,
            order_type = "STOP_MARKET",
            close_position = true,
            working_type = "MARK_PRICE",
            "placing stop loss order on binance"
        );

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

        info!(
            symbol = %symbol,
            side = %side,
            stop_price = stop_price,
            order_type = "STOP_MARKET",
            close_position = true,
            working_type = "MARK_PRICE",
            "stop loss order placed successfully"
        );

        Ok(())
    }
}
