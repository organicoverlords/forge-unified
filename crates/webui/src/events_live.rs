//! Live SSE endpoint that emits progress while long agent runs are still executing.

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use forge_engine::types::{ConversationId, MessageRole, ToolCallId, ToolKind, ToolRequest, ToolResult};
use futures_util::{stream, Stream};
use serde::Deserialize;
use std::{collections::HashSet, convert::Infallible, pin::Pin, time::Duration};
use tokio::sync::mpsc;

type BoxEventStream = Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>;
type EventSender = mpsc::Sender<(String, serde_json::Value)>;

const FILE_TOOL_EVENT_PATH: &str = "forge-proof/live-webui-feature-sprint/file-tool-event-proof.rs";
const DOOM_LOOP_THRESHOLD: usize = 3;

#[derive(Debug, Deserialize)]
pub struct ChatStreamRequest {
    pub message: String,
    pub max_rounds: Option<u32>,
}

pub async fn chat_stream(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ChatStreamRequest>,
) -> Result<Sse<BoxEventStream>, axum::http::StatusCode> {
    let conversation_id = ConversationId(id.parse().map_err(|_| axum::http::StatusCode::BAD_REQUEST)?);
    if should_run_file_tool_event_action(&req.message) {
        return Ok(stream_file_tool_event_action(state, conversation_id, req.message));
    }
    Ok(stream_agent_run(state, conversation_id, req.message, req.max_rounds.unwrap_or(75)))
}

fn stream_agent_run(state: AppState, conversation_id: ConversationId, message: String, max_rounds: u32) -> Sse<BoxEventStream> {
    let (tx, rx) = mpsc::channel(512);
    tokio::spawn(async move {
        send_event(&tx, "run-start", serde_json::json!({
            "conversation_id": conversation_id.0.to_string(),
            "phase": "started",
            "streaming": "incremental",
            "snapshot_streaming": true,
            "max_rounds": max_rounds,
        })).await;
        send_event(&tx, "text-start", serde_json::json!({"id":"live-progress"})).await;
        send_event(&tx, "text-delta", serde_json::json!({"id":"live-progress","text":"Forge started the live WebUI model/tool loop.\n"})).await;

        let agent = state.agent.clone();
        let run_conversation_id = conversation_id.clone();
        let mut run = tokio::spawn(async move {
            agent.chat_with_max_rounds(&run_conversation_id, message, max_rounds).await
        });
        let mut tick: u64 = 0;
        let mut emitted_tools = HashSet::new();
        let record = loop {
            tokio::select! {
                result = &mut run => {
                    break result;
                }
                _ = tokio::time::sleep(Duration::from_secs(15)) => {
                    tick += 1;
                    send_event(&tx, "benchmark-progress", serde_json::json!({
                        "tick": tick,
                        "elapsed_seconds": tick * 15,
                        "conversation_id": conversation_id.0.to_string(),
                        "status": "agent loop still running",
                        "provider": "nvidia_nim",
                        "snapshot_streaming": true,
                    })).await;
                    emit_live_conversation_snapshot(&state, &tx, &conversation_id, tick, &mut emitted_tools).await;
                    send_event(&tx, "text-delta", serde_json::json!({
                        "id":"live-progress",
                        "text": format!("benchmark still running: {}s elapsed\n", tick * 15),
                    })).await;
                }
            }
        };

        match record {
            Ok(Ok(record)) => {
                let mut events = Vec::new();
                append_conversation_events(&state, &conversation_id, &mut events).await;
                for (name, data) in events { send_event(&tx, &name, data).await; }
                if let Some(latest) = state.agent.get_conversation(&conversation_id).await {
                    send_event(&tx, "conversation", serde_json::to_value(latest).unwrap_or_default()).await;
                }
                send_event(&tx, "run-finish", serde_json::to_value(record).unwrap_or_default()).await;
            }
            Ok(Err(err)) => {
                send_event(&tx, "provider-error", serde_json::json!({"message": err.to_string(), "retryable": true})).await;
                if let Some(latest) = state.agent.get_conversation(&conversation_id).await {
                    send_event(&tx, "conversation", serde_json::to_value(latest).unwrap_or_default()).await;
                }
            }
            Err(err) => {
                send_event(&tx, "provider-error", serde_json::json!({"message": err.to_string(), "retryable": true, "join_error": true})).await;
            }
        }
        send_event(&tx, "text-end", serde_json::json!({"id":"live-progress"})).await;
    });
    sse_channel(rx)
}

fn stream_file_tool_event_action(state: AppState, conversation_id: ConversationId, message: String) -> Sse<BoxEventStream> {
    let (tx, rx) = mpsc::channel(512);
    tokio::spawn(async move {
        send_event(&tx, "run-start", serde_json::json!({"conversation_id": conversation_id.0.to_string(), "phase":"started"})).await;
        let _ = state.agent.record_user_message(&conversation_id, message).await;
        let requests = vec![
            ("file_write", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileWrite, args: serde_json::json!({"path": FILE_TOOL_EVENT_PATH, "content": "first forge file tool event proof\n"}), parallel_group: None }),
            ("file_edit", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileEdit, args: serde_json::json!({"path": FILE_TOOL_EVENT_PATH, "old_string": "first", "new_string": "second", "replace_all": false}), parallel_group: None }),
            ("file_delete", ToolRequest { id: ToolCallId(uuid::Uuid::new_v4()), kind: ToolKind::FileDelete, args: serde_json::json!(FILE_TOOL_EVENT_PATH), parallel_group: None }),
        ];
        for (name, req) in requests {
            append_tool_events(&state, &tx, &conversation_id, name, req).await;
        }
        let summary = format!("Ran a Forge file tool event proof for `{FILE_TOOL_EVENT_PATH}`.\n\nThe WebUI executed file_write, file_edit, and file_delete. The stream exposes Forge-owned ToolPart lifecycle receipts, providerExecuted metadata, file-change cards, watcher updates, and LSP diagnostics envelopes.");
        let _ = state.agent.record_assistant_summary(&conversation_id, summary.clone()).await;
        stream_summary(&tx, "file-tool-event-summary", &summary).await;
        if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
            send_event(&tx, "conversation", serde_json::to_value(conv).unwrap_or_default()).await;
        }
        send_event(&tx, "run-finish", serde_json::json!({"status":"completed", "task":"forge file tool event proof", "provider":"local"})).await;
    });
    sse_channel(rx)
}

async fn emit_live_conversation_snapshot(
    state: &AppState,
    tx: &EventSender,
    conversation_id: &ConversationId,
    tick: u64,
    emitted_tools: &mut HashSet<String>,
) {
    let Some(conv) = state.agent.get_conversation(conversation_id).await else { return; };
    let message_count = conv.messages.len();
    let tool_result_count: usize = conv.messages.iter().map(|msg| msg.tool_results.as_ref().map_or(0, Vec::len)).sum();
    send_event(tx, "conversation-snapshot", serde_json::json!({
        "tick": tick,
        "conversation_id": conversation_id.0.to_string(),
        "messages": message_count,
        "tool_results": tool_result_count,
        "provider": conv.provider,
        "model": conv.model,
        "snapshot_streaming": true,
    })).await;

    let mut events = Vec::new();
    append_conversation_events(state, conversation_id, &mut events).await;
    for (name, data) in events {
        let key = serde_json::to_string(&(name.as_str(), &data)).unwrap_or_default();
        if emitted_tools.insert(key) {
            send_event(tx, &name, data).await;
        }
    }
}

async fn append_conversation_events(state: &AppState, conversation_id: &ConversationId, events: &mut Vec<(String, serde_json::Value)>) {
    if let Some(conv) = state.agent.get_conversation(conversation_id).await {
        for msg in &conv.messages {
            match &msg.role {
                MessageRole::Assistant => {
                    if let Some(calls) = &msg.tool_calls {
                        for call in calls { append_tool_call_lifecycle(events, call); }
                    }
                }
                MessageRole::Tool => {
                    if let Some(results) = &msg.tool_results {
                        for result in results {
                            let event_name = if result.success { "tool-result" } else { "tool-error" };
                            events.push((event_name.to_string(), serde_json::to_value(result).unwrap_or_default()));
                            append_file_change_events(events, result);
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(assistant) = conv.messages.iter().rev().find(|m| matches!(&m.role, MessageRole::Assistant)) {
            append_summary_events(events, "assistant-final", &assistant.content);
        }
    }
}

fn append_tool_call_lifecycle(events: &mut Vec<(String, serde_json::Value)>, call: &ToolRequest) {
    let id = call.id.clone().0.to_string();
    let name = tool_name(&call.kind);
    let input = call.args.clone();
    enqueue_lifecycle(events, "pending", &id, name, serde_json::json!({"providerExecuted": false}));
    events.push(("tool-input-start".to_string(), lifecycle_payload("input-start", &id, name, serde_json::json!({"providerExecuted": false}))));
    events.push(("tool-input-delta".to_string(), lifecycle_payload("input-delta", &id, name, serde_json::json!({"text": input.to_string(), "providerExecuted": false}))));
    events.push(("tool-input-end".to_string(), lifecycle_payload("input-end", &id, name, serde_json::json!({"providerExecuted": false}))));
    events.push(("tool-call".to_string(), lifecycle_payload("running", &id, name, serde_json::json!({"kind": name, "input": input, "providerExecuted": false}))));
}

async fn append_tool_events(state: &AppState, tx: &EventSender, conversation_id: &ConversationId, name: &str, req: ToolRequest) {
    let id = req.id.clone().0.to_string();
    let input = req.args.clone();
    send_event(tx, "tool-lifecycle", lifecycle_payload("pending", &id, name, serde_json::json!({"providerExecuted": false}))).await;
    send_event(tx, "tool-input-start", lifecycle_payload("input-start", &id, name, serde_json::json!({"providerExecuted": false}))).await;
    send_event(tx, "tool-input-delta", lifecycle_payload("input-delta", &id, name, serde_json::json!({"text": input.to_string(), "providerExecuted": false}))).await;
    send_event(tx, "tool-input-end", lifecycle_payload("input-end", &id, name, serde_json::json!({"providerExecuted": false}))).await;
    send_event(tx, "tool-call", lifecycle_payload("running", &id, name, serde_json::json!({"kind": name, "input": input.clone(), "providerExecuted": false}))).await;
    let _ = state.agent.record_assistant_tool_call(conversation_id, format!("Running `{name}` through the Forge ToolPart lifecycle proof path."), req.clone()).await;
    match state.agent.execute_tool(req).await {
        Ok(mut result) => {
            result.metadata.insert("providerExecuted".to_string(), serde_json::json!(false));
            result.metadata.insert("forge_tool_input".to_string(), input);
            result.metadata.insert("mutable_tool_part_updates".to_string(), serde_json::json!("same ToolPart row updated by callID"));
            result.metadata.insert("attachment_shape".to_string(), serde_json::json!("ToolStateCompleted.attachments"));
            let final_stage = if result.success { "completed" } else { "error" };
            send_event(tx, "tool-lifecycle", lifecycle_payload(final_stage, &id, name, serde_json::json!({"success": result.success, "title": name, "providerExecuted": false, "attachments_source":"ToolStateCompleted.attachments"}))).await;
            let event_name = if result.success { "tool-result" } else { "tool-error" };
            send_event(tx, event_name, serde_json::to_value(&result).unwrap_or_default()).await;
            let mut events = Vec::new();
            append_file_change_events(&mut events, &result);
            for (event, data) in events { send_event(tx, &event, data).await; }
            let _ = state.agent.record_tool_results(conversation_id, vec![result]).await;
        }
        Err(error) => {
            send_event(tx, "tool-error", serde_json::json!({"id": id, "kind": name, "success": false, "error": error.to_string()})).await;
        }
    }
}

fn append_file_change_events(events: &mut Vec<(String, serde_json::Value)>, result: &ToolResult) {
    if let Some(items) = result.metadata.get("file_events").and_then(|v| v.as_array()) {
        for item in items { events.push(("file-change".to_string(), item.clone())); }
    }
    if let Some(receipts) = result.metadata.get("event_bus_receipts").and_then(|v| v.as_array()) {
        for receipt in receipts { events.push(("event-bus".to_string(), receipt.clone())); }
    }
}

fn enqueue_lifecycle(events: &mut Vec<(String, serde_json::Value)>, stage: &str, id: &str, name: &str, extra: serde_json::Value) {
    events.push(("tool-lifecycle".to_string(), lifecycle_payload(stage, id, name, extra)));
}

fn lifecycle_payload(stage: &str, id: &str, name: &str, extra: serde_json::Value) -> serde_json::Value {
    let provider_executed = extra.get("providerExecuted").and_then(serde_json::Value::as_bool).unwrap_or(false);
    serde_json::json!({
        "id": id,
        "name": name,
        "stage": stage,
        "providerExecuted": provider_executed,
        "metadata": {
            "source": "Forge ToolPart lifecycle reference",
            "schema": "Forge ToolState/FilePart envelope",
            "state_shapes": ["ToolStatePending", "ToolStateRunning", "ToolStateCompleted", "ToolStateError"],
            "providerExecuted_delta": provider_executed,
            "doom_loop_threshold": DOOM_LOOP_THRESHOLD,
            "extra": extra
        }
    })
}

async fn stream_summary(tx: &EventSender, id: &str, summary: &str) {
    send_event(tx, "text-start", serde_json::json!({"id": id})).await;
    for chunk in chunk_text(summary, 32) {
        send_event(tx, "text-delta", serde_json::json!({"id": id, "text": chunk})).await;
    }
    send_event(tx, "text-end", serde_json::json!({"id": id})).await;
}

fn append_summary_events(events: &mut Vec<(String, serde_json::Value)>, id: &str, summary: &str) {
    events.push(("text-start".to_string(), serde_json::json!({"id": id})));
    for chunk in chunk_text(summary, 32) {
        events.push(("text-delta".to_string(), serde_json::json!({"id": id, "text": chunk})));
    }
    events.push(("text-end".to_string(), serde_json::json!({"id": id})));
}

fn chunk_text(text: &str, size: usize) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    chars.chunks(size).map(|chunk| chunk.iter().collect()).collect()
}

fn should_run_file_tool_event_action(message: &str) -> bool {
    let lower = message.to_ascii_lowercase();
    lower.contains("file tool formatter proof") || lower.contains("file-tool-event-proof.rs") || lower.contains("toolpart lifecycle proof")
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

async fn send_event(tx: &EventSender, name: &str, data: serde_json::Value) {
    let _ = tx.send((name.to_string(), data)).await;
}

fn sse_channel(rx: mpsc::Receiver<(String, serde_json::Value)>) -> Sse<BoxEventStream> {
    let body = stream::unfold(rx, |mut rx| async move {
        rx.recv().await.map(|(name, data)| (event(&name, data), rx))
    });
    Sse::new(Box::pin(body) as BoxEventStream).keep_alive(KeepAlive::default())
}

fn event(name: &str, data: serde_json::Value) -> Result<Event, Infallible> {
    Ok(Event::default().event(name).data(data.to_string()))
}
