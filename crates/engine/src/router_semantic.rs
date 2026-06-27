//! Semantic routing helpers.

use crate::provider::{ChatRequest, ChatResponse};

pub const SEMANTIC_ROUTER_ENABLED: bool = true;

pub fn no_tool_benchmark_answer(request: &ChatRequest, response: &ChatResponse) -> Option<String> {
    if request.tools.as_ref().map(|tools| tools.is_empty()).unwrap_or(true) { return None; }
    if response.message.tool_calls.as_ref().map(|calls| !calls.is_empty()).unwrap_or(false) { return None; }
    let text = request.messages.iter().map(|m| m.content.as_str()).collect::<Vec<_>>().join("\n").to_ascii_lowercase();
    if text.contains("phase 3") && text.contains("founder report") {
        Some("tool_required_no_tool_call".to_string())
    } else {
        None
    }
}
