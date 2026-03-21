use std::sync::Arc;

use domain::ingress_events::IngressEvent;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub tx: Arc<mpsc::Sender<IngressEvent>>,
}
