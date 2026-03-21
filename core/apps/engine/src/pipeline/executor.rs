use domain::approved_trade::ApprovedTrade;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub async fn run(mut rx: mpsc::Receiver<ApprovedTrade>) {
    info!("approved_trade_executor started");

    while let Some(approved_trade) = rx.recv().await {
        info!(
            intent_id = %approved_trade.trade_intent.intent_id,
            symbol = %approved_trade.trade_intent.symbol,
            timeframe = %approved_trade.trade_intent.timeframe,
            "received approved trade"
        );
    }

    warn!("approved_trade_executor stopped: all senders dropped");
}
