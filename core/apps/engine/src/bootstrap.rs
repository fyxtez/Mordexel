use adapter_telegram::utils::load_telegram_config;
use binance::{Binance, client::BinanceClient, utils::load_binance_config};
use dotenvy::dotenv;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{debug, info};

use crate::{
    execution_policy::ExecutionPolicy,
    utils::{create_reqwest_client, get_build_version, init_tracing},
};

use domain::{
    approved_trade::ApprovedTrade, ingress_events::IngressEvent, rejected_trade::RejectedTrade,
    trade_intent::TradeIntent,
};

pub struct RuntimeChannels {
    pub ingress_event_tx: Sender<IngressEvent>,
    pub ingress_event_rx: Receiver<IngressEvent>,

    pub trade_intent_tx: Sender<TradeIntent>,
    pub trade_intent_rx: Receiver<TradeIntent>,

    pub approved_trade_tx: Sender<ApprovedTrade>,
    pub approved_trade_rx: Receiver<ApprovedTrade>,

    pub rejected_trade_tx: Sender<RejectedTrade>,
    pub rejected_trade_rx: Receiver<RejectedTrade>,
}

pub struct RuntimeDeps {
    pub telegram_config: adapter_telegram::types::TelegramConfig,
    pub execution_policy: ExecutionPolicy,
    pub binance: Binance,
    pub channels: RuntimeChannels,
}

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

    let telegram_config = load_telegram_config();

    let execution_policy = ExecutionPolicy::strict_profit_only();

    let is_test = build_version == "DEV";

    if build_version == "DEV" && !is_test {
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

    RuntimeDeps {
        telegram_config,
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
        },
    }
}
