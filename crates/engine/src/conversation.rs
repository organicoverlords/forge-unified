//! Conversation manager — CRUD for conversations with messages.

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
            conv.messages.push(Message {
                role: MessageRole::Assistant,
                content,
                tool_calls,
                tool_results: None,
                metadata: Default::default(),
            });
            conv.updated_at = chrono::Utc::now();
        }
    }

    pub fn add_tool_results(&mut self, id: &ConversationId, results: Vec<ToolResult>) {
        if let Some(conv) = self.conversations.get_mut(id) {
            conv.messages.push(Message {
                role: MessageRole::Tool,
                content: String::new(),
                tool_calls: None,
                tool_results: Some(results),
                metadata: Default::default(),
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
