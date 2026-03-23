use domain::approved_trade::ApprovedTrade;

use crate::{
    constants::symbol_filters,
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
    let account_info = exchange.account_info().await?;

    let portfolio_equity = account_info.balance;

    let symbol = &approved_trade.trade_intent.symbol;

    let filters = symbol_filters()
        .get(symbol)
        .ok_or_else(|| ExecutionError::Internal {
            message: format!("missing symbol filters for {}", symbol),
        })?;

    let entry = approved_trade.trade_intent.entry;

    if entry <= 0.0 {
        return Err(ExecutionError::Internal {
            message: format!("entry must be > 0 for {}", symbol),
        });
    }

    let stop_loss = approved_trade.trade_intent.stop_loss;

    if stop_loss <= 0.0 {
        return Err(ExecutionError::Internal {
            message: format!("stop_loss must be > 0 for {}", symbol),
        });
    }

    if portfolio_equity <= 0.0 {
        return Err(ExecutionError::Internal {
            message: "portfolio_equity must be > 0".into(),
        });
    }

    let stop_distance_pct = ((entry - stop_loss).abs()) / entry;

    if stop_distance_pct <= 0.0 {
        return Err(ExecutionError::Internal {
            message: format!("stop distance must be > 0 for {}", symbol),
        });
    }

    let allocated_margin = portfolio_equity * config.margin_pct;

    if allocated_margin <= 0.0 {
        return Err(ExecutionError::Internal {
            message: "allocated_margin must be > 0".into(),
        });
    }

    let theoretical_max_leverage = 1.0 / stop_distance_pct;

    if !(0.0 < config.leverage_safety && config.leverage_safety <= 1.0) {
        return Err(ExecutionError::Internal {
            message: "leverage_safety must be in (0, 1]".into(),
        });
    }

    let leverage = (theoretical_max_leverage * config.leverage_safety)
        .floor()
        .max(1.0)
        .min(config.max_leverage as f64) as u32;
    let position_notional = allocated_margin * leverage as f64;
    let raw_quantity = position_notional / entry;
    let mut quantity = round_down_to_step(raw_quantity, filters.step_size);
    let min_qty = round_up_to_step(filters.min_qty, filters.step_size);

    if quantity < min_qty {
        quantity = min_qty;
    }

    let min_notional_target = filters.min_notional * 1.10;
    let notional = quantity * entry;

    if notional < min_notional_target {
        quantity = round_up_to_step(min_notional_target / entry, filters.step_size);
    }

    let final_notional = quantity * entry;

    if quantity <= 0.0 {
        return Err(ExecutionError::Internal {
            message: format!("computed quantity must be > 0 for {}", symbol),
        });
    }

    if quantity < filters.min_qty {
        return Err(ExecutionError::Internal {
            message: format!(
                "computed quantity {} below min_qty {} for {}",
                quantity, filters.min_qty, symbol
            ),
        });
    }

    if final_notional < filters.min_notional {
        return Err(ExecutionError::Internal {
            message: format!(
                "computed notional {} below min_notional {} for {}",
                final_notional, filters.min_notional, symbol
            ),
        });
    }

    Ok(ExecutionPlan { quantity, leverage })
}
