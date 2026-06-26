//! Server-sent event endpoints for chat runs.

use crate::state::AppState;
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use forge_engine::types::{ConversationId, MessageRole};
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
                    let result = state.agent.chat(&conversation_id, message).await;
                    let mut events = VecDeque::new();

                    match result {
                        Ok(record) => {
                            if let Some(conv) = state.agent.get_conversation(&conversation_id).await {
                                if let Some(assistant) = conv.messages.iter().rev().find(|m| matches!(&m.role, MessageRole::Assistant)) {
                                    events.push_back(event("text-start", serde_json::json!({ "id": "assistant-final" })));
                                    for chunk in chunk_text(&assistant.content, 28) {
                                        events.push_back(event("text-delta", serde_json::json!({
                                            "id": "assistant-final",
                                            "text": chunk
                                        })));
                                    }
                                    events.push_back(event("text-end", serde_json::json!({ "id": "assistant-final" })));

                                    if let Some(calls) = &assistant.tool_calls {
                                        for call in calls {
                                            events.push_back(event("tool-call", serde_json::to_value(call).unwrap_or_default()));
                                        }
                                    }
                                }

                                if let Some(tool_message) = conv.messages.iter().rev().find(|m| matches!(&m.role, MessageRole::Tool)) {
                                    if let Some(results) = &tool_message.tool_results {
                                        for result in results {
                                            let event_name = if result.success { "tool-result" } else { "tool-error" };
                                            events.push_back(event(event_name, serde_json::to_value(result).unwrap_or_default()));
                                        }
                                    }
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
