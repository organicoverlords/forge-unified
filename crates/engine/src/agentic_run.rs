//! OpenCode-style agentic run memory and checkpoints.
//!
//! Source parity references:
//! - `packages/core/src/session/history.ts`: runner loads only current epoch and latest compaction tail.
//! - `packages/opencode/src/session/compaction.ts`: preserve recent tail, summarize older head, prune old tool output.
//! - `packages/schema/src/session-compaction-event.ts`: session.compacted event receipt shape.

use crate::types::{ConversationId, RunId, ToolResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgenticCheckpoint {
    pub run_id: String,
    pub conversation_id: String,
    pub round: u32,
    pub total_tool_calls: u32,
    pub total_tool_failures: u32,
    pub last_tools: Vec<String>,
    pub files_touched: Vec<String>,
    pub has_todo_memory: bool,
    pub has_subagent_memory: bool,
    pub has_parallel_memory: bool,
    pub compactable: bool,
}

impl AgenticCheckpoint {
    pub fn from_results(
        run_id: &RunId,
        conversation_id: &ConversationId,
        round: u32,
        total_tool_calls: u32,
        total_tool_failures: u32,
        results: &[ToolResult],
        message_count: usize,
    ) -> Self {
        let mut files = BTreeSet::new();
        let mut last_tools = Vec::new();
        let mut has_todo_memory = false;
        let mut has_subagent_memory = false;
        let mut has_parallel_memory = false;
        for result in results {
            last_tools.push(format!("{:?}:{}", result.kind, result.success));
            if let Some(path) = result.metadata.get("path").and_then(serde_json::Value::as_str) {
                files.insert(path.to_string());
            }
            if let Some(values) = result.metadata.get("files").and_then(serde_json::Value::as_array) {
                for value in values {
                    if let Some(path) = value.get("path").or_else(|| value.get("relativePath")).and_then(serde_json::Value::as_str) {
                        files.insert(path.to_string());
                    }
                }
            }
            let alias = result.metadata.get("tool_alias").and_then(serde_json::Value::as_str);
            has_todo_memory |= alias == Some("todo_write") || result.output.contains("todo_write");
            has_subagent_memory |= format!("{:?}", result.kind) == "Task" && alias != Some("todo_write");
            has_parallel_memory |= format!("{:?}", result.kind) == "BatchParallel";
        }
        Self {
            run_id: run_id.0.to_string(),
            conversation_id: conversation_id.0.to_string(),
            round,
            total_tool_calls,
            total_tool_failures,
            last_tools,
            files_touched: files.into_iter().collect(),
            has_todo_memory,
            has_subagent_memory,
            has_parallel_memory,
            compactable: message_count > 32,
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "Agentic checkpoint round {}: tool_calls={}, failures={}, todo={}, subagent={}, parallel={}, files={}",
            self.round,
            self.total_tool_calls,
            self.total_tool_failures,
            self.has_todo_memory,
            self.has_subagent_memory,
            self.has_parallel_memory,
            if self.files_touched.is_empty() { "none".to_string() } else { self.files_touched.join(", ") }
        )
    }

    pub fn metadata(&self) -> std::collections::HashMap<String, serde_json::Value> {
        std::collections::HashMap::from([
            ("agentic_checkpoint".to_string(), serde_json::json!(self)),
            ("agentic_memory".to_string(), serde_json::json!({
                "todo": self.has_todo_memory,
                "subagent": self.has_subagent_memory,
                "parallel_tools": self.has_parallel_memory,
                "files_touched": self.files_touched,
                "last_tools": self.last_tools,
            })),
            ("opencode_session_history_source".to_string(), serde_json::json!("packages/core/src/session/history.ts")),
            ("opencode_compaction_runtime_source".to_string(), serde_json::json!("packages/opencode/src/session/compaction.ts")),
            ("opencode_checkpoint_behavior".to_string(), serde_json::json!("durable per-round session memory; preserve recent evidence; compact only when safe")),
        ])
    }
}
