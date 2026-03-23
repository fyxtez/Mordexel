use execution::{entry::entry_model::EntryModel, sizing::types::MarginSizingConfig};
use tokio::sync::mpsc::Sender;

use domain::ingress_events::IngressEvent;
use tracing::error;

use crate::{
    bootstrap::RuntimeDeps,
    pipeline::{builder, evaluator, executor, rejected_logger},
    utils::start_server,
};

pub async fn run_runtime(runtime: RuntimeDeps) {
    let RuntimeDeps {
        telegram_config,
        execution_policy,
        binance,
        channels,
    } = runtime;

    let ingress_event_tx_clone = channels.ingress_event_tx.clone();

    let telegram_handle = {
        let tx: Sender<IngressEvent> = channels.ingress_event_tx;
        tokio::spawn(async move {
            adapter_telegram::run(telegram_config, tx).await;
        })
    };

    let trade_intent_builder_handle = tokio::spawn(async move {
        builder::run(channels.ingress_event_rx, channels.trade_intent_tx).await;
    });

    let trade_intent_evaluator_handle = tokio::spawn(async move {
        evaluator::run(
            channels.trade_intent_rx,
            channels.approved_trade_tx,
            channels.rejected_trade_tx,
            execution_policy,
        )
        .await;
    });

    let sizing_config = match MarginSizingConfig::new(0.01, 0.90, 100) {
        Ok(config) => config,
        Err(err) => {
            error!(error = %err, "failed to create margin sizing config");
            return;
        }
    };

    let approved_trade_executor_handle = tokio::spawn(async move {
        executor::run(
            channels.approved_trade_rx,
            binance,
            EntryModel::Instant,
            sizing_config,
        )
        .await;
    });

    let rejected_trade_logger_handle = tokio::spawn(async move {
        rejected_logger::run(channels.rejected_trade_rx).await;
    });

    let http_handle = {
        tokio::spawn(async move {
            start_server(ingress_event_tx_clone).await;
        })
    };

    let (
        telegram_result,
        builder_result,
        evaluator_result,
        executor_result,
        rejected_logger_result,
        http_result,
    ) = tokio::join!(
        telegram_handle,
        trade_intent_builder_handle,
        trade_intent_evaluator_handle,
        approved_trade_executor_handle,
        rejected_trade_logger_handle,
        http_handle,
    );

    if let Err(err) = telegram_result {
        tracing::error!(error = %err, "telegram task panicked");
    }
    if let Err(err) = builder_result {
        tracing::error!(error = %err, "builder task panicked");
    }
    if let Err(err) = evaluator_result {
        tracing::error!(error = %err, "evaluator task panicked");
    }
    if let Err(err) = executor_result {
        tracing::error!(error = %err, "executor task panicked");
    }
    if let Err(err) = rejected_logger_result {
        tracing::error!(error = %err, "rejected logger task panicked");
    }
    if let Err(err) = http_result {
        tracing::error!(error = %err, "http task panicked");
    }
}
