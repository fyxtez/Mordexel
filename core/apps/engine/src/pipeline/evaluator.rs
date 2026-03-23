use domain::{
    approved_trade::ApprovedTrade,
    rejected_trade::{RejectedTrade, RejectionReason},
    trade_intent::TradeIntent,
};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::execution_policy::ExecutionPolicy;

pub async fn run(
    mut rx: mpsc::Receiver<TradeIntent>,
    approved_trade_tx: mpsc::Sender<ApprovedTrade>,
    rejected_trade_tx: mpsc::Sender<RejectedTrade>,
    policy: ExecutionPolicy,
) {
    info!("trade_intent_evaluator started");

    while let Some(trade_intent) = rx.recv().await {
        if policy.is_allowed(trade_intent.timeframe, &trade_intent.symbol) {
            let approved_trade = ApprovedTrade { trade_intent };

            info!(
                intent_id = %approved_trade.trade_intent.intent_id,
                symbol = %approved_trade.trade_intent.symbol,
                timeframe = %approved_trade.trade_intent.timeframe,
                "trade intent approved"
            );

            if let Err(err) = approved_trade_tx.send(approved_trade).await {
                error!(error = %err, "failed to send approved trade");
                break;
            }
        } else {
            let rejected_trade = RejectedTrade {
                trade_intent,
                reason: RejectionReason::ExecutionPolicyDenied,
            };

            if let Err(err) = rejected_trade_tx.send(rejected_trade).await {
                error!(error = %err, "failed to send rejected trade");
                break;
            }
        }
    }

    warn!("trade_intent_evaluator stopped: all senders dropped");
}
