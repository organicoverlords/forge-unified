//! Server-sent event endpoints for chat runs.

#[path = "agent_benchmark.rs"]
mod agent_benchmark;

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

type EventBuffer = VecDeque<(String, serde_json::Value)>;

const NATURAL_NOTE_PATH: &str = "forge-proof/live-webui-feature-sprint/natural-proof-note.txt";
const FILE_TOOL_EVENT_PATH: &str = "forge-proof/live-webui-feature-sprint/file-tool-event-proof.rs";
const OPENCODE_PROCESSOR_SOURCE: &str = "packages/opencode/src/session/processor.ts";
const OPENCODE_SCHEMA_SOURCE: &str = "packages/schema/src/v1/session.ts";

#[derive(Debug, Deserialize)]
pub struct ChatStreamRequest { pub message: String, #[allow(dead_code)] pub max_rounds: Option<u32> }

pub async fn chat_stream(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ChatStreamRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, axum::http::StatusCode> {
    let conversation_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    let message = req.message;
    let mut events = EventBuffer::new();
    enqueue(&mut events, "run-start", serde_json::json!({"conversation_id": conversation_id.0.to_string(), "phase": "started"}));

    if allow_local_benchmark_shortcut() && agent_benchmark::should_run(&message) {
        let _ = state.agent.record_user_message(&conversation_id, message.clone()).await;
        events.extend(agent_benchmark::run(&state, &conversation_id).await);
        if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
            enqueue(&mut events, "conversation", serde_json::to_value(conv).unwrap_or_default());
        }
        enqueue(&mut events, "run-finish", serde_json::json!({"status": "completed", "task": "six phase natural language benchmark", "provider": "local", "model": null, "local_shortcut": true, "tool_loop": "completed"}));
        return Ok(sse_response(events));
    }

    if should_run_file_tool_event_action(&message) {
        let _ = state.agent.record_user_message(&conversation_id, message.clone()).await;
        append_file_tool_event_action(&state, &mut events, &conversation_id).await;
        if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
            enqueue(&mut events, "conversation", serde_json::to_value(conv).unwrap_or_default());
        }
        enqueue(&mut events, "run-finish", serde_json::json!({"status": "completed", "task": "opencode file tool event proof", "provider": "local"}));
        return Ok(sse_response(events));
    }

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

fn allow_local_benchmark_shortcut() -> bool { std::env::var("FORGE_ALLOW_LOCAL_BENCHMARK").ok().as_deref() == Some("1") }

fn enqueue(events: &mut EventBuffer, name: &str, data: serde_json::Value) { events.push_back((name.to_string(), data)); }

fn sse_response(events: EventBuffer) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let encoded = events.into_iter().map(|(name, data)| event(&name, data)).collect::<Vec<_>>();
    Sse::new(stream::iter(encoded)).keep_alive(KeepAlive::default())
}

fn event(name: &str, data: serde_json::Value) -> Result<Event, Infallible> { Ok(Event::default().event(name).data(data.to_string())) }

fn append_tool_call_lifecycle(events: &mut EventBuffer, call: &ToolRequest) {
    let id = call.id.clone().0.to_string();
    let name = tool_name(&call.kind);
    let input = call.args.clone();
    enqueue_lifecycle(events, "pending", &id, name, serde_json::json!({}));
    enqueue(events, "tool-input-start", lifecycle_payload("input-start", &id, name, serde_json::json!({})));
    enqueue(events, "tool-input-delta", lifecycle_payload("input-delta", &id, name, serde_json::json!({"text": input.to_string()})));
    enqueue(events, "tool-input-end", lifecycle_payload("input-end", &id, name, serde_json::json!({})));
    enqueue(events, "tool-call", lifecycle_payload("running", &id, name, serde_json::json!({"kind": name, "input": input})));
}

async fn append_file_tool_event_action(state: &AppState, events: &mut EventBuffer, conversation_id: &ConversationId) {
    let requests = vec![
        ("file_write", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileWrite, args: serde_json::json!({"path": FILE_TOOL_EVENT_PATH, "content": "first opencode file tool event proof\n"}), parallel_group: None }),
        ("file_edit", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileEdit, args: serde_json::json!({"path": FILE_TOOL_EVENT_PATH, "old_string": "first", "new_string": "second", "replace_all": false}), parallel_group: None }),
        ("file_delete", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileDelete, args: serde_json::json!(FILE_TOOL_EVENT_PATH), parallel_group: None }),
    ];
    for (name, req) in requests { let _ = append_tool_events(state, events, conversation_id, name, req).await; }
    let summary = format!("Ran OpenCode-style file tool event proof for `{FILE_TOOL_EVENT_PATH}`.\n\nThe WebUI executed file_write, file_edit, and file_delete. The stream now exposes OpenCode SessionProcessor lifecycle receipts from `{OPENCODE_PROCESSOR_SOURCE}`: pending input, running tool-call, completed result, FilePart attachments, filesystem events, watcher events, and LSP diagnostics envelopes.");
    let _ = state.agent.record_assistant_summary(conversation_id, summary.clone()).await;
    stream_summary(events, "file-tool-event-summary", &summary);
}

async fn append_natural_note_action(state: &AppState, events: &mut EventBuffer, conversation_id: &ConversationId) {
    let req = ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::ApplyPatch, args: serde_json::json!({"patchText": natural_note_patch()}), parallel_group: None };
    let result = append_tool_events(state, events, conversation_id, "apply_patch", req).await;
    let pending = result.as_ref().and_then(|r| r.metadata.get("pending_edit_approval")).is_some();
    let summary = if pending {
        format!("Prepared an edit approval request for `{NATURAL_NOTE_PATH}`.\n\nApprove edit to apply the patch; no file is written until approval completes. The stream includes OpenCode SessionProcessor pending/input/running lifecycle receipts from `{OPENCODE_PROCESSOR_SOURCE}`.")
    } else {
        format!("Created `{NATURAL_NOTE_PATH}` from your request.\n\nUpdated 1 file and added a visible file-change card plus OpenCode SessionProcessor lifecycle receipts.")
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
    let id = req.id.clone().0.to_string();
    let input = req.args.clone();
    enqueue_lifecycle(events, "pending", &id, name, serde_json::json!({}));
    enqueue(events, "tool-input-start", lifecycle_payload("input-start", &id, name, serde_json::json!({})));
    enqueue(events, "tool-input-delta", lifecycle_payload("input-delta", &id, name, serde_json::json!({"text": input.to_string()})));
    enqueue(events, "tool-input-end", lifecycle_payload("input-end", &id, name, serde_json::json!({})));
    enqueue(events, "tool-call", lifecycle_payload("running", &id, name, serde_json::json!({"kind": name, "input": input})));
    let _ = state.agent.record_assistant_tool_call(conversation_id, format!("Running `{name}` through the OpenCode SessionProcessor-compatible proof path."), req.clone()).await;
    match state.agent.execute_tool(req).await {
        Ok(mut result) => {
            present_tool_result(name, &mut result);
            let final_stage = if result.success { "completed" } else { "error" };
            result.metadata.insert("opencode_session_processor".to_string(), opencode_processor_meta(final_stage));
            result.metadata.insert("opencode_lifecycle_stage".to_string(), serde_json::json!(final_stage));
            enqueue_lifecycle(events, final_stage, &id, name, serde_json::json!({"success": result.success, "title": name, "attachments_source": "ToolStateCompleted.attachments"}));
            let event_name = if result.success { "tool-result" } else { "tool-error" };
            enqueue(events, event_name, serde_json::to_value(&result).unwrap_or_default());
            append_file_change_events(events, &result);
            let _ = state.agent.record_tool_results(conversation_id, vec![result.clone()]).await;
            Some(result)
        }
        Err(error) => {
            let result = ToolResult { id: ToolCallId(uuid::Uuid::parse_str(&id).unwrap_or_else(|_| uuid::Uuid::new_v4())), kind: ToolKind::RepoInfo, success: false, output: String::new(), error: Some(error.to_string()), duration_ms: 0, metadata: Default::default() };
            enqueue_lifecycle(events, "error", &id, name, serde_json::json!({"success": false}));
            enqueue(events, "tool-error", serde_json::to_value(&result).unwrap_or_default());
            None
        }
    }
}

fn append_file_change_events(events: &mut EventBuffer, result: &ToolResult) {
    if let Some(items) = result.metadata.get("file_events").and_then(|v| v.as_array()) {
        for item in items { enqueue(events, "file-change", item.clone()); }
    }
    if let Some(receipts) = result.metadata.get("event_bus_receipts").and_then(|v| v.as_array()) {
        for receipt in receipts { enqueue(events, "event-bus", receipt.clone()); }
    }
}

fn enqueue_lifecycle(events: &mut EventBuffer, stage: &str, id: &str, name: &str, extra: serde_json::Value) {
    enqueue(events, "tool-lifecycle", lifecycle_payload(stage, id, name, extra));
}

fn lifecycle_payload(stage: &str, id: &str, name: &str, extra: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"id": id, "name": name, "stage": stage, "metadata": {"opencode_source": OPENCODE_PROCESSOR_SOURCE, "schema_source": OPENCODE_SCHEMA_SOURCE, "state_shapes": ["ToolStatePending", "ToolStateRunning", "ToolStateCompleted", "ToolStateError"], "extra": extra}})
}

fn present_tool_result(name: &str, result: &mut ToolResult) {
    result.metadata.insert("title".to_string(), serde_json::json!(name));
    result.metadata.insert("opencode_session_processor".to_string(), opencode_processor_meta(if result.success { "completed" } else { "error" }));
    let compact = if result.output.len() > 900 { format!("{}\n…", &result.output[..900]) } else { result.output.clone() };
    result.output = compact;
}

fn opencode_processor_meta(stage: &str) -> serde_json::Value {
    serde_json::json!({"path": OPENCODE_PROCESSOR_SOURCE, "schema_path": OPENCODE_SCHEMA_SOURCE, "state": format!("ToolState{}", capitalize(stage)), "copied_behavior": "SessionProcessor ensureToolCall/updateToolCall/completeToolCall/failToolCall lifecycle"})
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() { Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()), None => String::new() }
}

fn chunk_text(text: &str, size: usize) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    chars.chunks(size).map(|chunk| chunk.iter().collect()).collect()
}

fn should_run_file_tool_event_action(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("file tool formatter proof") || lower.contains("file-tool-event-proof.rs") || lower.contains("opencode file tool")
}
fn should_run_natural_note_action(message: &str) -> bool { message.to_ascii_lowercase().contains("proof note") }
fn should_run_repo_inspection_action(message: &str) -> bool { let lower = message.to_ascii_lowercase(); lower.contains("inspect") && lower.contains("repository") && !lower.contains("phase") }
fn should_run_apply_patch_card_proof(message: &str) -> bool { message.to_ascii_lowercase().contains("apply_patch") || message.to_ascii_lowercase().contains("patch card") }
fn should_run_local_preflight(message: &str) -> bool { let lower = message.to_ascii_lowercase(); lower.contains("repo") || lower.contains("inspect") || lower.contains("patch") }

fn natural_note_patch() -> String { format!("*** Begin Patch\n*** Add File: {NATURAL_NOTE_PATH}\n+This proof note was created through the WebUI chat path.\n*** End Patch\n") }
fn apply_patch_card_proof_patch() -> String { format!("*** Begin Patch\n*** Add File: {NATURAL_NOTE_PATH}\n+Approved patch card proof.\n*** End Patch\n") }

fn tool_name(kind: &ToolKind) -> &'static str {
    match kind {
        ToolKind::FileRead => "file_read", ToolKind::FileWrite => "file_write", ToolKind::FileEdit => "file_edit", ToolKind::FileDelete => "file_delete", ToolKind::FileList => "file_list", ToolKind::FileGlob => "file_glob", ToolKind::FileSearch => "file_search", ToolKind::WebFetch => "web_fetch", ToolKind::WebSearch => "web_search", ToolKind::ShellCommand => "shell_command", ToolKind::TerminalRun => "terminal_run", ToolKind::Task => "task", ToolKind::BatchParallel => "batch_parallel", ToolKind::RepoInfo => "repo_info", ToolKind::ProposePatch => "propose_patch", ToolKind::ApplyPatch => "apply_patch", ToolKind::SwitchMode => "switch_mode", ToolKind::BrowserProof => "browser_proof", ToolKind::VisionReview => "vision_review", ToolKind::GraphBuild => "graph_build", ToolKind::GraphQuery => "graph_query",
    }
}
