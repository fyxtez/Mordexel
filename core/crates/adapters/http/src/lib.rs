mod cors;
mod state;
mod types;

use axum::Json;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Router, body::Body, extract::Request, http::StatusCode, response::IntoResponse};
use domain::ingress_events::{IngressEvent, SignalReceivedEvent, SignalSource};
use std::io;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::cors::build_cors_layer;
use crate::state::AppState;
use crate::types::{SignalIngressRequest, TradeRequest};

pub async fn start_api_server(
    address: &str,
    port: u16,
    tx: mpsc::Sender<IngressEvent>,
    is_test: bool,
) -> Result<(), io::Error> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    let state = AppState { tx, is_test };

    let router = Router::new()
        .route("/ping", get(ping))
        .route("/trade", post(trade))
        .route("/ingress/signal", post(ingress_signal))
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
    let build_version = option_env!("BUILD_VERSION").unwrap_or("dev");
    let is_dev_build = build_version.trim().eq_ignore_ascii_case("dev");

    if !is_dev_build {
        return (
            StatusCode::FORBIDDEN,
            "Trade endpoint is only enabled in DEV builds",
        );
    }

    if !state.is_test {
        return (
            StatusCode::BAD_REQUEST,
            "DEV build must use Binance testnet (Demo) for /trade",
        );
    }
    match state
        .tx
        .send(IngressEvent::SignalReceived(SignalReceivedEvent {
            source: SignalSource::Manual,
            external_id: None,
            text: payload.text,
        }))
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

fn parse_signal_source(raw: &str) -> Result<SignalSource, &'static str> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "telegram" => Ok(SignalSource::Telegram),
        "http" => Ok(SignalSource::Http),
        "replay" => Ok(SignalSource::Replay),
        "manual" => Ok(SignalSource::Manual),
        _ => Err("unsupported source"),
    }
}

async fn ingress_signal(
    State(state): State<AppState>,
    Json(payload): Json<SignalIngressRequest>,
) -> impl IntoResponse {
    let source = match parse_signal_source(&payload.source) {
        Ok(source) => source,
        Err(msg) => return (StatusCode::BAD_REQUEST, msg),
    };

    match state
        .tx
        .send(IngressEvent::SignalReceived(SignalReceivedEvent {
            source,
            external_id: payload.external_id,
            text: payload.text,
        }))
        .await
    {
        Ok(_) => (StatusCode::OK, "Signal received"),
        Err(err) => {
            error!(error=%err, "Failed to send ingress signal to engine channel");
            (StatusCode::INTERNAL_SERVER_ERROR, "Engine unavailable")
        }
    }
}
