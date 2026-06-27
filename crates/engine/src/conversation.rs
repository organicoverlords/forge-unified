//! Conversation manager — CRUD for conversations with messages.

use crate::tool_parts::{finished_tool_part, running_tool_part};
use crate::types::{Conversation, ConversationId, Message, MessageRole, ToolRequest, ToolResult};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ConversationManager {
    conversations: HashMap<ConversationId, Conversation>,
}

impl ConversationManager {
    pub fn new() -> Self {
        Self {
            conversations: HashMap::new(),
        }
    }

    pub fn create(&mut self, title: String) -> ConversationId {
        let id = ConversationId(Uuid::new_v4());
        let now = chrono::Utc::now();
        let conversation = Conversation {
            id: id.clone(),
            title,
            messages: Vec::new(),
            provider: None,
            model: None,
            mode: crate::types::AgentMode::Chat,
            created_at: now,
            updated_at: now,
        };
        self.conversations.insert(id.clone(), conversation);
        id
    }

    pub fn get(&self, id: &ConversationId) -> Option<&Conversation> {
        self.conversations.get(id)
    }

    pub fn get_mut(&mut self, id: &ConversationId) -> Option<&mut Conversation> {
        self.conversations.get_mut(id)
    }

    pub fn list(&self) -> Vec<&Conversation> {
        self.conversations.values().collect()
    }

    pub fn delete(&mut self, id: &ConversationId) -> Option<Conversation> {
        self.conversations.remove(id)
    }

    pub fn insert(&mut self, id: ConversationId, conv: Conversation) {
        self.conversations.insert(id, conv);
    }

    pub fn get_messages(&self, id: &ConversationId) -> &[Message] {
        self.conversations.get(id).map(|c| c.messages.as_slice()).unwrap_or(&[])
    }

    pub fn add_user_message(&mut self, id: &ConversationId, content: String) {
        if let Some(conv) = self.conversations.get_mut(id) {
            conv.messages.push(Message {
                role: MessageRole::User,
                content,
                tool_calls: None,
                tool_results: None,
                metadata: Default::default(),
            });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn add_assistant_message(&mut self, id: &ConversationId, content: String) {
        self.add_assistant_message_with_tools(id, content, None);
    }

    pub fn add_assistant_message_with_tools(&mut self, id: &ConversationId, content: String, tool_calls: Option<Vec<ToolRequest>>) {
        if let Some(conv) = self.conversations.get_mut(id) {
            let (content, mut metadata) = normalize_assistant_content(content);
            if let Some(calls) = &tool_calls {
                metadata.insert("tool_parts".to_string(), serde_json::json!(
                    calls.iter().map(running_tool_part).collect::<Vec<_>>()
                ));
            }
            conv.messages.push(Message {
                role: MessageRole::Assistant,
                content,
                tool_calls,
                tool_results: None,
                metadata,
            });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn add_tool_results(&mut self, id: &ConversationId, results: Vec<ToolResult>) {
        if let Some(conv) = self.conversations.get_mut(id) {
            let tool_parts = results.iter().map(finished_tool_part).collect::<Vec<_>>();
            conv.messages.push(Message {
                role: MessageRole::Tool,
                content: String::new(),
                tool_calls: None,
                tool_results: Some(results),
                metadata: HashMap::from([
                    ("tool_parts".to_string(), serde_json::json!(tool_parts)),
                    ("opencode_tool_part_source".to_string(), crate::tool_parts::opencode_tool_part_source()),
                ]),
            });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn message_count(&self, id: &ConversationId) -> usize {
        self.conversations.get(id).map(|c| c.messages.len()).unwrap_or(0)
    }

    pub fn compact(&mut self, id: &ConversationId, keep_last: usize) {
        if let Some(conv) = self.conversations.get_mut(id) {
            if conv.messages.len() > keep_last {
                let system_msgs: Vec<_> = conv.messages.iter()
                    .filter(|m| m.role == MessageRole::System)
                    .cloned()
                    .collect();
                let tail: Vec<_> = conv.messages.iter().rev().take(keep_last).cloned().collect();
                conv.messages = system_msgs;
                conv.messages.extend(tail.into_iter().rev());
                conv.updated_at = chrono::Utc::now();
            }
        }
    }
}

fn normalize_assistant_content(content: String) -> (String, HashMap<String, serde_json::Value>) {
    if !content.starts_with("[Provider error:") {
        return (content, HashMap::new());
    }

    let lower = content.to_ascii_lowercase();
    let classification = if lower.contains("busy") || lower.contains("exhausted") || lower.contains("capacity") {
        "model_busy_capacity"
    } else if lower.contains("429") || lower.contains("rate") {
        "rate_limited"
    } else if lower.contains("missing") || lower.contains("unauthorized") || lower.contains("api key") {
        "missing_key"
    } else {
        "provider_error"
    };

    let message = match classification {
        "model_busy_capacity" => "Provider error: model is temporarily busy; retrying another model is safe.",
        "rate_limited" => "Provider error: model-level throttle; this is not a provider-wide outage.",
        "missing_key" => "Provider error: runtime is missing a usable model API key.",
        _ => "Provider error: model route could not complete this turn.",
    };

    let metadata = HashMap::from([
        ("type".to_string(), serde_json::json!("provider-error")),
        ("classification".to_string(), serde_json::json!(classification)),
        ("retryable".to_string(), serde_json::json!(classification != "missing_key")),
    ]);

    (message.to_string(), metadata)
}
