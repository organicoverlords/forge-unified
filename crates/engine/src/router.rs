//! FreeLLMAPI-style provider router with deterministic model fallback and per-model cooldowns.

use crate::config::Config;
use crate::provider::{ChatRequest, ChatResponse, ChatStream, Provider};
use crate::types::{ModelId, ProviderConfig, ProviderId};
use anyhow::Result;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Router {
    providers: HashMap<ProviderId, Box<dyn Provider>>,
    priority_order: Vec<ProviderId>,
    cooldowns: Mutex<HashMap<String, CooldownEntry>>,
}

#[derive(Debug, Clone)]
struct CooldownEntry {
    until: Instant,
    classification: FailureClass,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailureClass {
    BusyCapacity,
    RateLimited,
    QuotaCapped,
    Timeout,
    Server,
    Empty,
    MissingKey,
    Other,
}

#[derive(Debug, Clone, serde::Serialize)]
struct AttemptReceipt {
    provider: String,
    model: String,
    cooldown_scope: String,
    status: String,
    classification: Option<String>,
    error: Option<String>,
    cooldown_ms: u64,
}

impl Router {
    pub fn from_config(config: &Config) -> Self {
        let mut providers = HashMap::new();
        let enabled = config.provider_priority_order();
        let mut priority_order = Vec::new();

        if enabled.is_empty() {
            let nim_config = super::provider::default_nim_config();
            let nim_id = nim_config.id.clone();
            let provider = super::provider::create_provider(nim_config);
            providers.insert(nim_id.clone(), provider);
            priority_order.push(nim_id);
        } else {
            for pc in &enabled {
                let provider = super::provider::create_provider((*pc).clone());
                priority_order.push(pc.id.clone());
                providers.insert(pc.id.clone(), provider);
            }
        }

        Self { providers, priority_order, cooldowns: Mutex::new(HashMap::new()) }
    }

    pub fn with_default_nim() -> Self {
        let nim_config = super::provider::default_nim_config();
        let nim_id = nim_config.id.clone();
        let provider = super::provider::create_provider(nim_config);

        Self {
            providers: HashMap::from([(nim_id.clone(), provider)]),
            priority_order: vec![nim_id],
            cooldowns: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&mut self, config: ProviderConfig) {
        let id = config.id.clone();
        let provider = super::provider::create_provider(config);
        self.providers.insert(id.clone(), provider);
        if !self.priority_order.contains(&id) {
            self.priority_order.push(id);
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        self.expire_cooldowns();
        let candidates = self.candidates(&request.model);
        let mut attempts = Vec::new();
        let mut tried = 0usize;

        for (provider_id, model_id) in candidates {
            if tried >= 20 {
                break;
            }

            let key = route_key(&provider_id, &model_id);
            if let Some(entry) = self.cooldowns.lock().get(&key).cloned() {
                attempts.push(AttemptReceipt {
                    provider: provider_id.0.clone(),
                    model: model_id.0.clone(),
                    cooldown_scope: "model".to_string(),
                    status: "skipped_cooldown".to_string(),
                    classification: Some(class_name(entry.classification).to_string()),
                    error: Some(entry.message),
                    cooldown_ms: entry.until.saturating_duration_since(Instant::now()).as_millis() as u64,
                });
                continue;
            }

            let Some(provider) = self.providers.get(&provider_id) else { continue; };
            tried += 1;
            let mut routed = request.clone();
            routed.model = model_id.clone();

            match provider.chat(routed).await {
                Ok(mut response) => {
                    if response.message.content.trim().is_empty()
                        && response.message.tool_calls.as_ref().map(|v| v.is_empty()).unwrap_or(true)
                    {
                        let classification = FailureClass::Empty;
                        self.put_cooldown(&provider_id, &model_id, classification, "empty response".to_string());
                        attempts.push(AttemptReceipt {
                            provider: provider_id.0,
                            model: model_id.0,
                            cooldown_scope: "model".to_string(),
                            status: "failed".to_string(),
                            classification: Some(class_name(classification).to_string()),
                            error: Some("empty response".to_string()),
                            cooldown_ms: cooldown_for(classification).as_millis() as u64,
                        });
                        continue;
                    }

                    attempts.push(AttemptReceipt {
                        provider: provider_id.0.clone(),
                        model: model_id.0.clone(),
                        cooldown_scope: "none".to_string(),
                        status: "selected".to_string(),
                        classification: None,
                        error: None,
                        cooldown_ms: 0,
                    });
                    response.message.metadata.insert("routing_receipt".to_string(), serde_json::json!({
                        "strategy": "freellmapi_priority_cooldown",
                        "cooldown_scope": "provider_model",
                        "exhausted_means": "model_busy_capacity_not_provider_quota",
                        "selected_provider": response.provider.0,
                        "selected_model": response.model.0,
                        "attempts": attempts,
                    }));
                    return Ok(response);
                }
                Err(error) => {
                    let message = error.to_string();
                    let classification = classify_failure(&message);
                    self.put_cooldown(&provider_id, &model_id, classification, message.clone());
                    attempts.push(AttemptReceipt {
                        provider: provider_id.0,
                        model: model_id.0,
                        cooldown_scope: "model".to_string(),
                        status: "failed".to_string(),
                        classification: Some(class_name(classification).to_string()),
                        error: Some(message),
                        cooldown_ms: cooldown_for(classification).as_millis() as u64,
                    });
                }
            }
        }

        anyhow::bail!("All providers/models failed: {}", serde_json::to_string(&attempts).unwrap_or_default())
    }

    pub async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream> {
        self.expire_cooldowns();
        for (provider_id, model_id) in self.candidates(&request.model) {
            let key = route_key(&provider_id, &model_id);
            if self.cooldowns.lock().contains_key(&key) {
                continue;
            }
            if let Some(provider) = self.providers.get(&provider_id) {
                let mut routed = request.clone();
                routed.model = model_id.clone();
                match provider.chat_stream(routed).await {
                    Ok(stream) => return Ok(stream),
                    Err(error) => {
                        let classification = classify_failure(&error.to_string());
                        self.put_cooldown(&provider_id, &model_id, classification, error.to_string());
                    }
                }
            }
        }
        anyhow::bail!("All providers failed for streaming model {}", request.model.0)
    }

    pub async fn health_check_all(&self) -> HashMap<ProviderId, bool> {
        let mut results = HashMap::new();
        for (id, provider) in &self.providers {
            results.insert(id.clone(), provider.health_check().await.unwrap_or(false));
        }
        results
    }

    pub fn provider(&self, id: &ProviderId) -> Option<&dyn Provider> {
        self.providers.get(id).map(|p| p.as_ref())
    }

    pub fn available_models(&self) -> Vec<(ProviderId, ModelId)> {
        self.providers.values()
            .flat_map(|p| {
                p.config().models.iter().map(|m| (p.id().clone(), m.id.clone())).collect::<Vec<_>>()
            })
            .collect()
    }

    fn candidates(&self, preferred: &ModelId) -> Vec<(ProviderId, ModelId)> {
        let mut out = Vec::new();
        let mut push_unique = |provider_id: ProviderId, model_id: ModelId| {
            if !out.iter().any(|(p, m)| p == &provider_id && m == &model_id) {
                out.push((provider_id, model_id));
            }
        };

        for provider_id in &self.priority_order {
            if let Some(provider) = self.providers.get(provider_id) {
                if provider.config().models.iter().any(|m| &m.id == preferred) {
                    push_unique(provider_id.clone(), preferred.clone());
                }
            }
        }

        for provider_id in &self.priority_order {
            if let Some(provider) = self.providers.get(provider_id) {
                for model in &provider.config().models {
                    push_unique(provider_id.clone(), model.id.clone());
                }
            }
        }

        out
    }

    fn put_cooldown(&self, provider_id: &ProviderId, model_id: &ModelId, classification: FailureClass, message: String) {
        self.cooldowns.lock().insert(route_key(provider_id, model_id), CooldownEntry {
            until: Instant::now() + cooldown_for(classification),
            classification,
            message,
        });
    }

    fn expire_cooldowns(&self) {
        let now = Instant::now();
        self.cooldowns.lock().retain(|_, entry| entry.until > now);
    }
}

fn route_key(provider_id: &ProviderId, model_id: &ModelId) -> String {
    format!("{}:{}", provider_id.0, model_id.0)
}

fn classify_failure(message: &str) -> FailureClass {
    let lower = message.to_ascii_lowercase();

    if lower.contains("exhausted")
        || lower.contains("model is busy")
        || lower.contains("engine is busy")
        || lower.contains("capacity")
        || lower.contains("temporarily unavailable")
        || lower.contains("overloaded")
    {
        FailureClass::BusyCapacity
    } else if lower.contains("quota exceeded")
        || lower.contains("hard limit")
        || lower.contains("billing")
        || lower.contains("insufficient credits")
        || lower.contains("monthly limit")
        || lower.contains("daily limit")
    {
        FailureClass::QuotaCapped
    } else if lower.contains("429") || lower.contains("rate limit") || lower.contains("too many") {
        FailureClass::RateLimited
    } else if lower.contains("timeout") || lower.contains("timed out") || lower.contains("deadline") {
        FailureClass::Timeout
    } else if lower.contains("500") || lower.contains("502") || lower.contains("503") || lower.contains("504") {
        FailureClass::Server
    } else if lower.contains("api key") || lower.contains("missing") || lower.contains("unauthorized") || lower.contains("401") {
        FailureClass::MissingKey
    } else {
        FailureClass::Other
    }
}

fn cooldown_for(classification: FailureClass) -> Duration {
    match classification {
        FailureClass::BusyCapacity => Duration::from_secs(12),
        FailureClass::RateLimited => Duration::from_secs(45),
        FailureClass::QuotaCapped => Duration::from_secs(600),
        FailureClass::Timeout => Duration::from_secs(30),
        FailureClass::Server => Duration::from_secs(45),
        FailureClass::Empty => Duration::from_secs(20),
        FailureClass::MissingKey => Duration::from_secs(300),
        FailureClass::Other => Duration::from_secs(20),
    }
}

fn class_name(classification: FailureClass) -> &'static str {
    match classification {
        FailureClass::BusyCapacity => "model_busy_capacity",
        FailureClass::RateLimited => "rate_limited",
        FailureClass::QuotaCapped => "quota_capped",
        FailureClass::Timeout => "timeout",
        FailureClass::Server => "server_error",
        FailureClass::Empty => "empty_response",
        FailureClass::MissingKey => "missing_key",
        FailureClass::Other => "other",
    }
}
