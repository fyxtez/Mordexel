#[derive(Debug, Clone)]
pub enum IngressEvent {
    TelegramMessage(TelegramMessageEvent),
}

#[derive(Debug, Clone)]
pub struct TelegramMessageEvent {
    pub peer_id: i64,
    pub text: String,
}
