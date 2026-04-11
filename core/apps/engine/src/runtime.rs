use execution::{entry::entry_model::EntryModel, sizing::types::MarginSizingConfig};
use tokio::task::JoinHandle;

use tracing::error;

use crate::{
    pipeline::{builder, evaluator, executor, policy_logger, rejected_logger},
    types::{RuntimeChannels, RuntimeDeps},
    utils::{load_ingress_secret, start_server},
};

struct RuntimeHandles {
    websocket: JoinHandle<()>,
    builder: JoinHandle<()>,
    evaluator: JoinHandle<()>,
    executor: JoinHandle<()>,
    rejected_logger: JoinHandle<()>,
    policy_logger: JoinHandle<()>,
    http: JoinHandle<()>,
    ws_consumer: JoinHandle<()>,
}

pub async fn run_runtime(runtime: RuntimeDeps) {
    let handles = spawn_runtime_tasks(runtime);
    wait_for_runtime_tasks(handles).await;
}

fn spawn_runtime_tasks(runtime: RuntimeDeps) -> RuntimeHandles {
    let RuntimeDeps {
        execution_policy,
        binance,
        channels,
    } = runtime;

    let RuntimeChannels {
        ingress_event_tx,
        ingress_event_rx,
        trade_intent_tx,
        trade_intent_rx,
        approved_trade_tx,
        approved_trade_rx,
        rejected_trade_tx,
        rejected_trade_rx,
        ws_event_tx,
        mut ws_event_rx,
    } = channels;

    let is_test = binance.client.is_test;

    let ingress_event_tx_clone = ingress_event_tx.clone();

    let sizing_config = create_sizing_config();

    let rest_base = binance.client.base_url.clone();
    let api_key = binance.client.api_key.clone();
    let ws_base = if is_test {
        "wss://fstream.binancefuture.com".to_string()
    } else {
        "wss://fstream.binance.com".to_string()
    };

    let websocket = tokio::spawn(async move {
        if let Err(err) = binance::ws::run::run(&rest_base, &api_key, &ws_base, ws_event_tx).await {
            tracing::error!(error = %err, "binance websocket runtime stopped");
        }
    });

    let ws_consumer = tokio::spawn(async move {
        while let Some(event) = ws_event_rx.recv().await {
            tracing::info!("**************************");
            tracing::info!(?event, "received ws event");
            tracing::info!("**************************");
        }

        tracing::warn!("ws event consumer stopped");
    });

    let builder = tokio::spawn(async move {
        builder::run(ingress_event_rx, trade_intent_tx).await;
    });

    let policy_for_logger = execution_policy.clone();

    let evaluator = tokio::spawn(async move {
        evaluator::run(
            trade_intent_rx,
            approved_trade_tx,
            rejected_trade_tx,
            execution_policy,
        )
        .await;
    });

    let policy_logger = tokio::spawn(async move {
        policy_logger::run(policy_for_logger).await;
    });

    let executor = tokio::spawn(async move {
        executor::run(
            approved_trade_rx,
            binance,
            EntryModel::Instant,
            sizing_config,
        )
        .await;
    });

    let rejected_logger = tokio::spawn(async move {
        rejected_logger::run(rejected_trade_rx).await;
    });

    let ingress_secret = load_ingress_secret();
    let http = tokio::spawn(async move {
        start_server(ingress_event_tx_clone, is_test, ingress_secret).await;
    });

    RuntimeHandles {
        websocket,
        builder,
        evaluator,
        executor,
        policy_logger,
        ws_consumer,
        rejected_logger,
        http,
    }
}

fn create_sizing_config() -> MarginSizingConfig {
    match MarginSizingConfig::new(0.02, 0.98, 120) {
        Ok(config) => config,
        Err(err) => {
            error!(error = %err, "failed to create margin sizing config");
            panic!("invalid margin sizing config");
        }
    }
}

async fn wait_for_runtime_tasks(handles: RuntimeHandles) {
    let RuntimeHandles {
        websocket,
        builder,
        evaluator,
        executor,
        rejected_logger,
        policy_logger,
        ws_consumer,
        http,
    } = handles;

    let (
        websocket_result,
        builder_result,
        evaluator_result,
        executor_result,
        rejected_logger_result,
        policy_logger_result,
        ws_consumer_result,
        http_result,
    ) = tokio::join!(
        websocket,
        builder,
        evaluator,
        executor,
        rejected_logger,
        policy_logger,
        ws_consumer,
        http,
    );

    log_task_panic("websocket", websocket_result);
    log_task_panic("ws consumer", ws_consumer_result);
    log_task_panic("builder", builder_result);
    log_task_panic("evaluator", evaluator_result);
    log_task_panic("executor", executor_result);
    log_task_panic("rejected logger", rejected_logger_result);
    log_task_panic("policy logger", policy_logger_result);
    log_task_panic("http", http_result);
}

fn log_task_panic(task_name: &str, result: Result<(), tokio::task::JoinError>) {
    if let Err(err) = result {
        tracing::error!(task = task_name, error = %err, "task panicked");
    }
}
