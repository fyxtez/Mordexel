use domain::approved_trade::ApprovedTrade;
use execution::exchange::Exchange;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub async fn run(mut rx: mpsc::Receiver<ApprovedTrade>, exchange: impl Exchange) {
    info!("approved_trade_executor started");

    while let Some(approved_trade) = rx.recv().await {
        info!(
            intent_id = %approved_trade.trade_intent.intent_id,
            symbol = %approved_trade.trade_intent.symbol,
            timeframe = %approved_trade.trade_intent.timeframe,
            "received approved trade"
        );

        // TODO: Await in hot path? hmmmm... think of spawning a thread for this.
        match exchange.account_info().await {
            Ok(account_info) => {
                dbg!(account_info);
            }
            Err(error) => {
                dbg!(error);
            }
        }
    }

    warn!("approved_trade_executor stopped: all senders dropped");
}
