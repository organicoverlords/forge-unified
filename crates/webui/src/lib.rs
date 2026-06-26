//! Forge WebUI — axum HTTP server with REST API for the chat-first UI.

pub mod chat_ui;
pub mod routes;
pub mod state;
pub mod ws;

use axum::{response::Html, routing::{get, post}, Router};
use state::AppState;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

async fn index() -> Html<&'static str> {
    Html(chat_ui::CHAT_HTML)
}

pub async fn serve(state: AppState, addr: SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/api/health", get(routes::health))
        .route("/api/conversations", get(routes::list_conversations))
        .route("/api/conversations", post(routes::create_conversation))
        .route("/api/conversations/:id", get(routes::get_conversation))
        .route("/api/conversations/:id/chat", post(routes::chat))
        .route("/api/conversations/:id/cancel", post(routes::cancel))
        .route("/api/conversations/:id/pause", post(routes::pause))
        .route("/api/conversations/:id/resume", post(routes::resume))
        .route("/api/conversations/:id/snapshot", post(routes::save_snapshot))
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
    tracing::info!("Forge WebUI listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
