use async_trait::async_trait;
use domain::{side::Side, symbol::Symbol};

use crate::{
    error::ExchangeError,
    types::{AccountInfo, SetLeverageResponse, SymbolFilters},
};

// TODO: Result<()
// Must return valid type instead of ()
#[async_trait]
pub trait Exchange: Send + Sync {
    async fn place_market_order(
        &self,
        symbol: &Symbol,
        side: Side,
        quantity: f64,
    ) -> Result<(), ExchangeError>;

    async fn account_info(&self) -> Result<AccountInfo, ExchangeError>;

    async fn set_leverage(
        &self,
        symbol: &Symbol,
        leverage: u32,
    ) -> Result<SetLeverageResponse, ExchangeError>;

    async fn place_stop_loss_order(
        &self,
        symbol: &Symbol,
        side: Side, // opposite of position side
        stop_price: f64,
    ) -> Result<(), ExchangeError>;

    async fn place_take_profit_order(
        &self,
        symbol: &Symbol,
        side: Side,
        quantity: f64,
        trigger_price: f64,
    ) -> Result<(), ExchangeError>;

    fn symbol_filters(&self, symbol: &Symbol) -> Result<&SymbolFilters, ExchangeError>;
}
