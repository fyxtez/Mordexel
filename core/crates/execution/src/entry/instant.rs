use domain::approved_trade::ApprovedTrade;

use crate::{error::ExecutionError, exchange::Exchange};

pub async fn execute<E: Exchange>(
    exchange: &E,
    approved_trade: &ApprovedTrade,
    quantity: f64,
) -> Result<(), ExecutionError> {
    let intent = &approved_trade.trade_intent;
    let symbol = &intent.symbol;

    exchange
        .place_market_order(symbol, intent.side, quantity)
        .await?;

    Ok(())
}
