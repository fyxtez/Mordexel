use tokio::sync::mpsc::Sender;
use tracing::error;

use adapter_http::start_api_server;
use domain::ingress_events::IngressEvent;

use crate::{
    bootstrap::RuntimeDeps,
    pipeline::{builder, evaluator, executor, rejected_logger},
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
        let tx = channels.ingress_event_tx;
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

    let approved_trade_executor_handle = tokio::spawn(async move {
        executor::run(channels.approved_trade_rx, binance).await;
    });

    let rejected_trade_logger_handle = tokio::spawn(async move {
        rejected_logger::run(channels.rejected_trade_rx).await;
    });

    let http_handle = {
        tokio::spawn(async move {
            start_server(ingress_event_tx_clone).await;
        })
    };

    let _ = tokio::join!(
        telegram_handle,
        trade_intent_builder_handle,
        trade_intent_evaluator_handle,
        approved_trade_executor_handle,
        rejected_trade_logger_handle,
        http_handle,
    );
}

async fn start_server(tx: Sender<IngressEvent>) {
    let address = if cfg!(feature = "production") {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    let port = 8656;

    match start_api_server(address, port, tx).await {
        Ok(_) => {}
        Err(error) => {
            error!(error = %error, "Failed starting api server.");
        }
    }
}
