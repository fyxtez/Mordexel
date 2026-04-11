use binance::{Binance, client::BinanceClient, utils::load_binance_config, ws::types::WsEvent};
use domain::{
    approved_trade::ApprovedTrade, ingress_events::IngressEvent, rejected_trade::RejectedTrade,
    trade_intent::TradeIntent,
};
use dotenvy::dotenv;
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::{
    execution_policy::ExecutionPolicy,
    types::{RuntimeChannels, RuntimeDeps},
    utils::{create_reqwest_client, get_build_version, init_tracing},
};

pub fn bootstrap() -> RuntimeDeps {
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

    let execution_policy = ExecutionPolicy::continuation_v1();

    let is_test = build_version == "dev";

    info!(build_version=%build_version,"Environment:");

    if build_version == "dev" && !is_test {
        panic!("🚨 DEV build cannot run in LIVE mode. This is blocked for safety.");
    }

    let binance_config = load_binance_config(is_test);
    let binance = Binance {
        client: BinanceClient {
            request_client: create_reqwest_client(),
            base_url: binance_config.base_url,
            api_key: binance_config.api_key,
            api_secret: binance_config.api_secret,
            is_test: binance_config.is_test,
        },
    };

    info!(
        mode = %if binance_config.is_test { "DEMO" } else { "LIVE" },
        "Started with binance mode"
    );

    let (ws_event_tx, ws_event_rx) = mpsc::channel::<WsEvent>(1024);
    debug!(channel_capacity = 1024, "WsEvent channel created");

    RuntimeDeps {
        execution_policy,
        binance,
        channels: RuntimeChannels {
            ingress_event_tx,
            ingress_event_rx,
            trade_intent_tx,
            trade_intent_rx,
            approved_trade_tx,
            approved_trade_rx,
            rejected_trade_tx,
            rejected_trade_rx,
            ws_event_tx,
            ws_event_rx,
        },
    }
}
