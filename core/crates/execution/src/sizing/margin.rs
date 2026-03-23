use domain::approved_trade::ApprovedTrade;
use tracing::{error, info};

use crate::{
    error::ExecutionError,
    exchange::Exchange,
    sizing::types::{ExecutionPlan, MarginSizingConfig},
    utils::{round_down_to_step, round_up_to_step},
};

pub async fn build_execution_plan<E: Exchange>(
    exchange: &E,
    approved_trade: &ApprovedTrade,
    config: &MarginSizingConfig,
) -> Result<ExecutionPlan, ExecutionError> {
    let symbol = &approved_trade.trade_intent.symbol;
    let intent_id = approved_trade.trade_intent.intent_id;
    let entry = approved_trade.trade_intent.entry;
    let stop_loss = approved_trade.trade_intent.stop_loss;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        entry = entry,
        stop_loss = stop_loss,
        margin_pct = config.margin_pct,
        leverage_safety = config.leverage_safety,
        max_leverage = config.max_leverage,
        "building execution plan"
    );

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        "fetching account info"
    );

    let account_info = match exchange.account_info().await {
        Ok(account_info) => {
            info!(
                intent_id = %intent_id,
                symbol = %symbol,
                balance = account_info.balance,
                "fetched account info"
            );
            account_info
        }
        Err(err) => {
            error!(
                intent_id = %intent_id,
                symbol = %symbol,
                error = %err,
                "failed to fetch account info"
            );
            return Err(err.into());
        }
    };

    let portfolio_equity = account_info.balance;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        "loading symbol filters"
    );

    let filters = exchange.symbol_filters(symbol)?;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        min_qty = filters.min_qty,
        step_size = filters.step_size,
        min_notional = filters.min_notional,
        "loaded symbol filters"
    );

    if entry <= 0.0 {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            entry = entry,
            "invalid entry"
        );
        return Err(ExecutionError::Internal {
            message: format!("entry must be > 0 for {}", symbol),
        });
    }

    if stop_loss <= 0.0 {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            stop_loss = stop_loss,
            "invalid stop_loss"
        );
        return Err(ExecutionError::Internal {
            message: format!("stop_loss must be > 0 for {}", symbol),
        });
    }

    if portfolio_equity <= 0.0 {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            portfolio_equity = portfolio_equity,
            "invalid portfolio equity"
        );
        return Err(ExecutionError::Internal {
            message: "portfolio_equity must be > 0".into(),
        });
    }

    if !(0.0 < config.leverage_safety && config.leverage_safety <= 1.0) {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            leverage_safety = config.leverage_safety,
            "invalid leverage safety"
        );
        return Err(ExecutionError::Internal {
            message: "leverage_safety must be in (0, 1]".into(),
        });
    }

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        "validated execution plan inputs"
    );

    let stop_distance_pct = ((entry - stop_loss).abs()) / entry;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        stop_distance_pct = stop_distance_pct,
        "computed stop distance percent"
    );

    if stop_distance_pct <= 0.0 {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            stop_distance_pct = stop_distance_pct,
            "invalid stop distance percent"
        );
        return Err(ExecutionError::Internal {
            message: format!("stop distance must be > 0 for {}", symbol),
        });
    }

    let allocated_margin = portfolio_equity * config.margin_pct;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        portfolio_equity = portfolio_equity,
        margin_pct = config.margin_pct,
        allocated_margin = allocated_margin,
        "computed allocated margin"
    );

    if allocated_margin <= 0.0 {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            allocated_margin = allocated_margin,
            "invalid allocated margin"
        );
        return Err(ExecutionError::Internal {
            message: "allocated_margin must be > 0".into(),
        });
    }

    let theoretical_max_leverage = 1.0 / stop_distance_pct;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        theoretical_max_leverage = theoretical_max_leverage,
        "computed theoretical max leverage"
    );

    let leverage = (theoretical_max_leverage * config.leverage_safety)
        .floor()
        .max(1.0)
        .min(config.max_leverage as f64) as u32;

    let position_notional = allocated_margin * leverage as f64;
    let raw_quantity = position_notional / entry;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        leverage = leverage,
        position_notional = position_notional,
        raw_quantity = raw_quantity,
        "computed leverage and raw quantity"
    );

    let mut quantity = round_down_to_step(raw_quantity, filters.step_size);
    let min_qty = round_up_to_step(filters.min_qty, filters.step_size);

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        rounded_quantity = quantity,
        min_qty = min_qty,
        step_size = filters.step_size,
        "rounded quantity to step size"
    );

    if quantity < min_qty {
        info!(
            intent_id = %intent_id,
            symbol = %symbol,
            previous_quantity = quantity,
            adjusted_quantity = min_qty,
            "quantity below min_qty, adjusting upward"
        );
        quantity = min_qty;
    }

    let min_notional_target = filters.min_notional * 1.10;
    let notional = quantity * entry;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        quantity = quantity,
        notional = notional,
        min_notional = filters.min_notional,
        min_notional_target = min_notional_target,
        "computed notional after min_qty adjustment"
    );

    if notional < min_notional_target {
        let adjusted_quantity = round_up_to_step(min_notional_target / entry, filters.step_size);

        info!(
            intent_id = %intent_id,
            symbol = %symbol,
            previous_quantity = quantity,
            adjusted_quantity = adjusted_quantity,
            "notional below target, adjusting quantity upward"
        );

        quantity = adjusted_quantity;
    }

    let final_notional = quantity * entry;

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        final_quantity = quantity,
        final_notional = final_notional,
        leverage = leverage,
        "computed final execution values"
    );

    if quantity <= 0.0 {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            quantity = quantity,
            "computed quantity is invalid"
        );
        return Err(ExecutionError::Internal {
            message: format!("computed quantity must be > 0 for {}", symbol),
        });
    }

    if quantity < filters.min_qty {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            quantity = quantity,
            min_qty = filters.min_qty,
            "computed quantity below exchange min_qty"
        );
        return Err(ExecutionError::Internal {
            message: format!(
                "computed quantity {} below min_qty {} for {}",
                quantity, filters.min_qty, symbol
            ),
        });
    }

    if final_notional < filters.min_notional {
        error!(
            intent_id = %intent_id,
            symbol = %symbol,
            final_notional = final_notional,
            min_notional = filters.min_notional,
            "computed notional below exchange min_notional"
        );
        return Err(ExecutionError::Internal {
            message: format!(
                "computed notional {} below min_notional {} for {}",
                final_notional, filters.min_notional, symbol
            ),
        });
    }

    info!(
        intent_id = %intent_id,
        symbol = %symbol,
        quantity = quantity,
        leverage = leverage,
        final_notional = final_notional,
        "execution plan built successfully"
    );

    Ok(ExecutionPlan { quantity, leverage })
}
