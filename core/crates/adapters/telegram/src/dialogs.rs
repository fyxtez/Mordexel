use grammers_client::{Client, peer::Dialog};

use crate::error::TelegramError;

#[allow(dead_code)]
pub async fn load_dialogs(client: &Client) -> Result<Vec<Dialog>, TelegramError> {
    let mut iter_dialogs = client.iter_dialogs();

    let mut dialogs: Vec<Dialog> = Vec::new();

    while let Some(dialog) = iter_dialogs.next().await? {
        dialogs.push(dialog);
    }
    println!("Telegram dialogs loaded.");

    Ok(dialogs)
}
