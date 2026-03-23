use domain::approved_trade::ApprovedTrade;
use execution::{
    entry::{entry_model::EntryModel, execute_trade},
    exchange::Exchange,
    sizing::{margin::build_execution_plan, types::MarginSizingConfig},
};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub async fn run(
    mut rx: mpsc::Receiver<ApprovedTrade>,
    exchange: impl Exchange,
    entry_model: EntryModel,
    sizing_config: MarginSizingConfig,
) {
    info!("approved_trade_executor started");

    while let Some(approved_trade) = rx.recv().await {
        let symbol = &approved_trade.trade_intent.symbol;

        info!(
            intent_id = %approved_trade.trade_intent.intent_id,
            symbol = %symbol,
            timeframe = %approved_trade.trade_intent.timeframe,
            "received approved trade"
        );

        let plan = match build_execution_plan(&exchange, &approved_trade, &sizing_config).await {
            Ok(plan) => plan,
            Err(err) => {
                error!(
                    error = %err,
                    trade = %approved_trade,
                    "failed to build execution plan"
                );
                continue;
            }
        };

        let leverage = plan.leverage;

        info!(
            trade = %approved_trade,
            quantity = plan.quantity,
            leverage = leverage,
            "built execution plan"
        );

        match exchange.set_leverage(symbol, leverage).await {
            Ok(_) => {
                info!(symbol=%symbol,leverage=%leverage,"Leverage set:")
            }
            Err(error) => {
                error!(error=%error,"Error setting leverage:")
            }
        }

        match execute_trade(&exchange, &approved_trade, &entry_model, plan.quantity).await {
            Ok(_) => {
                info!(trade=%approved_trade,"Executed trade:")
            }
            Err(err) => {
                error!(error=%err,trade=%approved_trade,"Error executing trade:");
            }
        }
    }

    warn!("approved_trade_executor stopped: all senders dropped");
}
