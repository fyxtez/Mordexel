use domain::{approved_trade::ApprovedTrade, symbol::Symbol, timeframe::Timeframe};
use execution::{
    entry::{entry_model::EntryModel, execute_trade},
    exchange::Exchange,
    sizing::{
        margin::build_execution_plan,
        types::{ExecutionPlan, MarginSizingConfig},
    },
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
        let Some(approved_trade) = rx.recv().await else {
            warn!("approved_trade_executor stopped: all senders dropped");
            break;
        };

        process_approved_trade(&exchange, &approved_trade, &entry_model, &sizing_config).await;
    }
}

async fn process_approved_trade(
    exchange: &impl Exchange,
    approved_trade: &ApprovedTrade,
    entry_model: &EntryModel,
    sizing_config: &MarginSizingConfig,
) {
    let symbol = &approved_trade.trade_intent.symbol;
    let intent_id = approved_trade.trade_intent.intent_id;
    let timeframe = approved_trade.trade_intent.timeframe;

    log_received_trade(approved_trade);

    let Some(plan) = build_plan_or_log(exchange, approved_trade, sizing_config).await else {
        return;
    };

    if !set_leverage_or_log(exchange, symbol, intent_id, plan.leverage).await {
        return;
    }

    execute_approved_trade(
        exchange,
        approved_trade,
        entry_model,
        plan.quantity,
        timeframe,
    )
    .await;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        "finished processing approved trade"
    );
}

fn log_received_trade(approved_trade: &ApprovedTrade) {
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
}

async fn build_plan_or_log(
    exchange: &impl Exchange,
    approved_trade: &ApprovedTrade,
    sizing_config: &MarginSizingConfig,
) -> Option<ExecutionPlan> {
    let symbol = &approved_trade.trade_intent.symbol;
    let intent_id = approved_trade.trade_intent.intent_id;
    let timeframe = approved_trade.trade_intent.timeframe;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        "building execution plan"
    );

    match build_execution_plan(exchange, approved_trade, sizing_config).await {
        Ok(plan) => {
            info!(
                intent_id = %intent_id,
                symbol = %symbol,
                timeframe = %timeframe,
                planned_quantity = plan.quantity,
                leverage = plan.leverage,
                "built execution plan"
            );
            Some(plan)
        }
        Err(err) => {
            error!(
                intent_id = %intent_id,
                symbol = %symbol,
                error = %err,
                trade = %approved_trade,
                "failed to build execution plan"
            );
            None
        }
    }
}

async fn set_leverage_or_log(
    exchange: &impl Exchange,
    symbol: &Symbol,
    intent_id: uuid::Uuid,
    leverage: u32,
) -> bool {
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
            true
        }
        Err(err) => {
            error!(
                intent_id = %intent_id,
                symbol = %symbol,
                leverage = leverage,
                error = %err,
                "failed to set leverage"
            );
            false
        }
    }
}

async fn execute_approved_trade(
    exchange: &impl Exchange,
    approved_trade: &ApprovedTrade,
    entry_model: &EntryModel,
    quantity: f64,
    timeframe: Timeframe,
) {
    let symbol = &approved_trade.trade_intent.symbol;
    let intent_id = approved_trade.trade_intent.intent_id;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        quantity = quantity,
        "executing trade"
    );

    match execute_trade(exchange, approved_trade, entry_model, quantity).await {
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
}
