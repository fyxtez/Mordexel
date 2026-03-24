use execution::{entry::entry_model::EntryModel, sizing::types::MarginSizingConfig};
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

use domain::ingress_events::IngressEvent;
use tracing::error;

use crate::{
    bootstrap::RuntimeDeps,
    pipeline::{builder, evaluator, executor, rejected_logger},
    utils::start_server,
};

struct RuntimeHandles {
    telegram: JoinHandle<()>,
    builder: JoinHandle<()>,
    evaluator: JoinHandle<()>,
    executor: JoinHandle<()>,
    rejected_logger: JoinHandle<()>,
    http: JoinHandle<()>,
}

pub async fn run_runtime(runtime: RuntimeDeps) {
    let handles = spawn_runtime_tasks(runtime);
    wait_for_runtime_tasks(handles).await;
}

fn spawn_runtime_tasks(runtime: RuntimeDeps) -> RuntimeHandles {
    let RuntimeDeps {
        telegram_config,
        execution_policy,
        binance,
        channels,
    } = runtime;

    let is_test = binance.client.is_test;

    let ingress_event_tx_clone = channels.ingress_event_tx.clone();

    let sizing_config = create_sizing_config();

    let telegram = {
        let tx: Sender<IngressEvent> = channels.ingress_event_tx;
        tokio::spawn(async move {
            adapter_telegram::run(telegram_config, tx).await;
        })
    };

    let builder = tokio::spawn(async move {
        builder::run(channels.ingress_event_rx, channels.trade_intent_tx).await;
    });

    let evaluator = tokio::spawn(async move {
        evaluator::run(
            channels.trade_intent_rx,
            channels.approved_trade_tx,
            channels.rejected_trade_tx,
            execution_policy,
        )
        .await;
    });

    let executor = tokio::spawn(async move {
        executor::run(
            channels.approved_trade_rx,
            binance,
            EntryModel::Instant,
            sizing_config,
        )
        .await;
    });

    let rejected_logger = tokio::spawn(async move {
        rejected_logger::run(channels.rejected_trade_rx).await;
    });

    let http = tokio::spawn(async move {
        start_server(ingress_event_tx_clone, is_test).await;
    });

    RuntimeHandles {
        telegram,
        builder,
        evaluator,
        executor,
        rejected_logger,
        http,
    }
}

fn create_sizing_config() -> MarginSizingConfig {
    match MarginSizingConfig::new(0.02, 0.95, 120) {
        Ok(config) => config,
        Err(err) => {
            error!(error = %err, "failed to create margin sizing config");
            panic!("invalid margin sizing config");
        }
    }
}

async fn wait_for_runtime_tasks(handles: RuntimeHandles) {
    let RuntimeHandles {
        telegram,
        builder,
        evaluator,
        executor,
        rejected_logger,
        http,
    } = handles;

    let (
        telegram_result,
        builder_result,
        evaluator_result,
        executor_result,
        rejected_logger_result,
        http_result,
    ) = tokio::join!(
        telegram,
        builder,
        evaluator,
        executor,
        rejected_logger,
        http,
    );

    log_task_panic("telegram", telegram_result);
    log_task_panic("builder", builder_result);
    log_task_panic("evaluator", evaluator_result);
    log_task_panic("executor", executor_result);
    log_task_panic("rejected logger", rejected_logger_result);
    log_task_panic("http", http_result);
}

fn log_task_panic(task_name: &str, result: Result<(), tokio::task::JoinError>) {
    if let Err(err) = result {
        tracing::error!(task = task_name, error = %err, "task panicked");
    }
}
