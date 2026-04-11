use binance::{Binance, ws::types::WsEvent};
use domain::{
    approved_trade::ApprovedTrade, ingress_events::IngressEvent, rejected_trade::RejectedTrade,
    trade_intent::TradeIntent,
};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::execution_policy::ExecutionPolicy;
pub struct RuntimeChannels {
    pub ingress_event_tx: Sender<IngressEvent>,
    pub ingress_event_rx: Receiver<IngressEvent>,

    pub trade_intent_tx: Sender<TradeIntent>,
    pub trade_intent_rx: Receiver<TradeIntent>,

    pub approved_trade_tx: Sender<ApprovedTrade>,
    pub approved_trade_rx: Receiver<ApprovedTrade>,

    pub rejected_trade_tx: Sender<RejectedTrade>,
    pub rejected_trade_rx: Receiver<RejectedTrade>,

    pub ws_event_tx: Sender<WsEvent>,
    pub ws_event_rx: Receiver<WsEvent>,
}

pub struct RuntimeDeps {
    pub execution_policy: ExecutionPolicy,
    pub binance: Binance,
    pub channels: RuntimeChannels,
}
