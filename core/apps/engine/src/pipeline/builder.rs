use domain::{ingress_events::IngressEvent, trade_intent::TradeIntent};
use signals::parser::parse_trading_signal;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub async fn run(mut rx: mpsc::Receiver<IngressEvent>, tx: mpsc::Sender<TradeIntent>) {
    info!("trade_intent_builder started");

    while let Some(event) = rx.recv().await {
        match event {
            IngressEvent::TelegramMessage(message) => {
                match parse_trading_signal(&message.text) {
                    Some(signal) => {
                        info!(?signal, "parsed trading signal");

                        let intent_result = TradeIntent::builder(signal.symbol)
                            .side(signal.side)
                            .entry(signal.entry)
                            .targets(&signal.targets)
                            .timeframe(signal.timeframe)
                            .stop_loss(signal.stop_loss)
                            .build();

                        match intent_result {
                            Ok(intent) => {
                                info!(intent_id = %intent.intent_id, "built trade intent");

                                if let Err(err) = tx.send(intent).await {
                                    error!(error = %err, "failed to send trade intent");
                                    break;
                                }
                            }
                            Err(err) => {
                                warn!(error = %err, "failed to build trade intent");
                            }
                        }
                    }
                    None => {
                        warn!(peer_id = message.peer_id, "failed to parse trading signal");
                    }
                }
            }
        }
    }

    warn!("trade_intent_builder stopped: all senders dropped");
}
