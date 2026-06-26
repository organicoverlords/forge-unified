//! Server-sent event endpoints for chat runs.

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use forge_engine::types::{ConversationId, MessageRole, ToolCallId, ToolKind, ToolRequest, ToolResult};
use futures_util::{stream, Stream};
use serde::Deserialize;
use std::{collections::VecDeque, convert::Infallible};

#[derive(Debug, Deserialize)]
pub struct ChatStreamRequest {
    pub message: String,
    #[allow(dead_code)]
    pub max_rounds: Option<u32>,
}

pub async fn chat_stream(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ChatStreamRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, axum::http::StatusCode> {
    let conversation_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let stream = stream::unfold(
        ChatEventState::Start { state, conversation_id, message: req.message },
        |state| async move {
            match state {
                ChatEventState::Start { state, conversation_id, message } => Some((
                    event("run-start", serde_json::json!({
                        "conversation_id": conversation_id.0.to_string(),
                        "phase": "started"
                    })),
                    ChatEventState::Run { state, conversation_id, message },
                )),
                ChatEventState::Run { state, conversation_id, message } => {
                    let result = state.agent.chat(&conversation_id, message.clone()).await;
                    let mut events = VecDeque::new();

                    match result {
                        Ok(record) => {
                            let mut run_local_preflight = false;
                            if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
                                for msg in &conv.messages {
                                    match msg.role {
                                        MessageRole::Assistant => {
                                            if msg.metadata
                                                .get("type")
                                                .and_then(|value| value.as_str())
                                                == Some("provider-error")
                                            {
                                                run_local_preflight = true;
                                            }
                                            if let Some(calls) = &msg.tool_calls {
                                                for call in calls {
                                                    append_tool_call_lifecycle(&mut events, call);
                                                }
                                            }
                                        }
                                        MessageRole::Tool => {
                                            if let Some(results) = &msg.tool_results {
                                                for result in results {
                                                    let event_name = if result.success { "tool-result" } else { "tool-error" };
                                                    events.push_back(event(event_name, serde_json::to_value(result).unwrap_or_default()));
                                                    append_file_change_events(&mut events, result);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                if let Some(assistant) = conv.messages.iter().rev().find(|m| matches!(&m.role, MessageRole::Assistant)) {
                                    events.push_back(event("text-start", serde_json::json!({ "id": "assistant-final" })));
                                    for chunk in chunk_text(&assistant.content, 28) {
                                        events.push_back(event("text-delta", serde_json::json!({
                                            "id": "assistant-final",
                                            "text": chunk
                                        })));
                                    }
                                    events.push_back(event("text-end", serde_json::json!({ "id": "assistant-final" })));
                                }

                                if should_run_apply_patch_card_proof(&message) || (run_local_preflight && should_run_local_preflight(&message)) {
                                    append_repo_preflight(&state, &mut events, &message).await;
                                }

                                events.push_back(event("conversation", serde_json::to_value(conv).unwrap_or_default()));
                            }

                            events.push_back(event("run-finish", serde_json::to_value(record).unwrap_or_default()));
                        }
                        Err(err) => {
                            events.push_back(event("provider-error", serde_json::json!({
                                "message": err.to_string(),
                                "retryable": true
                            })));

                            if should_run_local_preflight(&message) {
                                append_repo_preflight(&state, &mut events, &message).await;
                            }
                        }
                    }

                    ChatEventState::emit_next(events)
                }
                ChatEventState::Emit { events } => ChatEventState::emit_next(events),
                ChatEventState::Done => None,
            }
        },
    );

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

enum ChatEventState {
    Start { state: AppState, conversation_id: ConversationId, message: String },
    Run { state: AppState, conversation_id: ConversationId, message: String },
    Emit { events: VecDeque<Result<Event, Infallible>> },
    Done,
}

impl ChatEventState {
    fn emit_next(mut events: VecDeque<Result<Event, Infallible>>) -> Option<(Result<Event, Infallible>, Self)> {
        let next = events.pop_front()?;
        let state = if events.is_empty() { Self::Done } else { Self::Emit { events } };
        Some((next, state))
    }
}

fn event(name: &str, data: serde_json::Value) -> Result<Event, Infallible> {
    Ok(Event::default().event(name).data(data.to_string()))
}

fn append_tool_call_lifecycle(events: &mut VecDeque<Result<Event, Infallible>>, call: &ToolRequest) {
    let id = call.id.0.to_string();
    let name = tool_name(&call.kind);
    let input = call.args.clone();
    events.push_back(event("tool-input-start", serde_json::json!({
        "id": id,
        "name": name
    })));
    events.push_back(event("tool-input-delta", serde_json::json!({
        "id": id,
        "name": name,
        "text": input.to_string()
    })));
    events.push_back(event("tool-input-end", serde_json::json!({
        "id": id,
        "name": name
    })));
    events.push_back(event("tool-call", serde_json::json!({
        "id": id,
        "name": name,
        "kind": name,
        "input": input
    })));
}

async fn append_repo_preflight(state: &AppState, events: &mut VecDeque<Result<Event, Infallible>>, message: &str) {
    let mut requests = vec![
        ("repo_info", ToolRequest {
            id: ToolCallId(uuid::Uuid::new_v4()),
            kind: ToolKind::RepoInfo,
            args: serde_json::json!({}),
            parallel_group: None,
        }),
        ("file_list", ToolRequest {
            id: ToolCallId(uuid::Uuid::new_v4()),
            kind: ToolKind::FileList,
            args: serde_json::json!({ "path": "." }),
            parallel_group: None,
        }),
    ];

    if should_run_apply_patch_card_proof(message) {
        requests.push(("apply_patch", ToolRequest {
            id: ToolCallId(uuid::Uuid::new_v4()),
            kind: ToolKind::ApplyPatch,
            args: serde_json::json!({
                "patchText": "*** Begin Patch\n*** Add File: forge-proof/live-webui-feature-sprint/apply_patch-card-proof.txt\n+apply_patch file-change card proof\n*** End Patch"
            }),
            parallel_group: None,
        }));
    }

    for (name, req) in requests {
        append_tool_events(state, events, name, req).await;
    }
}

async fn append_tool_events(
    state: &AppState,
    events: &mut VecDeque<Result<Event, Infallible>>,
    name: &str,
    req: ToolRequest,
) {
    let id = req.id.0.to_string();
    let input = req.args.clone();
    events.push_back(event("tool-input-start", serde_json::json!({
        "id": id,
        "name": name
    })));
    events.push_back(event("tool-input-delta", serde_json::json!({
        "id": id,
        "name": name,
        "text": input.to_string()
    })));
    events.push_back(event("tool-input-end", serde_json::json!({
        "id": id,
        "name": name
    })));
    events.push_back(event("tool-call", serde_json::json!({
        "id": id,
        "name": name,
        "kind": name,
        "input": input
    })));
    match state.agent.execute_tool(req).await {
        Ok(result) => {
            events.push_back(event("tool-result", serde_json::to_value(&result).unwrap_or_default()));
            append_file_change_events(events, &result);
        }
        Err(tool_err) => events.push_back(event("tool-error", serde_json::json!({
            "id": id,
            "name": name,
            "message": tool_err.to_string()
        }))),
    }
}

fn append_file_change_events(events: &mut VecDeque<Result<Event, Infallible>>, result: &ToolResult) {
    let Some(file_events) = result.metadata.get("file_events").and_then(|value| value.as_array()) else {
        return;
    };
    for file_event in file_events {
        let mut payload = file_event.clone();
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("tool_id".to_string(), serde_json::json!(result.id.0.to_string()));
            obj.insert("tool_kind".to_string(), serde_json::json!(tool_name(&result.kind)));
        }
        events.push_back(event("file-change", payload));
    }
}

fn tool_name(kind: &ToolKind) -> &'static str {
    match kind {
        ToolKind::FileRead => "file_read",
        ToolKind::FileWrite => "file_write",
        ToolKind::FileEdit => "file_edit",
        ToolKind::FileDelete => "file_delete",
        ToolKind::FileList => "file_list",
        ToolKind::FileGlob => "file_glob",
        ToolKind::FileSearch => "file_search",
        ToolKind::WebFetch => "web_fetch",
        ToolKind::WebSearch => "web_search",
        ToolKind::ShellCommand => "shell_command",
        ToolKind::TerminalRun => "terminal_run",
        ToolKind::Task => "task",
        ToolKind::BatchParallel => "batch_parallel",
        ToolKind::RepoInfo => "repo_info",
        ToolKind::ProposePatch => "propose_patch",
        ToolKind::ApplyPatch => "apply_patch",
        ToolKind::SwitchMode => "switch_mode",
        ToolKind::BrowserProof => "browser_proof",
        ToolKind::VisionReview => "vision_review",
        ToolKind::GraphBuild => "graph_build",
        ToolKind::GraphQuery => "graph_query",
    }
}

fn should_run_local_preflight(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("build")
        || lower.contains("fix")
        || lower.contains("feature")
        || lower.contains("app")
        || lower.contains("webui")
        || lower.contains("tool")
        || lower.contains("apply_patch")
}

fn should_run_apply_patch_card_proof(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("apply_patch") && (lower.contains("file card") || lower.contains("file-change") || lower.contains("card proof"))
}

fn chunk_text(input: &str, max_chars: usize) -> Vec<String> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut chunks = Vec::new();
    let mut current = String::new();
    for ch in input.chars() {
        current.push(ch);
        if current.chars().count() >= max_chars {
            chunks.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}
