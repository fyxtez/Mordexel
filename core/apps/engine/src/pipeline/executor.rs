use domain::approved_trade::ApprovedTrade;
use execution::{
    entry::{entry_model::EntryModel, execute_trade},
    exchange::Exchange,
};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub async fn run(
    mut rx: mpsc::Receiver<ApprovedTrade>,
    exchange: impl Exchange,
    entry_model: EntryModel,
) {
    info!("approved_trade_executor started");

    while let Some(approved_trade) = rx.recv().await {
        info!(
            intent_id = %approved_trade.trade_intent.intent_id,
            symbol = %approved_trade.trade_intent.symbol,
            timeframe = %approved_trade.trade_intent.timeframe,
            "received approved trade"
        );

        // TODO: Await in hot path? hmmmm... think of spawning a thread for this.
        match execute_trade(&exchange, &approved_trade, &entry_model).await {
            Ok(_) => {}
            Err(err) => {
                error!(error=%err,trade=%approved_trade,"Error executing trade:");
            }
        }
    }

    warn!("approved_trade_executor stopped: all senders dropped");
}
