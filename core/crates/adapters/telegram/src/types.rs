use std::path::PathBuf;

use grammers_client::Client;
use grammers_session::updates::UpdatesLike;
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Debug)]
pub struct TelegramCredentials {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub password: String,
}

#[derive(Debug)]
pub struct TelegramConfig {
    pub signal_source_id: i64,
    pub session_path: PathBuf,
    pub credentials: TelegramCredentials,
}

pub struct InitializationData {
    pub client: Client,
    pub updates_receiver: UnboundedReceiver<UpdatesLike>,
}
