//! WebSocket handler for streaming chat.

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use crate::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(s): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, s))
}

async fn handle_socket(mut socket: WebSocket, _state: AppState) {
    while let Some(msg) = socket.recv().await {
        let Ok(msg) = msg else { break };
        if let Message::Text(text) = msg {
            let _ = socket.send(Message::Text(format!("echo: {}", text))).await;
        }
    }
}
