use std::{io, path::PathBuf, sync::Arc};

use crate::{
    error::TelegramError,
    types::{Credentials, InitializationData},
};
use grammers_client::{Client, SignInError};
use grammers_mtsender::SenderPool;
use grammers_session::storages::SqliteSession;
use tracing::info;

pub async fn init(
    credentials: Credentials,
    session_path: &PathBuf,
) -> Result<InitializationData, TelegramError> {
    let sender_pool = create_sender_pool(session_path, credentials.api_id).await?;

    let client = Client::new(sender_pool.handle);

    tokio::spawn(sender_pool.runner.run());

    if !client.is_authorized().await? {
        let token = client
            .request_login_code(&credentials.phone_number, credentials.api_hash.as_str())
            .await?;

        println!("Enter the OTP code: ");
        let mut code = String::new();
        io::stdin().read_line(&mut code)?;
        let code = code.trim();

        match client.sign_in(&token, code).await {
            Ok(_) => println!("Logged in successfully!"),
            Err(SignInError::PasswordRequired(password_token)) => {
                client
                    .check_password(password_token, &credentials.password)
                    .await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    let me = client.get_me().await?;
    let full_name = me.full_name().to_string();

    info!(
        name = %full_name,
        "Connected to Telegram via"
    );

    Ok(InitializationData {
        client,
        updates_receiver: sender_pool.updates,
    })
}

async fn create_sender_pool(
    session_path: &PathBuf,
    api_id: i32,
) -> Result<SenderPool, TelegramError> {
    Ok(SenderPool::new(
        Arc::new(
            SqliteSession::open(session_path)
                .await
                .map_err(|e| TelegramError::Other(e.to_string()))?,
        ),
        api_id,
    ))
}
