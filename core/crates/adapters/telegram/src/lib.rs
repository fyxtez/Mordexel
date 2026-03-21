pub mod error;
pub mod utils;
pub mod types;

mod dialogs;
mod initialize;

use std::sync::Arc;

use domain::ingress_events::{IngressEvent, TelegramMessageEvent};
use grammers_client::client::UpdatesConfiguration;
use tokio::sync::mpsc;

use crate::{initialize::init, types::TelegramConfig};

use tracing::{error, info, warn};

pub async fn run(config: TelegramConfig, tx: Arc<mpsc::Sender<IngressEvent>>) {
    info!(
        signal_source_id = config.signal_source_id,
        session_path = %config.session_path.display(),
        "starting telegram adapter"
    );

    let initialization_data = match init(config.credentials, &config.session_path).await {
        Ok(data) => {
            info!("telegram adapter initialized successfully");
            data
        }
        Err(error) => {
            error!(error = %error, "failed to initialize telegram adapter");
            return;
        }
    };

    let mut updates = initialization_data
        .client
        .stream_updates(
            initialization_data.updates_receiver,
            UpdatesConfiguration {
                catch_up: false,
                ..Default::default()
            },
        )
        .await;

    info!("telegram updates handler spawned");

    loop {
        let update_result = updates.next().await;
        let update = match update_result {
            Ok(update) => update,
            Err(err) => {
                error!(error = %err, "telegram update stream failed");
                break;
            }
        };

        if let grammers_client::update::Update::NewMessage(message) = update {
            let message_peer_id = message.peer_id().bare_id();

            if message_peer_id != config.signal_source_id {
                continue;
            }

            let message_text = message.text();

            let event = IngressEvent::TelegramMessage(TelegramMessageEvent {
                peer_id: message_peer_id,
                text: message_text.to_string(),
            });

            if let Err(err) = tx.send(event).await {
                error!(error = %err, "failed to send telegram event to engine");
                break;
            }
        }
    }
    warn!("telegram adapter stopped");
}
