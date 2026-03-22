use domain::approved_trade::ApprovedTrade;

use crate::{constants::symbol_filters, error::ExecutionError, exchange::Exchange, utils::round_up_to_step};

pub async fn execute<E: Exchange>(
    exchange: &E,
    approved_trade: &ApprovedTrade,
) -> Result<(), ExecutionError> {
    let intent = &approved_trade.trade_intent;
    let symbol = &intent.symbol;

    let filters = symbol_filters()
        .get(symbol)
        .expect("missing symbol filters");

    let quantity = {
        let target_notional = filters.min_notional * 1.10;
        let qty_from_notional = target_notional / intent.entry;
        let raw_qty = qty_from_notional.max(filters.min_qty);
        round_up_to_step(raw_qty, filters.step_size)
    };

    exchange
        .place_market_order(symbol, intent.side, quantity)
        .await?;

    Ok(())
}