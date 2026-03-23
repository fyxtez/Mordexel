use domain::rejected_trade::RejectedTrade;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub async fn run(mut rx: mpsc::Receiver<RejectedTrade>) {
    info!("rejected_trade_logger started");

    while let Some(rejected_trade) = rx.recv().await {
            warn!(
                intent_id = %rejected_trade.trade_intent.intent_id,
                symbol = %rejected_trade.trade_intent.symbol,
                timeframe = %rejected_trade.trade_intent.timeframe,
                reason = ?rejected_trade.reason,
                "trade intent rejected"
            );
    }

    warn!("rejected_trade_logger stopped: all senders dropped");
}
