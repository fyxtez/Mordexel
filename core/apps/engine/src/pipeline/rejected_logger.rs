use domain::rejected_trade::RejectedTrade;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub async fn run(mut rx: mpsc::Receiver<RejectedTrade>) {
    info!("rejected_trade_logger started");

    while let Some(rejected_trade) = rx.recv().await {
        dbg!(&rejected_trade);
    }

    warn!("rejected_trade_logger stopped: all senders dropped");
}
