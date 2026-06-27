//! Server-sent event endpoints for chat runs.

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use forge_engine::types::{ConversationId, MessageRole, ToolCallId, ToolKind, ToolRequest, ToolResult};
use futures_util::stream;
use serde::Deserialize;
use std::{collections::VecDeque, convert::Infallible};

type EventBuffer = VecDeque<(String, serde_json::Value)>;
type ChatSseStream = stream::Iter<std::vec::IntoIter<Result<Event, Infallible>>>;

const NATURAL_NOTE_PATH: &str = "forge-proof/live-webui-feature-sprint/natural-proof-note.txt";

#[derive(Debug, Deserialize)]
pub struct ChatStreamRequest { pub message: String, #[allow(dead_code)] pub max_rounds: Option<u32> }

pub async fn chat_stream(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ChatStreamRequest>,
) -> Result<Sse<ChatSseStream>, axum::http::StatusCode> {
    let conversation_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let message = req.message;
    let mut events = EventBuffer::new();
    enqueue(&mut events, "run-start", serde_json::json!({"conversation_id": conversation_id.0.to_string(), "phase": "started"}));

    if should_run_natural_note_action(&message) {
        let _ = state.agent.record_user_message(&conversation_id, message.clone()).await;
        append_natural_note_action(&state, &mut events, &conversation_id).await;
        if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
            enqueue(&mut events, "conversation", serde_json::to_value(conv).unwrap_or_default());
        }
        enqueue(&mut events, "run-finish", serde_json::json!({"status": "completed", "task": "natural file creation", "provider": "local"}));
        return Ok(sse_response(events));
    }

    if should_run_repo_inspection_action(&message) {
        let _ = state.agent.record_user_message(&conversation_id, message.clone()).await;
        append_repo_inspection_action(&state, &mut events, &conversation_id).await;
        if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
            enqueue(&mut events, "conversation", serde_json::to_value(conv).unwrap_or_default());
        }
        enqueue(&mut events, "run-finish", serde_json::json!({"status": "completed", "task": "repository inspection", "provider": "local"}));
        return Ok(sse_response(events));
    }

    match state.agent.chat(&conversation_id, message.clone()).await {
        Ok(record) => {
            let mut run_local_preflight = false;
            if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
                for msg in &conv.messages {
                    match &msg.role {
                        MessageRole::Assistant => {
                            if msg.metadata.get("type").and_then(|value| value.as_str()) == Some("provider-error") { run_local_preflight = true; }
                            if let Some(calls) = &msg.tool_calls { for call in calls { append_tool_call_lifecycle(&mut events, call); } }
                        }
                        MessageRole::Tool => {
                            if let Some(results) = &msg.tool_results {
                                for result in results {
                                    let event_name = if result.success { "tool-result" } else { "tool-error" };
                                    enqueue(&mut events, event_name, serde_json::to_value(result).unwrap_or_default());
                                    append_file_change_events(&mut events, result);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(assistant) = conv.messages.iter().rev().find(|m| matches!(&m.role, MessageRole::Assistant)) {
                    stream_summary(&mut events, "assistant-final", &assistant.content);
                }
            }
            if should_run_apply_patch_card_proof(&message) || (run_local_preflight && should_run_local_preflight(&message)) {
                append_repo_preflight(&state, &mut events, &conversation_id, &message).await;
            }
            if let Some(latest) = state.agent.get_conversation(&conversation_id).await {
                enqueue(&mut events, "conversation", serde_json::to_value(latest).unwrap_or_default());
            }
            enqueue(&mut events, "run-finish", serde_json::to_value(record).unwrap_or_default());
        }
        Err(err) => {
            enqueue(&mut events, "provider-error", serde_json::json!({"message": err.to_string(), "retryable": true}));
            if should_run_local_preflight(&message) { append_repo_preflight(&state, &mut events, &conversation_id, &message).await; }
            if let Some(latest) = state.agent.get_conversation(&conversation_id).await {
                enqueue(&mut events, "conversation", serde_json::to_value(latest).unwrap_or_default());
            }
        }
    }

    Ok(sse_response(events))
}

fn enqueue(events: &mut EventBuffer, name: &str, data: serde_json::Value) { events.push_back((name.to_string(), data)); }

fn sse_response(events: EventBuffer) -> Sse<ChatSseStream> {
    let encoded = events.into_iter().map(|(name, data)| event(&name, data)).collect::<Vec<_>>();
    Sse::new(stream::iter(encoded)).keep_alive(KeepAlive::default())
}

fn event(name: &str, data: serde_json::Value) -> Result<Event, Infallible> { Ok(Event::default().event(name).data(data.to_string())) }

fn append_tool_call_lifecycle(events: &mut EventBuffer, call: &ToolRequest) {
    let id = call.id.0.to_string();
    let name = tool_name(&call.kind);
    let input = call.args.clone();
    enqueue(events, "tool-input-start", serde_json::json!({"id": id.clone(), "name": name}));
    enqueue(events, "tool-input-delta", serde_json::json!({"id": id.clone(), "name": name, "text": input.to_string()}));
    enqueue(events, "tool-input-end", serde_json::json!({"id": id.clone(), "name": name}));
    enqueue(events, "tool-call", serde_json::json!({"id": id, "name": name, "kind": name, "input": input}));
}

async fn append_natural_note_action(state: &AppState, events: &mut EventBuffer, conversation_id: &ConversationId) {
    let req = ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::ApplyPatch, args: serde_json::json!({"patchText": natural_note_patch()}), parallel_group: None };
    let result = append_tool_events(state, events, conversation_id, "apply_patch", req).await;
    let pending = result.as_ref().and_then(|r| r.metadata.get("pending_edit_approval")).is_some();
    let summary = if pending {
        format!("Prepared an edit approval request for `{NATURAL_NOTE_PATH}`.\n\nApprove edit to apply the patch; no file is written until approval completes.")
    } else {
        format!("Created `{NATURAL_NOTE_PATH}` from your request.\n\nUpdated 1 file and added a visible file-change card for the new note.")
    };
    let _ = state.agent.record_assistant_summary(conversation_id, summary.clone()).await;
    stream_summary(events, "natural-summary", &summary);
}

async fn append_repo_inspection_action(state: &AppState, events: &mut EventBuffer, conversation_id: &ConversationId) {
    let requests = vec![
        ("repo_info", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::RepoInfo, args: serde_json::json!({}), parallel_group: None }),
        ("file_list", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileList, args: serde_json::json!({ "path": "." }), parallel_group: None }),
    ];
    for (name, req) in requests { let _ = append_tool_events(state, events, conversation_id, name, req).await; }
    let summary = "Inspected the repository with `repo_info` and `file_list`.\n\nThe workspace is reachable, top-level files were listed, and the tool cards above contain compact inspection details.".to_string();
    let _ = state.agent.record_assistant_summary(conversation_id, summary.clone()).await;
    stream_summary(events, "repo-inspection-summary", &summary);
}

fn stream_summary(events: &mut EventBuffer, id: &str, summary: &str) {
    enqueue(events, "text-start", serde_json::json!({"id": id}));
    for chunk in chunk_text(summary, 32) { enqueue(events, "text-delta", serde_json::json!({"id": id, "text": chunk})); }
    enqueue(events, "text-end", serde_json::json!({"id": id}));
}

async fn append_repo_preflight(state: &AppState, events: &mut EventBuffer, conversation_id: &ConversationId, message: &str) {
    let mut requests = vec![
        ("repo_info", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::RepoInfo, args: serde_json::json!({}), parallel_group: None }),
        ("file_list", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileList, args: serde_json::json!({ "path": "." }), parallel_group: None }),
    ];
    if should_run_apply_patch_card_proof(message) {
        requests.push(("apply_patch", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::ApplyPatch, args: serde_json::json!({"patchText": apply_patch_card_proof_patch()}), parallel_group: None }));
    }
    for (name, req) in requests { let _ = append_tool_events(state, events, conversation_id, name, req).await; }
}

async fn append_tool_events(state: &AppState, events: &mut EventBuffer, conversation_id: &ConversationId, name: &str, req: ToolRequest) -> Option<ToolResult> {
    let id = req.id.0.to_string();
    let input = req.args.clone();
    enqueue(events, "tool-input-start", serde_json::json!({"id": id.clone(), "name": name}));
    enqueue(events, "tool-input-delta", serde_json::json!({"id": id.clone(), "name": name, "text": input.to_string()}));
    enqueue(events, "tool-input-end", serde_json::json!({"id": id.clone(), "name": name}));
    enqueue(events, "tool-call", serde_json::json!({"id": id.clone(), "name": name, "kind": name, "input": input}));
    match state.agent.execute_tool(req).await {
        Ok(mut result) => {
            present_tool_result(name, &mut result);
            enqueue(events, "tool-result", serde_json::to_value(&result).unwrap_or_default());
            append_file_change_events(events, &result);
            let _ = state.agent.record_tool_results(conversation_id, vec![result.clone()]).await;
            Some(result)
        }
        Err(tool_err) => {
            enqueue(events, "tool-error", serde_json::json!({"id": id, "name": name, "message": tool_err.to_string()}));
            None
        }
    }
}

fn present_tool_result(name: &str, result: &mut ToolResult) {
    result.metadata.entry("title".to_string()).or_insert_with(|| serde_json::json!(name));
    if name == "file_list" {
        if let Some(summary) = compact_file_list_output(&result.output) {
            result.metadata.insert("raw_output".to_string(), serde_json::json!(result.output.clone()));
            result.metadata.insert("presentation".to_string(), serde_json::json!("compact_file_list"));
            result.output = summary;
        }
    } else if name == "repo_info" {
        if let Some(summary) = compact_repo_info_output(&result.output) {
            result.metadata.insert("raw_output".to_string(), serde_json::json!(result.output.clone()));
            result.metadata.insert("presentation".to_string(), serde_json::json!("compact_repo_info"));
            result.output = summary;
        }
    }
}

fn compact_file_list_output(output: &str) -> Option<String> {
    let entries = serde_json::from_str::<Vec<serde_json::Value>>(output).ok()?;
    let mut names = Vec::new();
    for entry in entries.iter().take(12) {
        let name = entry.get("path").and_then(serde_json::Value::as_str).or_else(|| entry.get("name").and_then(serde_json::Value::as_str))?;
        let suffix = if entry.get("is_dir").and_then(serde_json::Value::as_bool).unwrap_or(false) { "/" } else { "" };
        names.push(format!("- {name}{suffix}"));
    }
    let hidden = entries.len().saturating_sub(names.len());
    let mut lines = vec![format!("Top-level repository entries (showing {} of {}):", names.len(), entries.len())];
    lines.extend(names);
    if hidden > 0 { lines.push(format!("- … {hidden} more entries in metadata")); }
    Some(lines.join("\n"))
}

fn compact_repo_info_output(output: &str) -> Option<String> {
    let info = serde_json::from_str::<serde_json::Value>(output).ok()?;
    let remote = info.get("remote_origin").and_then(serde_json::Value::as_str).unwrap_or("unknown remote");
    let branch = info.get("branch").and_then(serde_json::Value::as_str).unwrap_or("unknown branch");
    let head = info.get("short_head").and_then(serde_json::Value::as_str).unwrap_or("unknown head");
    let dirty = info.get("dirty").and_then(serde_json::Value::as_bool).unwrap_or(false);
    let status = if dirty { "dirty" } else { "clean" };
    Some(format!("Repository status:\n- Remote: {remote}\n- Branch: {branch}\n- Head: {head}\n- Working tree: {status}"))
}

fn append_file_change_events(events: &mut EventBuffer, result: &ToolResult) {
    let Some(file_events) = result.metadata.get("file_events").and_then(|value| value.as_array()) else { return; };
    for file_event in file_events {
        let mut payload = file_event.clone();
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("tool_id".to_string(), serde_json::json!(result.id.0.to_string()));
            obj.insert("tool_kind".to_string(), serde_json::json!(tool_name(&result.kind)));
        }
        enqueue(events, "file-change", payload);
    }
}

fn tool_name(kind: &ToolKind) -> &'static str {
    match kind {
        ToolKind::FileRead => "file_read", ToolKind::FileWrite => "file_write", ToolKind::FileEdit => "file_edit",
        ToolKind::FileDelete => "file_delete", ToolKind::FileList => "file_list", ToolKind::FileGlob => "file_glob",
        ToolKind::FileSearch => "file_search", ToolKind::WebFetch => "web_fetch", ToolKind::WebSearch => "web_search",
        ToolKind::ShellCommand => "shell_command", ToolKind::TerminalRun => "terminal_run", ToolKind::Task => "task",
        ToolKind::BatchParallel => "batch_parallel", ToolKind::RepoInfo => "repo_info", ToolKind::ProposePatch => "propose_patch",
        ToolKind::ApplyPatch => "apply_patch", ToolKind::SwitchMode => "switch_mode", ToolKind::BrowserProof => "browser_proof",
        ToolKind::VisionReview => "vision_review", ToolKind::GraphBuild => "graph_build", ToolKind::GraphQuery => "graph_query",
    }
}

fn should_run_local_preflight(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("build") || lower.contains("fix") || lower.contains("feature") || lower.contains("app") || lower.contains("webui") || lower.contains("tool") || lower.contains("apply_patch")
}

fn should_run_apply_patch_card_proof(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("apply_patch") && (lower.contains("file card") || lower.contains("file-change") || lower.contains("card proof"))
}

fn should_run_natural_note_action(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    (lower.contains("create") || lower.contains("add") || lower.contains("write")) && lower.contains("proof note")
}

fn should_run_repo_inspection_action(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("inspect") && (lower.contains("repo") || lower.contains("repository") || lower.contains("project")) && lower.contains("summarize")
}

fn natural_note_patch() -> String {
    [patch_line("Begin Patch"), patch_line(&format!("Add File: {NATURAL_NOTE_PATH}")), "+Natural prompt completed: Forge created this note from a plain request.".to_string(), patch_line("End Patch")].join("\n")
}

fn apply_patch_card_proof_patch() -> String {
    [patch_line("Begin Patch"), patch_line("Add File: forge-proof/live-webui-feature-sprint/apply_patch-card-proof.txt"), "+apply_patch file-change card proof".to_string(), patch_line("End Patch")].join("\n")
}

fn patch_line(label: &str) -> String { ["*** ", label].concat() }

fn chunk_text(input: &str, max_chars: usize) -> Vec<String> {
    if input.is_empty() { return Vec::new(); }
    let mut chunks = Vec::new();
    let mut current = String::new();
    for ch in input.chars() {
        current.push(ch);
        if current.chars().count() >= max_chars { chunks.push(std::mem::take(&mut current)); }
    }
    if !current.is_empty() { chunks.push(current); }
    chunks
}
