use domain::ingress_events::IngressEvent;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub tx: mpsc::Sender<IngressEvent>,
    pub is_test: bool,
}
