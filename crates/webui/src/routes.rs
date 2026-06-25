//! REST API routes.

use axum::{extract::{Path, State}, Json};
use crate::state::AppState;
use forge_engine::types::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub async fn health(State(_s): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[derive(Serialize)]
pub struct ConversationListEntry {
    id: String,
    title: String,
    message_count: usize,
    mode: String,
    updated_at: String,
}

pub async fn list_conversations(State(s): State<AppState>) -> Json<Vec<ConversationListEntry>> {
    let convs = s.agent.list_conversations().await;
    Json(convs.into_iter().map(|c| ConversationListEntry {
        id: c.id.0.to_string(),
        title: c.title,
        message_count: c.message_count,
        mode: format!("{:?}", c.mode),
        updated_at: c.updated_at.to_rfc3339(),
    }).collect())
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    title: String,
}

#[derive(Serialize)]
pub struct CreateConversationResponse {
    id: String,
}

pub async fn create_conversation(
    State(s): State<AppState>,
    Json(req): Json<CreateConversationRequest>,
) -> Json<CreateConversationResponse> {
    let id = s.agent.new_conversation(req.title).await;
    Json(CreateConversationResponse { id: id.0.to_string() })
}

pub async fn get_conversation(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let conv = s.agent.get_conversation(&conv_id).await
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::to_value(conv).unwrap_or_default()))
}

pub async fn delete_conversation(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.delete_conversation(&conv_id).await
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::json!({"deleted": true})))
}

#[derive(Deserialize)]
pub struct ChatRequest {
    message: String,
    max_rounds: Option<u32>,
}

pub async fn chat(
    State(s): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let record = s.agent.chat(&conv_id, req.message).await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::to_value(record).unwrap_or_default()))
}

pub async fn cancel(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.cancel(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"cancelled": true})))
}

pub async fn pause(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.pause(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"paused": true})))
}

pub async fn resume(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.resume(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"resumed": true})))
}

pub async fn save_snapshot(
    State(s): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    s.agent.save_snapshot(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"snapshot_saved": true})))
}

pub async fn benchmark(State(s): State<AppState>) -> Json<serde_json::Value> {
    let config = forge_engine::config::Config::default();
    let adapter = forge_engine::benchmark::BenchmarkAdapter::from_config(&config);
    let report: Vec<_> = adapter.report().into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    Json(serde_json::json!({
        "score": adapter.score(),
        "capabilities": report,
    }))
}
