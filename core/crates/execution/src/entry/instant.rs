use domain::{approved_trade::ApprovedTrade, side::Side, timeframe::Timeframe};

use crate::{
    error::ExecutionError, exchange::Exchange, tp_strategy::{TpStrategy, resolve_tp_targets}, utils::{round_down_to_step, round_up_to_step}
};

use tracing::{error, info, warn};

pub async fn execute<E: Exchange>(
    exchange: &E,
    approved_trade: &ApprovedTrade,
    quantity: f64,
) -> Result<(), ExecutionError> {
    let intent = &approved_trade.trade_intent;
    let symbol = &intent.symbol;
    let close_side = intent.side.opposite();

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        side = %intent.side,
        close_side = %close_side,
        requested_quantity = quantity,
        stop_loss = intent.stop_loss,
        target_count = intent.targets.len(),
        "starting instant execution"
    );

    let filters = exchange.symbol_filters(symbol)?;

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        min_qty = filters.min_qty,
        step_size = filters.step_size,
        min_notional = filters.min_notional,
        "loaded symbol filters"
    );

    let quantity = round_down_to_step(quantity, filters.step_size);

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        rounded_quantity = quantity,
        step_size = filters.step_size,
        "rounded execution quantity to step size"
    );

    if quantity <= 0.0 {
        error!(
            intent_id = %intent.intent_id,
            symbol = %symbol,
            quantity = quantity,
            "quantity rounded to zero"
        );
        return Err(ExecutionError::Internal {
            message: format!("quantity rounded to zero for {}", symbol),
        });
    }

    if intent.targets.is_empty() {
        error!(
            intent_id = %intent.intent_id,
            symbol = %symbol,
            "no take profit targets provided"
        );
        return Err(ExecutionError::Internal {
            message: format!("no take profit targets for {}", symbol),
        });
    }

    let max_tp_orders = (quantity / filters.step_size).floor() as usize;

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        quantity = quantity,
        max_tp_orders = max_tp_orders,
        "computed max possible take-profit orders"
    );

    if max_tp_orders == 0 {
        error!(
            intent_id = %intent.intent_id,
            symbol = %symbol,
            quantity = quantity,
            step_size = filters.step_size,
            "quantity too small to place take profits"
        );
        return Err(ExecutionError::Internal {
            message: format!("quantity too small to place take profits for {}", symbol),
        });
    }

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        side = %intent.side,
        quantity = quantity,
        "placing market entry order"
    );

    match exchange
        .place_market_order(symbol, intent.side, quantity)
        .await
    {
        Ok(_) => {
            info!(
                intent_id = %intent.intent_id,
                symbol = %symbol,
                side = %intent.side,
                quantity = quantity,
                "market entry order placed successfully"
            );
        }
        Err(err) => {
            error!(
                intent_id = %intent.intent_id,
                symbol = %symbol,
                side = %intent.side,
                quantity = quantity,
                error = %err,
                "failed to place market entry order"
            );
            return Err(err.into());
        }
    }

    let sl_price = match intent.side {
        Side::Long => round_down_to_step(intent.stop_loss, filters.tick_size),
        Side::Short => round_up_to_step(intent.stop_loss, filters.tick_size),
    };

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        close_side = %close_side,
        stop_loss_input = intent.stop_loss,
        stop_loss_price = sl_price,
        tick_size = filters.tick_size,
        "computed stop-loss price"
    );

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        close_side = %close_side,
        stop_loss_price = sl_price,
        "placing stop-loss order"
    );

    match exchange
        .place_stop_loss_order(symbol, close_side, sl_price)
        .await
    {
        Ok(_) => {
            info!(
                intent_id = %intent.intent_id,
                symbol = %symbol,
                close_side = %close_side,
                stop_loss_price = sl_price,
                "stop-loss order placed successfully"
            );
        }
        Err(err) => {
            error!(
                intent_id = %intent.intent_id,
                symbol = %symbol,
                close_side = %close_side,
                stop_loss_price = sl_price,
                error = %err,
                "failed to place stop-loss order"
            );
            return Err(err.into());
        }
    }

    let timeframe = intent.timeframe;
    let tp_strategy = TpStrategy::Tp2Adjusted;

    let effective_targets = resolve_tp_targets(
        tp_strategy,
        &intent.targets,
        intent.timeframe,
    );

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        timeframe = %timeframe,
        mode = if timeframe == Timeframe::H1 { "TP1_ONLY" } else { "FULL" },
        "execution mode selected"
    );

    let tp_count = effective_targets.len().min(max_tp_orders);

    if tp_count == 0 {
        return Err(ExecutionError::Internal {
            message: format!("no usable take profit targets for {}", symbol),
        });
    }

    let targets = &effective_targets[..tp_count];
    let tp_base = round_down_to_step(quantity / tp_count as f64, filters.step_size);

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        requested_targets = intent.targets.len(),
        effective_targets = ?targets,
        tp_count = tp_count,
        tp_base_quantity = tp_base,
        "computed take-profit distribution"
    );

    for (i, target) in targets.iter().enumerate() {
        let effective_tp_index = i + 1;

        let tp_qty = if i == tp_count - 1 {
            round_down_to_step(
                quantity - tp_base * (tp_count - 1) as f64,
                filters.step_size,
            )
        } else {
            tp_base
        };

        if tp_qty <= 0.0 {
            warn!(
                intent_id = %intent.intent_id,
                symbol = %symbol,
                tp_index = effective_tp_index,
                target = *target,
                tp_qty = tp_qty,
                "skipping take-profit order because quantity is zero after rounding"
            );
            continue;
        }

        let tp_price = match intent.side {
            Side::Long => round_down_to_step(*target, filters.tick_size),
            Side::Short => round_up_to_step(*target, filters.tick_size),
        };

        info!(
            intent_id = %intent.intent_id,
            symbol = %symbol,
            tp_index = effective_tp_index,
            close_side = %close_side,
            target_input = *target,
            tp_price = tp_price,
            tp_qty = tp_qty,
            "placing take-profit order"
        );

        match exchange
            .place_take_profit_order(symbol, close_side, tp_qty, tp_price)
            .await
        {
            Ok(_) => {
                info!(
                    intent_id = %intent.intent_id,
                    symbol = %symbol,
                    tp_index = effective_tp_index,
                    tp_price = tp_price,
                    tp_qty = tp_qty,
                    "take-profit order placed successfully"
                );
            }
            Err(err) => {
                error!(
                    intent_id = %intent.intent_id,
                    symbol = %symbol,
                    tp_index = effective_tp_index,
                    tp_price = tp_price,
                    tp_qty = tp_qty,
                    error = %err,
                    "failed to place take-profit order"
                );
                return Err(err.into());
            }
        }
    }

    info!(
        intent_id = %intent.intent_id,
        symbol = %symbol,
        quantity = quantity,
        tp_count = tp_count,
        "instant execution completed successfully"
    );

    Ok(())
}
