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
    Ok(Json(serde_json::json!({
        "checkpoint_saved": true,
        "snapshot": snapshot,
        "backend_backed": true,
        "receipt": control_receipt("checkpoint", &conv_id, serde_json::json!({"snapshot": snapshot}))
    })))
}

pub async fn fork(State(s): State<AppState>, Path(id): Path<String>, Json(req): Json<ForkRequest>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = parse_id(id)?;
    let fork_id = s.agent.fork_conversation(&conv_id, req.title).await.map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(serde_json::json!({
        "forked": true,
        "id": fork_id.0.to_string(),
        "from": conv_id.0.to_string(),
        "backend_backed": true,
        "receipt": control_receipt("fork", &conv_id, serde_json::json!({"to": fork_id.0.to_string()}))
    })))
}

pub async fn revert_last_turn(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = parse_id(id)?;
    let mut result = s.agent.revert_last_turn(&conv_id).await.map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    if let Some(object) = result.as_object_mut() {
        object.insert("receipt".to_string(), control_receipt("revert_last_turn", &conv_id, object.get("receipt").cloned().unwrap_or_default()));
    }
    Ok(Json(result))
}

pub async fn retry_source(State(s): State<AppState>, Path(id): Path<String>) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let conv_id = parse_id(id)?;
    let mut result = s.agent.retry_source(&conv_id).await.map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    if let Some(object) = result.as_object_mut() {
        object.insert("receipt".to_string(), control_receipt("retry_source", &conv_id, serde_json::json!({"message_found": object.get("message").is_some()})));
    }
    Ok(Json(result))
}

fn parse_id(id: String) -> Result<ConversationId, axum::http::StatusCode> {
    id.parse().map(ConversationId).map_err(|_| axum::http::StatusCode::BAD_REQUEST)
}

fn control_receipt(action: &str, id: &ConversationId, payload: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "type": "forge.session_control",
        "action": action,
        "conversation_id": id.0.to_string(),
        "status": "completed",
        "backend_backed": true,
        "payload": payload,
        "created_at": chrono::Utc::now().to_rfc3339()
    })
}
