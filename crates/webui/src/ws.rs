use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use crate::state::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade, State(s): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, s))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    for item in state.agent.recent_change_events() {
        let payload = serde_json::json!({"event":"change-bus", "data": item}).to_string();
        if socket.send(Message::Text(payload)).await.is_err() { return; }
    }
    let mut rx = state.agent.subscribe_change_events();
    loop {
        tokio::select! {
            incoming = socket.recv() => {
                let Some(Ok(msg)) = incoming else { break; };
                if let Message::Text(text) = msg {
                    if text == "ping" {
                        let _ = socket.send(Message::Text("pong".to_string())).await;
                    }
                }
            }
            event = rx.recv() => {
                match event {
                    Ok(item) => {
                        let payload = serde_json::json!({"event":"change-bus", "data": item}).to_string();
                        if socket.send(Message::Text(payload)).await.is_err() { break; }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}
