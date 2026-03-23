use std::collections::HashMap;
use std::sync::OnceLock;

use domain::symbol::{Asset, Symbol};

use crate::types::SymbolFilters;

pub static SYMBOL_FILTERS: OnceLock<HashMap<Symbol, SymbolFilters>> = OnceLock::new();

pub fn symbol_filters() -> &'static HashMap<Symbol, SymbolFilters> {
    SYMBOL_FILTERS.get_or_init(|| {
        let mut map = HashMap::new();

        map.insert(
            Symbol::new(Asset::BTC, Asset::USDT),
            SymbolFilters {
                step_size: 0.001,
                min_qty: 0.001,
                min_notional: 100.0,
                tick_size: 0.1,
            },
        );

        map.insert(
            Symbol::new(Asset::ETH, Asset::USDT),
            SymbolFilters {
                step_size: 0.001,
                min_qty: 0.001,
                min_notional: 20.0,
                tick_size: 0.01,
            },
        );

        map.insert(
            Symbol::new(Asset::SOL, Asset::USDT),
            SymbolFilters {
                step_size: 0.01,
                min_qty: 0.01,
                min_notional: 5.0,
                tick_size: 0.01,
            },
        );

        map.insert(
            Symbol::new(Asset::BNB, Asset::USDT),
            SymbolFilters {
                step_size: 0.01,
                min_qty: 0.01,
                min_notional: 5.0,
                tick_size: 0.01,
            },
        );

        map.insert(
            Symbol::new(Asset::XRP, Asset::USDT),
            SymbolFilters {
                step_size: 0.1,
                min_qty: 0.1,
                min_notional: 5.0,
                tick_size: 0.0001,
            },
        );

        map.insert(
            Symbol::new(Asset::TRX, Asset::USDT),
            SymbolFilters {
                step_size: 1.0,
                min_qty: 1.0,
                min_notional: 5.0,
                tick_size: 0.00001,
            },
        );

        map.insert(
            Symbol::new(Asset::ADA, Asset::USDT),
            SymbolFilters {
                step_size: 1.0,
                min_qty: 1.0,
                min_notional: 5.0,
                tick_size: 0.0001,
            },
        );
        map
    })
}
