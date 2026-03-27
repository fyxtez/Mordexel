use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{debug, info, warn};

use crate::{
    error::BinanceError,
    ws::{
        keys::{create_listen_key, keepalive_listen_key},
        mask::mask_tail,
        types::{RawOrderTradeUpdate, UserStreamEnvelope, WsEvent, WsEventKind},
    },
};

fn extract_order_trade_update(value: &Value) -> Result<RawOrderTradeUpdate, BinanceError> {
    let Some(order_value) = value.get("o").cloned() else {
        return Err(BinanceError::MissingField(
            "ORDER_TRADE_UPDATE missing nested 'o' payload".to_string(),
        ));
    };

    let order: RawOrderTradeUpdate = serde_json::from_value(order_value)?;
    Ok(order)
}

fn classify_order_update(order: &RawOrderTradeUpdate) -> WsEventKind {
    let exec = order.execution_type.as_deref().unwrap_or("");
    let status = order.order_status.as_deref().unwrap_or("");
    let order_type = order.order_type.as_deref().unwrap_or("");
    let original_order_type = order.original_order_type.as_deref().unwrap_or(order_type);
    let reduce_only = order.reduce_only.unwrap_or(false);
    let close_position = order.close_position.unwrap_or(false);

    if exec == "NEW" {
        return WsEventKind::OrderPlaced;
    }

    if status == "CANCELED" || exec == "CANCELED" || exec == "EXPIRED" || status == "EXPIRED" {
        return WsEventKind::Cancelled;
    }

    if exec == "TRADE" {
        let is_stop = matches!(
            original_order_type,
            "STOP" | "STOP_MARKET" | "TRAILING_STOP_MARKET"
        );

        let is_take_profit = matches!(original_order_type, "TAKE_PROFIT" | "TAKE_PROFIT_MARKET");

        if is_stop {
            return WsEventKind::StopLossHit;
        }

        if is_take_profit {
            return WsEventKind::TakeProfitHit;
        }

        if reduce_only || close_position {
            return WsEventKind::ReducedOrClosed;
        }

        return WsEventKind::PositionOpened;
    }

    WsEventKind::UnknownOrderUpdate
}

fn order_update_title(order: &RawOrderTradeUpdate, kind: WsEventKind) -> String {
    match kind {
        WsEventKind::OrderPlaced => format!("Order placed: {}", order.symbol),
        WsEventKind::PositionOpened => format!("Position opened: {}", order.symbol),
        WsEventKind::TakeProfitHit => format!("Take profit hit: {}", order.symbol),
        WsEventKind::StopLossHit => format!("Stop loss hit: {}", order.symbol),
        WsEventKind::ReducedOrClosed => format!("Position reduced/closed: {}", order.symbol),
        WsEventKind::Cancelled => format!("Order cancelled: {}", order.symbol),
        WsEventKind::UnknownOrderUpdate => format!("Order update: {}", order.symbol),
        WsEventKind::LeverageChanged => format!("Leverage changed: {}", order.symbol),
    }
}

fn order_update_message(order: &RawOrderTradeUpdate, kind: WsEventKind) -> String {
    let side = order.side.as_deref().unwrap_or("-");
    let position_side = order.position_side.as_deref().unwrap_or("-");
    let order_type = order.order_type.as_deref().unwrap_or("-");
    let original_order_type = order.original_order_type.as_deref().unwrap_or(order_type);

    let qty = order
        .accumulated_filled_qty
        .as_deref()
        .or(order.last_filled_qty.as_deref())
        .or(order.original_qty.as_deref())
        .unwrap_or("-");

    let avg_price = order.average_price.as_deref().unwrap_or("-");
    let last_price = order.last_filled_price.as_deref().unwrap_or("-");
    let stop_price = order.stop_price.as_deref().unwrap_or("-");
    let pnl = order.realized_pnl.as_deref().unwrap_or("0");
    let exec = order.execution_type.as_deref().unwrap_or("-");
    let status = order.order_status.as_deref().unwrap_or("-");
    let reduce_only = order.reduce_only.unwrap_or(false);
    let close_position = order.close_position.unwrap_or(false);

    match kind {
        WsEventKind::OrderPlaced => format!(
            "New order accepted. side={side}, position_side={position_side}, type={original_order_type}, qty={qty}, stop_price={stop_price}, reduce_only={reduce_only}, close_position={close_position}"
        ),
        WsEventKind::PositionOpened => format!(
            "Entry/order filled. side={side}, position_side={position_side}, qty={qty}, avg_price={avg_price}, last_fill_price={last_price}, type={original_order_type}, realized_pnl={pnl}"
        ),
        WsEventKind::TakeProfitHit => format!(
            "Take profit filled. side={side}, position_side={position_side}, qty={qty}, avg_price={avg_price}, last_fill_price={last_price}, realized_pnl={pnl}"
        ),
        WsEventKind::StopLossHit => format!(
            "Stop loss filled. side={side}, position_side={position_side}, qty={qty}, stop_price={stop_price}, avg_price={avg_price}, last_fill_price={last_price}, realized_pnl={pnl}"
        ),
        WsEventKind::ReducedOrClosed => format!(
            "Reduce-only / close-position order filled. side={side}, position_side={position_side}, qty={qty}, avg_price={avg_price}, last_fill_price={last_price}, realized_pnl={pnl}"
        ),
        WsEventKind::Cancelled => format!(
            "Order cancelled or expired. side={side}, position_side={position_side}, type={original_order_type}, qty={qty}, execution_type={exec}, status={status}"
        ),
        WsEventKind::UnknownOrderUpdate => format!(
            "Unclassified order event. side={side}, position_side={position_side}, type={original_order_type}, qty={qty}, avg_price={avg_price}, stop_price={stop_price}, execution_type={exec}, status={status}, realized_pnl={pnl}"
        ),
        WsEventKind::LeverageChanged => format!(
            "Leverage update received. symbol={}, side={side}, position_side={position_side}",
            order.symbol
        ),
    }
}

fn event_timestamp(env: &UserStreamEnvelope) -> u64 {
    if let Some(ts) = env.transaction_time {
        return ts;
    }

    if let Some(ts) = env.event_time {
        return ts;
    }

    chrono::Utc::now().timestamp_millis() as u64
}

fn build_order_event(order: &RawOrderTradeUpdate, env: &UserStreamEnvelope) -> WsEvent {
    let kind = classify_order_update(order);

    WsEvent {
        kind,
        symbol: Some(order.symbol.clone()),
        title: order_update_title(order, kind),
        message: order_update_message(order, kind),
        timestamp: event_timestamp(env),
    }
}

pub async fn run(
    rest_base: &str,
    api_key: &str,
    ws_base: &str,
    ws_event_tx: Sender<WsEvent>,
) -> Result<(), BinanceError> {
    let client = reqwest::Client::builder()
        .user_agent("mordexel-engine")
        .build()?;

    let listen_key = create_listen_key(&client, rest_base, api_key).await?;

    let keepalive_client = client.clone();
    let keepalive_rest_base = rest_base.to_string();
    let keepalive_api_key = api_key.to_string();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30 * 60));

        // first tick fires immediately, so consume it
        interval.tick().await;

        loop {
            interval.tick().await;

            match keepalive_listen_key(&keepalive_client, &keepalive_rest_base, &keepalive_api_key)
                .await
            {
                Ok(()) => info!("listen key keepalive sent successfully"),
                Err(err) => warn!(error = %err, "listen key keepalive failed"),
            }
        }
    });

    let ws_url = format!("{ws_base}/ws/{}", listen_key);
    let (mut ws, response) = connect_async(&ws_url)
        .await
        .map_err(|err| BinanceError::InvalidInput(format!("failed to connect websocket: {err}")))?;

    let masked_ws_url = mask_tail(&ws_url, 0, 10);

    info!(
        status = %response.status(),
        ws_url = %masked_ws_url,
        "connected to binance user stream websocket"
    );

    while let Some(msg) = ws.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let value: Value = match serde_json::from_str(&text) {
                    Ok(v) => v,
                    Err(err) => {
                        warn!(error = %err, raw = %text, "invalid websocket json payload");
                        continue;
                    }
                };

                let env: UserStreamEnvelope = match serde_json::from_value(value.clone()) {
                    Ok(v) => v,
                    Err(err) => {
                        warn!(
                            error = %err,
                            payload = %serde_json::to_string_pretty(&value)
                                .unwrap_or_else(|_| text.to_string()),
                            "failed to decode websocket event envelope"
                        );
                        continue;
                    }
                };

                match env.event_type.as_deref() {
                    Some("ORDER_TRADE_UPDATE") => match extract_order_trade_update(&value) {
                        Ok(order) => {
                            let event = build_order_event(&order, &env);

                            info!(
                                kind = ?event.kind,
                                symbol = ?event.symbol,
                                title = %event.title,
                                "parsed ORDER_TRADE_UPDATE event"
                            );

                            if ws_event_tx.send(event).await.is_err() {
                                return Err(BinanceError::InvalidInput(
                                    "ws event receiver dropped".to_string(),
                                ));
                            }
                        }
                        Err(err) => {
                            warn!(
                                error = %err,
                                payload = %serde_json::to_string_pretty(&value)
                                    .unwrap_or_else(|_| text.to_string()),
                                "failed to parse ORDER_TRADE_UPDATE"
                            );
                        }
                    },

                    Some("ACCOUNT_CONFIG_UPDATE") => {
                        debug!("received ACCOUNT_CONFIG_UPDATE");
                    }

                    Some("ACCOUNT_UPDATE") => {
                        debug!("received ACCOUNT_UPDATE");
                    }

                    Some("listenKeyExpired") => {
                        warn!("listen key expired");
                        return Err(BinanceError::InvalidInput("listen key expired".to_string()));
                    }

                    Some(other) => {
                        debug!(event_type = %other, "ignored websocket event");
                    }

                    None => {
                        debug!("websocket event missing event_type");
                    }
                }
            }

            Ok(Message::Ping(payload)) => {
                debug!(payload_len = payload.len(), "received websocket ping");

                ws.send(Message::Pong(payload)).await.map_err(|err| {
                    BinanceError::InvalidInput(format!("failed to send pong: {err}"))
                })?;
            }

            Ok(Message::Pong(_)) => {
                debug!("received websocket pong");
            }

            Ok(Message::Binary(payload)) => {
                debug!(
                    payload_len = payload.len(),
                    "ignored binary websocket message"
                );
            }

            Ok(Message::Close(frame)) => {
                info!(?frame, "websocket close frame received");
                break;
            }

            Ok(Message::Frame(_)) => {
                debug!("received raw websocket frame");
            }

            Err(err) => {
                return Err(BinanceError::InvalidInput(format!(
                    "websocket read error: {err}"
                )));
            }
        }
    }

    Err(BinanceError::InvalidInput(
        "websocket stream ended".to_string(),
    ))
}
