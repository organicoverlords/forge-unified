//! Backend-backed conversation controls for visible WebUI session actions.

use axum::{extract::{Path, State}, Json};
use crate::state::AppState;
use forge_engine::types::ConversationId;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ForkRequest { title: Option<String> }

pub async fn checkpoint(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = parse_id(id)?;
    let snapshot = s.agent.save_snapshot_with_part(&conv_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({"checkpoint_saved": true, "snapshot": snapshot, "backend_backed": true})))
}

pub async fn fork(State(s): State<AppState>, Path(id): Path<String>, Json(req): Json<ForkRequest>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = parse_id(id)?;
    let fork_id = s.agent.fork_conversation(&conv_id, req.title).await.map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::json!({"forked": true, "id": fork_id.0.to_string(), "from": conv_id.0.to_string(), "backend_backed": true})))
}

pub async fn revert_last_turn(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = parse_id(id)?;
    let result = s.agent.revert_last_turn(&conv_id).await.map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    Ok(Json(result))
}

pub async fn retry_source(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = parse_id(id)?;
    let result = s.agent.retry_source(&conv_id).await.map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(result))
}

fn parse_id(id: String) -> Result<ConversationId, axum::http::StatusCode> {
    id.parse().map(ConversationId).map_err(|_| axum::http::StatusCode::BAD_REQUEST)
}
