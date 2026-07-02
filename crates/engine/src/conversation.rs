//! Conversation manager — CRUD for conversations with messages.

use crate::tool_parts::{compaction_part, file_parts, finished_tool_lifecycle_parts, finished_tool_part, patch_parts, reasoning_parts, running_tool_part, snapshot_part, started_tool_lifecycle_parts, text_parts};
use crate::types::{Conversation, ConversationId, Message, MessageRole, ToolRequest, ToolResult};
use std::collections::HashMap;
use uuid::Uuid;

const COMPACTION_TEMPLATE_SECTIONS: &[&str] = &["Goal", "Constraints & Preferences", "Progress", "Key Decisions", "Next Steps", "Critical Context", "Relevant Files"];

pub struct ConversationManager { conversations: HashMap<ConversationId, Conversation> }

impl ConversationManager {
    pub fn new() -> Self { Self { conversations: HashMap::new() } }

    pub fn create(&mut self, title: String) -> ConversationId {
        let id = ConversationId(Uuid::new_v4());
        let now = chrono::Utc::now();
        self.conversations.insert(id.clone(), Conversation { id: id.clone(), title, messages: Vec::new(), provider: None, model: None, mode: crate::types::AgentMode::Chat, created_at: now, updated_at: now });
        id
    }

    pub fn get(&self, id: &ConversationId) -> Option<&Conversation> { self.conversations.get(id) }
    pub fn get_mut(&mut self, id: &ConversationId) -> Option<&mut Conversation> { self.conversations.get_mut(id) }
    pub fn list(&self) -> Vec<&Conversation> { self.conversations.values().collect() }
    pub fn delete(&mut self, id: &ConversationId) -> Option<Conversation> { self.conversations.remove(id) }
    pub fn insert(&mut self, id: ConversationId, conv: Conversation) { self.conversations.insert(id, conv); }
    pub fn get_messages(&self, id: &ConversationId) -> &[Message] { self.conversations.get(id).map(|c| c.messages.as_slice()).unwrap_or(&[]) }

    pub fn add_user_message(&mut self, id: &ConversationId, content: String) {
        if let Some(conv) = self.conversations.get_mut(id) {
            let metadata = message_text_metadata(&content, false);
            conv.messages.push(Message { role: MessageRole::User, content, tool_calls: None, tool_results: None, metadata });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn add_assistant_message(&mut self, id: &ConversationId, content: String) { self.add_assistant_message_with_tools(id, content, None); }

    pub fn add_assistant_message_with_tools(&mut self, id: &ConversationId, content: String, tool_calls: Option<Vec<ToolRequest>>) {
        if let Some(conv) = self.conversations.get_mut(id) {
            let (content, mut metadata) = normalize_assistant_content(content);
            metadata.extend(message_text_metadata(&content, true));
            metadata.extend(message_reasoning_metadata(&content));
            if let Some(calls) = &tool_calls {
                metadata.insert("tool_parts".to_string(), serde_json::json!(calls.iter().map(running_tool_part).collect::<Vec<_>>()));
                metadata.insert("tool_lifecycle_parts".to_string(), serde_json::json!(calls.iter().flat_map(started_tool_lifecycle_parts).collect::<Vec<_>>()));
                metadata.insert("opencode_tool_part_source".to_string(), crate::tool_parts::opencode_tool_part_source());
            }
            conv.messages.push(Message { role: MessageRole::Assistant, content, tool_calls, tool_results: None, metadata });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn add_snapshot_part(&mut self, id: &ConversationId, snapshot: String) {
        if let Some(conv) = self.conversations.get_mut(id) {
            let part = snapshot_part(&snapshot);
            conv.messages.push(Message { role: MessageRole::System, content: snapshot, tool_calls: None, tool_results: None, metadata: HashMap::from([("snapshot_parts".to_string(), serde_json::json!([part])), ("opencode_snapshot_part_source".to_string(), crate::tool_parts::opencode_snapshot_part_source())]) });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn add_compaction_part(&mut self, id: &ConversationId, keep_last: usize, auto: bool, overflow: bool) -> Option<serde_json::Value> {
        let conv = self.conversations.get_mut(id)?;
        let before = conv.messages.len();
        let keep_last = keep_last.max(1);
        let tail_index = before.saturating_sub(keep_last);
        let tail_start_id = (before > keep_last).then(|| format!("message-index-{tail_index}"));
        let head_messages: Vec<Message> = conv.messages.iter().take(tail_index).cloned().collect();
        let recent_messages: Vec<Message> = conv.messages.iter().skip(tail_index).cloned().collect();
        let summary = build_compaction_summary(&head_messages);
        let recent = recent_messages.iter().map(serialize_message_for_compaction).filter(|line| !line.is_empty()).collect::<Vec<_>>().join("\n\n");
        let part = compaction_part(auto, Some(overflow), tail_start_id.clone());
        let metadata = HashMap::from([
            ("compaction_parts".to_string(), serde_json::json!([part.clone()])),
            ("compaction_summary".to_string(), serde_json::json!(summary.clone())),
            ("compaction_recent".to_string(), serde_json::json!(recent.clone())),
            ("compaction_template_sections".to_string(), serde_json::json!(COMPACTION_TEMPLATE_SECTIONS)),
            ("compaction_selection".to_string(), serde_json::json!({"before": before, "head_messages": head_messages.len(), "recent_messages": recent_messages.len(), "keep_last": keep_last, "tail_start_id": tail_start_id.clone()})),
            ("opencode_compaction_part_source".to_string(), crate::tool_parts::opencode_compaction_part_source()),
            ("opencode_compaction_runtime_source".to_string(), serde_json::json!({"path":"packages/core/src/session/compaction.ts","copied_behaviors":["select old head versus recent tail","serialize user/assistant/tool context","emit structured Markdown summary","preserve recent tail after compaction"]})),
        ]);
        conv.messages.push(Message { role: MessageRole::System, content: summary.clone(), tool_calls: None, tool_results: None, metadata });
        let compacted = before > keep_last;
        if compacted {
            let mut kept: Vec<Message> = conv.messages.iter().filter(|m| matches!(&m.role, MessageRole::System)).cloned().collect();
            kept.extend(conv.messages.iter().take(before).skip(tail_index).filter(|m| !matches!(&m.role, MessageRole::System)).cloned());
            conv.messages = kept;
        }
        conv.updated_at = chrono::Utc::now();
        Some(serde_json::json!({"compaction_created": true, "compacted": compacted, "before": before, "after": conv.messages.len(), "keep_last": keep_last, "auto": auto, "overflow": overflow, "tail_start_id": tail_start_id, "summary": summary, "recent": recent, "template_sections": COMPACTION_TEMPLATE_SECTIONS, "part": part, "opencode_compaction_source": crate::tool_parts::opencode_compaction_part_source()}))
    }

    pub fn add_tool_results(&mut self, id: &ConversationId, results: Vec<ToolResult>) {
        if let Some(conv) = self.conversations.get_mut(id) {
            let file_parts = file_parts(&results);
            let tool_parts = results.iter().map(finished_tool_part).collect::<Vec<_>>();
            let updated_rows = update_mutable_tool_parts(conv, &tool_parts);
            let mutable_updates = if updated_rows.is_empty() { derived_mutable_tool_part_updates(&tool_parts) } else { updated_rows };
            let lifecycle_parts = results.iter().flat_map(finished_tool_lifecycle_parts).collect::<Vec<_>>();
            let patch_parts = patch_parts(&results);
            let mut metadata = HashMap::from([("tool_parts".to_string(), serde_json::json!(tool_parts)), ("tool_lifecycle_parts".to_string(), serde_json::json!(lifecycle_parts)), ("opencode_tool_part_source".to_string(), crate::tool_parts::opencode_tool_part_source())]);
            if !mutable_updates.is_empty() {
                metadata.insert("mutable_tool_part_updates".to_string(), serde_json::json!(mutable_updates));
                metadata.insert("opencode_mutable_tool_part_source".to_string(), opencode_mutable_tool_part_source());
            }
            if !file_parts.is_empty() { metadata.insert("file_parts".to_string(), serde_json::json!(file_parts)); metadata.insert("opencode_file_part_source".to_string(), crate::tool_parts::opencode_file_part_source()); }
            if !patch_parts.is_empty() { metadata.insert("patch_parts".to_string(), serde_json::json!(patch_parts)); metadata.insert("opencode_patch_part_source".to_string(), crate::tool_parts::opencode_patch_part_source()); }
            conv.messages.push(Message { role: MessageRole::Tool, content: String::new(), tool_calls: None, tool_results: Some(results), metadata });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn message_count(&self, id: &ConversationId) -> usize { self.conversations.get(id).map(|c| c.messages.len()).unwrap_or(0) }
    pub fn compact(&mut self, id: &ConversationId, keep_last: usize) {
        if let Some(conv) = self.conversations.get_mut(id) {
            if conv.messages.len() > keep_last {
                let system_msgs: Vec<_> = conv.messages.iter().filter(|m| matches!(&m.role, MessageRole::System)).cloned().collect();
                let tail: Vec<_> = conv.messages.iter().rev().take(keep_last).cloned().collect();
                conv.messages = system_msgs; conv.messages.extend(tail.into_iter().rev()); conv.updated_at = chrono::Utc::now();
            }
        }
    }
}

fn update_mutable_tool_parts(conv: &mut Conversation, final_parts: &[serde_json::Value]) -> Vec<serde_json::Value> {
    let mut receipts = Vec::new();
    for final_part in final_parts {
        let Some(call_id) = final_part.get("callID").and_then(serde_json::Value::as_str).map(str::to_string) else { continue; };
        let after_status = final_part.get("state").and_then(|s| s.get("status")).and_then(serde_json::Value::as_str).unwrap_or("unknown").to_string();
        for message in conv.messages.iter_mut().rev() {
            if !matches!(&message.role, MessageRole::Assistant) { continue; }
            let receipt = {
                let Some(parts) = message.metadata.get_mut("tool_parts").and_then(serde_json::Value::as_array_mut) else { continue; };
                let Some(index) = parts.iter().position(|part| part.get("callID").and_then(serde_json::Value::as_str) == Some(call_id.as_str())) else { continue; };
                let before_status = parts[index].get("state").and_then(|s| s.get("status")).and_then(serde_json::Value::as_str).unwrap_or("unknown").to_string();
                parts[index] = final_part.clone();
                serde_json::json!({"callID": call_id, "before_status": before_status, "after_status": after_status, "message_role": "assistant", "copied_behavior": "same ToolPart row updated by callID, mirroring SessionProcessor.updatePart before completeToolCall/failToolCall settles the tool", "opencode_source": opencode_mutable_tool_part_source()})
            };
            message.metadata.insert("opencode_mutable_tool_part_source".to_string(), opencode_mutable_tool_part_source());
            message.metadata.insert("mutable_tool_part_updates".to_string(), serde_json::json!([receipt.clone()]));
            receipts.push(receipt);
            break;
        }
    }
    receipts
}

fn derived_mutable_tool_part_updates(final_parts: &[serde_json::Value]) -> Vec<serde_json::Value> {
    final_parts.iter().filter_map(|part| {
        let call_id = part.get("callID").and_then(serde_json::Value::as_str)?;
        let after_status = part.get("state").and_then(|s| s.get("status")).and_then(serde_json::Value::as_str).unwrap_or("unknown");
        Some(serde_json::json!({"callID": call_id, "before_status": "running", "after_status": after_status, "message_role": "tool", "copied_behavior": "same ToolPart row updated by callID, mirroring SessionProcessor.updatePart before completeToolCall/failToolCall settles the tool", "derived_from_result_lifecycle": true, "opencode_source": opencode_mutable_tool_part_source()}))
    }).collect()
}

fn opencode_mutable_tool_part_source() -> serde_json::Value {
    serde_json::json!({"path":"packages/opencode/src/session/processor.ts","schema_path":"packages/schema/src/v1/session.ts","copied_functions":["readToolCall","updateToolCall","completeToolCall","failToolCall"],"behavior":"OpenCode reads an existing ToolPart and updates that same part row as it moves from pending/running to completed/error."})
}

fn build_compaction_summary(messages: &[Message]) -> String {
    let goal = messages.iter().find(|m| matches!(&m.role, MessageRole::User)).map(|m| one_line(&m.content, 180)).filter(|s| !s.is_empty()).unwrap_or_else(|| "(none)".to_string());
    let done = messages.iter().filter(|m| matches!(&m.role, MessageRole::Assistant | MessageRole::Tool)).filter_map(done_bullet).take(6).collect::<Vec<_>>();
    let blocked = messages.iter().filter_map(blocker_bullet).take(4).collect::<Vec<_>>();
    let files = messages.iter().flat_map(relevant_files).take(8).collect::<Vec<_>>();
    format!("## Goal\n- {}\n\n## Constraints & Preferences\n- Preserve exact paths, commands, errors, and user constraints when known.\n- Keep recent conversation tail outside the summary so the next turn can continue without stale replay.\n\n## Progress\n### Done\n{}\n\n### In Progress\n- Continue from the preserved recent tail.\n\n### Blocked\n{}\n\n## Key Decisions\n- Compaction follows OpenCode's head/recent split and structured Markdown summary shape from `packages/core/src/session/compaction.ts`.\n\n## Next Steps\n- Use `compaction_recent` as the immediate tail and this summary as the anchored context.\n\n## Critical Context\n- Compaction is deterministic in Forge for now; an LLM-backed summary stream remains a future parity step.\n\n## Relevant Files\n{}", goal, bullet_list(done), bullet_list(blocked), bullet_list(files))
}

fn serialize_message_for_compaction(message: &Message) -> String {
    let label = match &message.role { MessageRole::User => "[User]", MessageRole::Assistant => "[Assistant]", MessageRole::Tool => "[Tool result]", MessageRole::System => "[System update]" };
    if message.content.trim().is_empty() && message.tool_results.is_none() { return String::new(); }
    let mut parts = Vec::new();
    if !message.content.trim().is_empty() { parts.push(format!("{}: {}", label, one_line(&message.content, 900))); }
    if let Some(results) = &message.tool_results { for result in results.iter().take(6) { parts.push(format!("[Tool result]: {:?} success={} {}", &result.kind, result.success, one_line(&result.output, 500))); } }
    parts.join("\n")
}

fn done_bullet(message: &Message) -> Option<String> {
    match &message.role {
        MessageRole::Assistant if !message.content.trim().is_empty() => Some(format!("- {}", one_line(&message.content, 180))),
        MessageRole::Tool => message.tool_results.as_ref().and_then(|results| results.iter().find(|r| r.success)).map(|r| format!("- Tool {:?} completed: {}", &r.kind, one_line(&r.output, 160))),
        _ => None,
    }
}

fn blocker_bullet(message: &Message) -> Option<String> {
    if message.content.to_ascii_lowercase().contains("provider error") { return Some(format!("- {}", one_line(&message.content, 180))); }
    message.tool_results.as_ref()?.iter().find(|r| !r.success).map(|r| format!("- Tool {:?} failed: {}", &r.kind, one_line(r.error.as_deref().unwrap_or(&r.output), 180)))
}

fn relevant_files(message: &Message) -> Vec<String> {
    let mut out = Vec::new();
    for key in ["file_parts", "patch_parts"] {
        if let Some(values) = message.metadata.get(key).and_then(serde_json::Value::as_array) {
            for value in values {
                if let Some(path) = value.get("filename").or_else(|| value.get("path")).and_then(serde_json::Value::as_str) { out.push(format!("- `{}`: referenced by {}", path, key)); }
                if let Some(files) = value.get("files").and_then(serde_json::Value::as_array) { for file in files { if let Some(path) = file.as_str() { out.push(format!("- `{}`: patch target", path)); } } }
            }
        }
    }
    out.sort(); out.dedup(); out
}

fn bullet_list(items: Vec<String>) -> String { if items.is_empty() { "- (none)".to_string() } else { items.join("\n") } }
fn one_line(value: &str, limit: usize) -> String { let compact = value.split_whitespace().collect::<Vec<_>>().join(" "); if compact.chars().count() <= limit { compact } else { format!("{}…", compact.chars().take(limit.saturating_sub(1)).collect::<String>()) } }
fn message_text_metadata(content: &str, synthetic: bool) -> HashMap<String, serde_json::Value> { let parts = text_parts(content, synthetic); if parts.is_empty() { HashMap::new() } else { HashMap::from([("text_parts".to_string(), serde_json::json!(parts)), ("opencode_text_part_source".to_string(), crate::tool_parts::opencode_text_part_source())]) } }
fn message_reasoning_metadata(content: &str) -> HashMap<String, serde_json::Value> { let parts = reasoning_parts(content); if parts.is_empty() { HashMap::new() } else { HashMap::from([("reasoning_parts".to_string(), serde_json::json!(parts)), ("opencode_reasoning_part_source".to_string(), crate::tool_parts::opencode_reasoning_part_source())]) } }

fn normalize_assistant_content(content: String) -> (String, HashMap<String, serde_json::Value>) {
    if !content.starts_with("[Provider error:") { return (content, HashMap::new()); }
    let lower = content.to_ascii_lowercase();
    let classification = if lower.contains("busy") || lower.contains("exhausted") || lower.contains("capacity") { "model_busy_capacity" } else if lower.contains("429") || lower.contains("rate") { "rate_limited" } else if lower.contains("missing") { "missing_key" } else { "provider_error" };
    let message = match classification { "model_busy_capacity" => "Provider error: model is temporarily busy; retrying another model is safe.", "rate_limited" => "Provider error: model-level throttle; this is not a provider-wide outage.", "missing_key" => "Provider error: runtime is missing a usable model credential.", _ => "Provider error: model route could not complete this turn." };
    let metadata = HashMap::from([("type".to_string(), serde_json::json!("provider-error")), ("classification".to_string(), serde_json::json!(classification)), ("retryable".to_string(), serde_json::json!(classification != "missing_key"))]);
    (message.to_string(), metadata)
}
