pub mod entry_model;
mod instant;

use domain::approved_trade::ApprovedTrade;

use crate::{entry::entry_model::EntryModel, error::ExecutionError, exchange::Exchange};

pub async fn execute_trade<E: Exchange>(
    exchange: &E,
    approved_trade: &ApprovedTrade,
    entry_model: &EntryModel,
    quantity: f64,
) -> Result<(), ExecutionError> {
    match entry_model {
        EntryModel::Instant => instant::execute(exchange, approved_trade, quantity).await,
        EntryModel::Scheduled(_scheduled_entry) => {
            instant::execute(exchange, approved_trade, quantity).await
            // TODO
            // scheduled::execute(exchange, approved_trade, scheduled_entry).await
        }
    }
}
