pub mod entry_model;
mod instant;

use domain::approved_trade::ApprovedTrade;

use crate::{entry::entry_model::EntryModel, error::ExecutionError, exchange::Exchange};

use tracing::{error, info};

pub async fn execute_trade<E: Exchange>(
    exchange: &E,
    approved_trade: &ApprovedTrade,
    entry_model: &EntryModel,
    quantity: f64,
) -> Result<(), ExecutionError> {
    let intent = &approved_trade.trade_intent;

    info!(
        intent_id = %intent.intent_id,
        symbol = %intent.symbol,
        timeframe = %intent.timeframe,
        quantity = quantity,
        ?entry_model,
        "starting trade execution"
    );

    match entry_model {
        EntryModel::Instant => {
            info!(
                intent_id = %intent.intent_id,
                symbol = %intent.symbol,
                quantity = quantity,
                "executing instant trade"
            );

            match instant::execute(exchange, approved_trade, quantity).await {
                Ok(_) => {
                    info!(
                        intent_id = %intent.intent_id,
                        symbol = %intent.symbol,
                        timeframe = %intent.timeframe,
                        execution_quantity = quantity,
                        "instant trade executed successfully"
                    );
                    Ok(())
                }
                Err(err) => {
                    error!(
                        intent_id = %intent.intent_id,
                        symbol = %intent.symbol,
                        quantity = quantity,
                        error = %err,
                        "instant trade execution failed"
                    );
                    Err(err)
                }
            }
        }

        EntryModel::Scheduled(scheduled_entry) => {
            info!(
                intent_id = %intent.intent_id,
                symbol = %intent.symbol,
                quantity = quantity,
                ?scheduled_entry,
                "executing scheduled trade (currently fallback to instant)"
            );

            // TEMP fallback
            match instant::execute(exchange, approved_trade, quantity).await {
                Ok(_) => {
                    info!(
                        intent_id = %intent.intent_id,
                        symbol = %intent.symbol,
                        quantity = quantity,
                        "scheduled trade (fallback) executed successfully"
                    );
                    Ok(())
                }
                Err(err) => {
                    error!(
                        intent_id = %intent.intent_id,
                        symbol = %intent.symbol,
                        quantity = quantity,
                        error = %err,
                        "scheduled trade (fallback) execution failed"
                    );
                    Err(err)
                }
            }

            // Future:
            // scheduled::execute(exchange, approved_trade, scheduled_entry).await
        }
    }
}
