#[derive(Debug, Clone)]
pub enum IngressEvent {
    SignalReceived(SignalReceivedEvent),
}

#[derive(Debug, Clone)]
pub struct SignalReceivedEvent {
    pub source: SignalSource,
    pub external_id: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum SignalSource {
    Telegram,
    Http,
    Replay,
    Manual,
}
