//! LLM provider router with priority-based fallback.

use crate::config::Config;
use crate::provider::{ChatRequest, ChatResponse, ChatStream, Provider};
use crate::types::{ProviderId, ModelId, ProviderConfig};
use anyhow::Result;
use std::collections::HashMap;

pub struct Router {
    providers: HashMap<ProviderId, Box<dyn Provider>>,
    priority_order: Vec<ProviderId>,
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

        Self { providers, priority_order }
    }

    pub fn with_default_nim() -> Self {
        let nim_config = super::provider::default_nim_config();
        let nim_id = nim_config.id.clone();
        let provider = super::provider::create_provider(nim_config);
        
        Self {
            providers: HashMap::from([(nim_id.clone(), provider)]),
            priority_order: vec![nim_id],
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
        let model_provider = request.model.0.split('/').next().map(|s| ProviderId(s.to_string()));
        
        if let Some(id) = &model_provider {
            if let Some(provider) = self.providers.get(id) {
                if let Ok(resp) = provider.chat(request.clone()).await {
                    return Ok(resp);
                }
            }
        }

        for pid in &self.priority_order {
            if Some(pid) == model_provider.as_ref() {
                continue;
            }
            if let Some(provider) = self.providers.get(pid) {
                if let Ok(resp) = provider.chat(request.clone()).await {
                    return Ok(resp);
                }
            }
        }

        anyhow::bail!("All providers failed for model {}", request.model.0)
    }

    pub async fn chat_stream(&self, request: ChatRequest) -> Result<ChatStream> {
        for provider_id in &self.priority_order {
            if let Some(provider) = self.providers.get(provider_id) {
                if let Ok(stream) = provider.chat_stream(request.clone()).await {
                    return Ok(stream);
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
}
