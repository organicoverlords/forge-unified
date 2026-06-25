//! Generic OpenAI-compatible provider.

#![allow(dead_code)]

use crate::provider::{ChatRequest, ChatResponse, ChatStream, Provider, StreamEvent, TokenUsage, ToolCallDelta};
use crate::types::{ProviderConfig, ProviderId, Message, MessageRole};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

pub struct OpenAiProvider {
    config: ProviderConfig,
    client: Client,
}

impl OpenAiProvider {
    pub fn new(config: ProviderConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .expect("Failed to create HTTP client");
        Self { config, client }
    }
}

#[async_trait]
impl Provider for OpenAiProvider {
    fn id(&self) -> &ProviderId {
        &self.config.id
    }

    fn config(&self) -> &ProviderConfig {
        &self.config
    }

    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let api_key = self.resolve_api_key()?;
        
        let mut body = json!({
            "model": request.model.0,
            "messages": self.format_messages(&request.messages),
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(4096),
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
            .await
            .with_context(|| format!("OpenAI API call failed for model {}", request.model.0))?;
        
        let status = response.status();
        let text = response.text().await?;
        
        if !status.is_success() {
            anyhow::bail!("OpenAI API error {}: {}", status, text);
        }
        
        let oai_response: OpenAIResponse = serde_json::from_str(&text)?;
        let choice = oai_response.choices.into_iter().next()
            .context("No response from model")?;
        
        let content = choice.message.content.unwrap_or_default();
        let tool_requests = choice.message.tool_calls.map(|tc| {
            let deltas: Vec<ToolCallDelta> = tc.into_iter().map(|t| ToolCallDelta {
                id: t.id,
                name: t.function.name,
                arguments: t.function.arguments,
            }).collect();
            crate::provider::tool_calls_from_deltas(deltas)
        });
        
        Ok(ChatResponse {
            message: Message {
                role: MessageRole::Assistant,
                content,
                tool_calls: tool_requests,
                tool_results: None,
                metadata: Default::default(),
            },
            usage: oai_response.usage.map(|u| TokenUsage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
            provider: self.config.id.clone(),
            model: request.model,
        })
    }

    async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream> {
        let api_key = self.resolve_api_key()?;
        
        let mut body = json!({
            "model": request.model.0,
            "messages": self.format_messages(&request.messages),
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens.unwrap_or(4096),
            "stream": true,
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
                                if let Ok(sse) = serde_json::from_str::<SSEChunk>(data) {
                                    for choice in sse.choices {
                                        if let Some(content) = choice.delta.content {
                                            let _ = tx.send(StreamEvent::Token(content)).await;
                                        }
                                        if let Some(tcs) = choice.delta.tool_calls {
                                            for tc in tcs {
                                                let _ = tx.send(StreamEvent::ToolCall(ToolCallDelta {
                                                    id: tc.id.unwrap_or_default(),
                                                    name: tc.function.name.clone().unwrap_or_default(),
                                                    arguments: tc.function.arguments.clone().unwrap_or_default(),
                                                })).await;
                                            }
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
        Ok(self.resolve_api_key().is_ok())
    }
}

impl OpenAiProvider {
    fn resolve_api_key(&self) -> Result<String> {
        if self.config.api_key_env.is_empty() {
            return Ok(String::new());
        }
        std::env::var(&self.config.api_key_env)
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .with_context(|| format!("No API key found for provider {}", self.config.id.0))
    }

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
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    index: u32,
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: Option<String>,
    function: OpenAIFunction,
}

#[derive(Debug, Deserialize)]
struct OpenAIFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct SSEChunk {
    choices: Vec<SSEChoice>,
}

#[derive(Debug, Deserialize)]
struct SSEChoice {
    delta: SSEDelta,
    finish_reason: Option<String>,
    index: u32,
}

#[derive(Debug, Deserialize)]
struct SSEDelta {
    content: Option<String>,
    tool_calls: Option<Vec<SSEToolCall>>,
    role: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SSEToolCall {
    id: Option<String>,
    function: SSEToolCallFunction,
}

#[derive(Debug, Deserialize)]
struct SSEToolCallFunction {
    name: Option<String>,
    arguments: Option<String>,
}