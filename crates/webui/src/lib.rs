#![allow(warnings, clippy::all)]

pub mod change_events;
pub mod chat_ui;
pub mod events;
pub mod routes;
pub mod state;
pub mod ws;

use axum::{response::Html, routing::{delete, get, post}, Router};
use state::AppState;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

async fn index() -> Html<&'static str> { Html(chat_ui::CHAT_HTML) }

pub async fn serve(state: AppState, addr: SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/events", get(change_events::events_page))
        .route("/api/health", get(routes::health))
        .route("/api/events/recent", get(change_events::recent_events))
        .route("/api/events/status", get(change_events::event_status))
        .route("/api/events/stream", get(change_events::stream_events))
        .route("/api/conversations", get(routes::list_conversations))
        .route("/api/conversations", post(routes::create_conversation))
        .route("/api/conversations/:id", get(routes::get_conversation))
        .route("/api/conversations/:id", delete(routes::delete_conversation))
        .route("/api/conversations/:id/chat", post(routes::chat))
        .route("/api/conversations/:id/chat/stream", post(events::chat_stream))
        .route("/api/conversations/:id/cancel", post(routes::cancel))
        .route("/api/conversations/:id/pause", post(routes::pause))
        .route("/api/conversations/:id/resume", post(routes::resume))
        .route("/api/conversations/:id/snapshot", post(routes::save_snapshot))
        .route("/api/conversations/:id/compact", post(routes::compact_conversation))
        .route("/api/conversations/:id/approvals/:approval_id/approve", post(routes::approve_edit))
        .route("/api/browser-proof", post(routes::browser_proof))
        .route("/api/vision-review", post(routes::vision_review))
        .route("/api/benchmark", get(routes::benchmark))
        .route("/api/graph", get(routes::graph_visualization))
        .route("/api/graph/data", get(routes::graph_data))
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
