mod cors;
mod state;
mod types;

use axum::Json;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Router, body::Body, extract::Request, http::StatusCode, response::IntoResponse};
use domain::ingress_events::IngressEvent;
use std::io;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::cors::build_cors_layer;
use crate::state::AppState;
use crate::types::TradeRequest;

pub async fn start_api_server(
    address: &str,
    port: u16,
    tx: mpsc::Sender<IngressEvent>,
) -> Result<(), io::Error> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    let state = AppState { tx };

    let router = Router::new()
        .route("/ping", get(ping))
        .route("/trade", post(trade))
        .layer(build_cors_layer())
        .fallback(fallback)
        .with_state(state);

    info!(address=%address,port=%port,"API Server starting at:");

    axum::serve(listener, router).await?;

    Ok(())
}

async fn ping() -> impl IntoResponse {
    (StatusCode::OK, "pong")
}

async fn trade(
    State(state): State<AppState>,
    Json(payload): Json<TradeRequest>,
) -> impl IntoResponse {
    match state
        .tx
        .send(IngressEvent::TelegramMessage(
            domain::ingress_events::TelegramMessageEvent {
                peer_id: 0,
                text: payload.text,
            },
        ))
        .await
    {
        Ok(_) => (StatusCode::OK, "Trade received"),
        Err(err) => {
            error!(error=%err, "Failed to send trade to engine channel");
            (StatusCode::INTERNAL_SERVER_ERROR, "Engine unavailable")
        }
    }
}

async fn fallback(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path();
    (
        StatusCode::NOT_FOUND,
        format!("Endpoint '{}' is not in our API.", path),
    )
}
