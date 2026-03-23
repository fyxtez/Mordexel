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
    info!(
        ?entry_model,
        ?sizing_config,
        "approved_trade_executor started"
    );

    loop {
        info!("waiting for approved trade");

        let Some(approved_trade) = rx.recv().await else {
            warn!("approved_trade_executor stopped: all senders dropped");
            break;
        };

        let symbol = &approved_trade.trade_intent.symbol;
        let intent_id = approved_trade.trade_intent.intent_id;
        let timeframe = approved_trade.trade_intent.timeframe;

        info!(
            intent_id = %intent_id,
            symbol = %symbol,
            timeframe = %timeframe,
            trade = %approved_trade,
            "received approved trade"
        );

        info!(
            intent_id = %intent_id,
            symbol = %symbol,
            "building execution plan"
        );

        let plan = match build_execution_plan(&exchange, &approved_trade, &sizing_config).await {
            Ok(plan) => {
                info!(
                    intent_id = %approved_trade.trade_intent.intent_id,
                    symbol = %approved_trade.trade_intent.symbol,
                    timeframe = %approved_trade.trade_intent.timeframe,
                    planned_quantity = plan.quantity,
                    leverage = plan.leverage,
                    "built execution plan"
                );
                plan
            }
            Err(err) => {
                error!(
                    intent_id = %intent_id,
                    symbol = %symbol,
                    error = %err,
                    trade = %approved_trade,
                    "failed to build execution plan"
                );
                continue;
            }
        };

        let leverage = plan.leverage;
        let quantity = plan.quantity;

        info!(
            intent_id = %intent_id,
            symbol = %symbol,
            leverage = leverage,
            "setting leverage"
        );

        match exchange.set_leverage(symbol, leverage).await {
            Ok(_) => {
                info!(
                    intent_id = %intent_id,
                    symbol = %symbol,
                    leverage = leverage,
                    "leverage set successfully"
                );
            }
            Err(err) => {
                error!(
                    intent_id = %intent_id,
                    symbol = %symbol,
                    leverage = leverage,
                    error = %err,
                    "failed to set leverage"
                );
                continue;
            }
        }

        info!(
            intent_id = %intent_id,
            symbol = %symbol,
            quantity = quantity,
            "executing trade"
        );

        match execute_trade(&exchange, &approved_trade, &entry_model, quantity).await {
            Ok(_) => {
                info!(
                    intent_id = %intent_id,
                    symbol = %symbol,
                    timeframe = %timeframe,
                    planned_quantity = quantity,
                    "trade executed successfully"
                );
            }
            Err(err) => {
                error!(
                    intent_id = %intent_id,
                    symbol = %symbol,
                    quantity = quantity,
                    error = %err,
                    trade = %approved_trade,
                    "failed to execute trade"
                );
            }
        }

        info!(
            intent_id = %intent_id,
            symbol = %symbol,
            "finished processing approved trade"
        );
    }
}
