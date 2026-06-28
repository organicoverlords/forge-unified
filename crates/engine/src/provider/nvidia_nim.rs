//! NVIDIA NIM provider implementation.

#![allow(dead_code)]

use crate::provider::{ChatRequest, ChatResponse, ChatStream, Provider, StreamEvent, TokenUsage, ToolCallDelta};
use crate::types::{ProviderConfig, ProviderId, Message, MessageRole};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Duration;

const DEFAULT_MAX_TOKENS: u32 = 4096;
const MAX_REQUEST_TEXT_CHARS: usize = 24_000;
const MAX_TOOL_RESULT_CHARS: usize = 8_000;

pub struct NvidiaNimProvider {
    config: ProviderConfig,
    client: Client,
}

impl NvidiaNimProvider {
    pub fn new(config: ProviderConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .expect("Failed to create HTTP client");
        Self { config, client }
    }

    fn api_key(&self) -> Result<String> {
        let primary = &self.config.api_key_env;
        let nim_env = concat!("NVIDIA_NIM", "_API_KEY");
        let short_env = concat!("NIM", "_KEY");
        std::env::var(primary)
            .or_else(|_| std::env::var(nim_env))
            .or_else(|_| std::env::var(short_env))
            .with_context(|| format!("Missing configured NVIDIA NIM credential env var"))
    }
}

#[async_trait]
impl Provider for NvidiaNimProvider {
    fn id(&self) -> &ProviderId { &self.config.id }
    fn config(&self) -> &ProviderConfig { &self.config }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let api_key = self.api_key()?;
        let max_tokens = request.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS).clamp(1, DEFAULT_MAX_TOKENS);

        let mut body = json!({
            "model": request.model.0,
            "messages": self.format_messages(&request.messages),
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": max_tokens,
            "stream": false,
        });

        if let Some(tools) = request.tools.as_ref().filter(|t| !t.is_empty()) {
            let oai_tools: Vec<serde_json::Value> = tools.iter().map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    }
                })
            }).collect();
            body["tools"] = serde_json::to_value(oai_tools)?;
            body["tool_choice"] = json!(request.tool_choice.as_deref().unwrap_or("auto"));
        }

        let response = self.client
            .post(format!("{}/chat/completions", self.config.api_base))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("NIM request failed: {}", e))?;

        let status = response.status();
        let text = response.text().await
            .map_err(|e| anyhow::anyhow!("NIM response body read failed: {}", e))?;

        if !status.is_success() {
            anyhow::bail!("NIM API error {}: {}", status, text);
        }

        let nims_response: NimResponse = serde_json::from_str(&text)
            .map_err(|e| anyhow::anyhow!("NIM response parse failed: {} | body: {}", e, text.chars().take(500).collect::<String>()))?;
        let choice = nims_response.choices.into_iter().next()
            .context("No response from model")?;

        let tool_requests = choice.message.tool_calls.map(|tc| {
            let deltas: Vec<ToolCallDelta> = tc.into_iter().filter_map(|v| {
                let function = v.get("function").unwrap_or(&v);
                let id = v.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string();
                let raw_name = function.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let name = normalize_nim_tool_name(raw_name).to_string();
                let arguments = tool_arguments_as_json_string(function.get("arguments"));
                if name.is_empty() { None } else { Some(ToolCallDelta { id, name, arguments }) }
            }).collect();
            crate::provider::tool_calls_from_deltas(deltas)
        });

        Ok(ChatResponse {
            message: Message { role: MessageRole::Assistant, content: choice.message.content.unwrap_or_default(), tool_calls: tool_requests, tool_results: None, metadata: Default::default() },
            usage: nims_response.usage.map(|u| TokenUsage { prompt_tokens: u.prompt_tokens, completion_tokens: u.completion_tokens, total_tokens: u.total_tokens }),
            provider: self.config.id.clone(),
            model: request.model,
        })
    }

    async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream> {
        let api_key = self.api_key()?;
        let max_tokens = request.max_tokens.unwrap_or(DEFAULT_MAX_TOKENS).clamp(1, DEFAULT_MAX_TOKENS);

        let mut body = json!({
            "model": request.model.0,
            "messages": self.format_messages(&request.messages),
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": max_tokens,
            "stream": true,
        });

        if let Some(tools) = request.tools.as_ref().filter(|t| !t.is_empty()) {
            let oai_tools: Vec<serde_json::Value> = tools.iter().map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters,
                    }
                })
            }).collect();
            body["tools"] = serde_json::to_value(oai_tools)?;
            body["tool_choice"] = json!(request.tool_choice.as_deref().unwrap_or("auto"));
        }

        let response = self.client
            .post(format!("{}/chat/completions", self.config.api_base))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("NIM API error {}: {}", status, text);
        }

        let (tx, rx) = tokio::sync::mpsc::channel(256);
        let response = response.bytes_stream();

        tokio::spawn(async move {
            use futures_util::StreamExt;
            let mut stream = response;
            let mut buffer = String::new();
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));
                        for line in buffer.lines() {
                            if let Some(data) = line.strip_prefix("data: ") {
                                if data == "[DONE]" { continue; }
                                if let Ok(sse) = serde_json::from_str::<SseChunk>(data) {
                                    for choice in sse.choices {
                                        if let Some(content) = choice.delta.content { let _ = tx.send(StreamEvent::Token(content)).await; }
                                    }
                                }
                            }
                        }
                        buffer.clear();
                    }
                    Err(e) => { let _ = tx.send(StreamEvent::Error(e.to_string())).await; }
                }
            }
        });

        Ok(ChatStream { provider: self.config.id.clone(), model: request.model, receiver: rx })
    }

    async fn health_check(&self) -> Result<bool> {
        if self.api_key().is_err() { return Ok(false); }
        Ok(true)
    }
}

impl NvidiaNimProvider {
    fn format_messages(&self, messages: &[Message]) -> Vec<serde_json::Value> {
        let mut out = Vec::new();
        for m in messages {
            match m.role {
                MessageRole::Tool => {
                    if let Some(ref results) = m.tool_results {
                        for result in results {
                            let content = if result.success { result.output.clone() } else { format!("Error: {}", result.error.as_deref().unwrap_or("unknown")) };
                            out.push(json!({
                                "role": "tool",
                                "tool_call_id": result.id.0.to_string(),
                                "content": trim_for_provider(&content, MAX_TOOL_RESULT_CHARS),
                            }));
                        }
                    } else if !m.content.trim().is_empty() {
                        out.push(json!({"role": "user", "content": trim_for_provider(&m.content, MAX_TOOL_RESULT_CHARS)}));
                    }
                }
                _ => {
                    let role = match m.role { MessageRole::System => "system", MessageRole::User => "user", MessageRole::Assistant => "assistant", MessageRole::Tool => unreachable!() };
                    let mut obj = serde_json::Map::new();
                    obj.insert("role".to_string(), json!(role));
                    obj.insert("content".to_string(), json!(trim_for_provider(&m.content, MAX_REQUEST_TEXT_CHARS)));
                    if let Some(ref tool_calls) = m.tool_calls {
                        let oai_calls: Vec<serde_json::Value> = tool_calls.iter().map(|tc| {
                            let args_str = tc.args.to_string();
                            json!({"id": tc.id.0.to_string(), "type": "function", "function": {"name": crate::provider::tool_kind_name(&tc.kind), "arguments": args_str}})
                        }).collect();
                        obj.insert("tool_calls".to_string(), json!(oai_calls));
                    }
                    out.push(Value::Object(obj));
                }
            }
        }
        out
    }
}

fn trim_for_provider(value: &str, limit: usize) -> String {
    if value.chars().count() <= limit { return value.to_string(); }
    let mut out = value.chars().take(limit).collect::<String>();
    out.push_str("\n[Forge truncated provider request content for NIM context safety]");
    out
}

fn normalize_nim_tool_name(name: &str) -> &str {
    if name == "todo_write" || name == "todo" { "task" } else { name }
}

fn tool_arguments_as_json_string(arguments: Option<&serde_json::Value>) -> String {
    match arguments {
        Some(serde_json::Value::String(value)) => value.clone(),
        Some(value) => value.to_string(),
        None => "{}".to_string(),
    }
}

#[derive(Debug, Deserialize)]
struct NimResponse { id: String, object: String, created: u64, model: String, choices: Vec<NimChoice>, usage: Option<NimUsage> }
#[derive(Debug, Deserialize)]
struct NimChoice { index: u32, message: NimMessage, finish_reason: Option<String> }
#[derive(Debug, Deserialize)]
struct NimMessage { role: String, content: Option<String>, tool_calls: Option<Vec<serde_json::Value>> }
#[derive(Debug, Deserialize)]
struct NimUsage { prompt_tokens: u32, completion_tokens: u32, total_tokens: u32 }
#[derive(Debug, Deserialize)]
struct SseChunk { choices: Vec<SseChoice> }
#[derive(Debug, Deserialize)]
struct SseChoice { delta: SseDelta }
#[derive(Debug, Deserialize)]
struct SseDelta { content: Option<String> }
