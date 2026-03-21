use adapter_http::start_api_server;
use adapter_telegram::utils::load_telegram_config;
use binance::{Binance, client::BinanceClient, utils::load_binance_config};
use domain::{
    approved_trade::ApprovedTrade, ingress_events::IngressEvent, rejected_trade::RejectedTrade,
    trade_intent::TradeIntent,
};
use dotenvy::dotenv;
use std::sync::Arc;

use crate::{
    execution_policy::ExecutionPolicy,
    pipeline::{builder, evaluator, executor, rejected_logger},
    utils::{create_reqwest_client, get_build_version, init_tracing},
};
use tokio::sync::mpsc::{self, Sender};
use tracing::{debug, error, info};

pub async fn run() {
    init_tracing();

    let build_version = get_build_version();
    info!(build_version = %build_version, "starting mordexel");

    dotenv().ok();
    debug!("dotenv loaded");

    let (ingress_event_tx, ingress_event_rx) = mpsc::channel::<IngressEvent>(1024);
    debug!(channel_capacity = 1024, "IngressEvent channel created");

    let (trade_intent_tx, trade_intent_rx) = mpsc::channel::<TradeIntent>(1024);
    debug!(channel_capacity = 1024, "TradeIntent channel created");

    let (approved_trade_tx, approved_trade_rx) = mpsc::channel::<ApprovedTrade>(1024);
    debug!(channel_capacity = 1024, "ApprovedTrade channel created");

    let (rejected_trade_tx, rejected_trade_rx) = mpsc::channel::<RejectedTrade>(1024);
    debug!(channel_capacity = 1024, "RejectedTrade channel created");

    // TODO: Check if i really need Arc here.
    let shared_tx = Arc::new(ingress_event_tx);

    let shared_tx_telegram_clone = shared_tx.clone();

    let telegram_config = load_telegram_config();
    let telegram_handle = tokio::spawn(async move {
        adapter_telegram::run(telegram_config, shared_tx_telegram_clone).await;
    });

    // TODO: Policy should not always be one.
    // Eventually in the future signals will be better, so instead
    // of tracking all trades executed since the start
    // evaluate the performance of the symbols in the shorter term.
    // -> allows adding more symbols to strict profit list if they
    // satisfy profatibility parameters.
    let policy = ExecutionPolicy::strict_profit_only();

    let trade_intent_builder_handle = tokio::spawn(async move {
        builder::run(ingress_event_rx, trade_intent_tx).await;
    });

    let trade_intent_evaluator_handle = tokio::spawn(async move {
        evaluator::run(
            trade_intent_rx,
            approved_trade_tx,
            rejected_trade_tx,
            policy,
        )
        .await;
    });

    let binance_config = load_binance_config(true);

    let binance_client = Binance {
        client: BinanceClient {
            request_client: create_reqwest_client(),
            base_url: binance_config.base_url,
            api_key: binance_config.api_key,
            api_secret: binance_config.api_secret,
        },
    };

    let approved_trade_executor_handle = tokio::spawn(async move {
        executor::run(approved_trade_rx, binance_client).await;
    });

    let rejected_trade_logger_handle = tokio::spawn(async move {
        rejected_logger::run(rejected_trade_rx).await;
    });

    let shared_tx_api_clone = shared_tx.clone();
    let http_handle = tokio::spawn(async move {
        start_server(shared_tx_api_clone).await;
    });

    let _ = tokio::join!(
        telegram_handle,
        trade_intent_builder_handle,
        trade_intent_evaluator_handle,
        approved_trade_executor_handle,
        rejected_trade_logger_handle,
        http_handle,
    );
}

async fn start_server(tx: Arc<Sender<IngressEvent>>) {
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
