//! NVIDIA NIM provider implementation.

#![allow(dead_code)]

use crate::provider::{ChatRequest, ChatResponse, ChatStream, Provider, StreamEvent, TokenUsage, ToolCallDelta};
use crate::types::{ProviderConfig, ProviderId, Message, MessageRole};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

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
        std::env::var(&self.config.api_key_env)
            .with_context(|| format!("Missing {} env var", self.config.api_key_env))
    }
}

#[async_trait]
impl Provider for NvidiaNimProvider {
    fn id(&self) -> &ProviderId {
        &self.config.id
    }

    fn config(&self) -> &ProviderConfig {
        &self.config
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let api_key = self.api_key()?;
        
        let mut body = json!({
            "model": request.model.0,
            "messages": self.format_messages(&request.messages),
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(8192),
            "stream": false,
        });
        
        if let Some(tools) = request.tools.as_ref().filter(|t| !t.is_empty()) {
            body["tools"] = serde_json::to_value(tools)?;
            body["tool_choice"] = json!(request.tool_choice.as_deref().unwrap_or("auto"));
        }
        
        let response = self.client
            .post(format!("{}/chat/completions", self.config.api_base))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;
        
        let status = response.status();
        let text = response.text().await?;
        
        if !status.is_success() {
            anyhow::bail!("NIM API error {}: {}", status, text);
        }
        
        let nims_response: NimResponse = serde_json::from_str(&text)?;
        let choice = nims_response.choices.into_iter().next()
            .context("No response from model")?;
        
        let tool_requests = choice.message.tool_calls.map(|tc| {
            let deltas: Vec<ToolCallDelta> = tc.into_iter().filter_map(|v| {
                let id = v.get("id").and_then(|i| i.as_str()).unwrap_or("").to_string();
                let name = v.get("function").and_then(|f| f.get("name")).and_then(|n| n.as_str()).unwrap_or("").to_string();
                let arguments = v.get("function").and_then(|f| f.get("arguments")).and_then(|a| a.as_str()).unwrap_or("{}").to_string();
                if name.is_empty() { None } else {
                    Some(ToolCallDelta { id, name, arguments })
                }
            }).collect();
            crate::provider::tool_calls_from_deltas(deltas)
        });
        
        Ok(ChatResponse {
            message: Message {
                role: MessageRole::Assistant,
                content: choice.message.content.unwrap_or_default(),
                tool_calls: tool_requests,
                tool_results: None,
                metadata: Default::default(),
            },
            usage: nims_response.usage.map(|u| TokenUsage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
            provider: self.config.id.clone(),
            model: request.model,
        })
    }

    async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream> {
        let api_key = self.api_key()?;
        
        let body = json!({
            "model": request.model.0,
            "messages": self.format_messages(&request.messages),
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(8192),
            "stream": true,
        });
        
        let response = self.client
            .post(format!("{}/chat/completions", self.config.api_base))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;
        
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
                                        if let Some(content) = choice.delta.content {
                                            let _ = tx.send(StreamEvent::Token(content)).await;
                                        }
                                    }
                                }
                            }
                        }
                        buffer.clear();
                    }
                    Err(e) => {
                        let _ = tx.send(StreamEvent::Error(e.to_string())).await;
                    }
                }
            }
        });
        
        Ok(ChatStream {
            provider: self.config.id.clone(),
            model: request.model,
            receiver: rx,
        })
    }

    async fn health_check(&self) -> Result<bool> {
        if self.api_key().is_err() {
            return Ok(false);
        }
        Ok(true)
    }
}

impl NvidiaNimProvider {
    fn format_messages(&self, messages: &[Message]) -> Vec<serde_json::Value> {
        messages.iter().map(|m| {
            let role = match m.role {
                MessageRole::System => "system",
                MessageRole::User => "user",
                MessageRole::Assistant => "assistant",
                MessageRole::Tool => "tool",
            };
            json!({
                "role": role,
                "content": m.content,
            })
        }).collect()
    }
}

#[derive(Debug, Deserialize)]
struct NimResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<NimChoice>,
    usage: Option<NimUsage>,
}

#[derive(Debug, Deserialize)]
struct NimChoice {
    index: u32,
    message: NimMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NimMessage {
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct NimUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct SseChunk {
    choices: Vec<SseChoice>,
}

#[derive(Debug, Deserialize)]
struct SseChoice {
    delta: SseDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SseDelta {
    content: Option<String>,
    tool_calls: Option<Vec<serde_json::Value>>,
    role: Option<String>,
}