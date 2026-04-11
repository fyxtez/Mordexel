use domain::{ingress_events::IngressEvent, trade_intent::TradeIntent};
use signals::parser::parse_trading_signal;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub async fn run(mut rx: mpsc::Receiver<IngressEvent>, tx: mpsc::Sender<TradeIntent>) {
    info!("trade_intent_builder started");

    while let Some(event) = rx.recv().await {
        let IngressEvent::SignalReceived(message) = event;

        if let Some(signal) = parse_trading_signal(&message.text) {
            info!(?signal, source = ?message.source, "parsed trading signal");

            match TradeIntent::builder(signal.symbol)
                .side(signal.side)
                .entry(signal.entry)
                .targets(&signal.targets)
                .timeframe(signal.timeframe)
                .stop_loss(signal.stop_loss)
                .build()
            {
                Ok(intent) => {
                    info!(
                        intent_id = %intent.intent_id,
                        source = ?message.source,
                        "built trade intent"
                    );

                    if let Err(err) = tx.send(intent).await {
                        error!(error = %err, "failed to send trade intent");
                        break;
                    }
                }
                Err(err) => {
                    warn!(error = %err, "failed to build trade intent");
                }
            }
        } else {
            warn!(
                source = ?message.source,
                external_id = ?message.external_id,
                "ignored ingress event because it was not a parseable signal"
            );
        }
    }

    warn!("trade_intent_builder stopped: all senders dropped");
}
