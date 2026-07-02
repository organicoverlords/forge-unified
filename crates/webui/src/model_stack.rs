//! FreeLLMAPI-style fallback stack endpoints.

use axum::Json;
use forge_engine::config::Config;
use forge_engine::types::{ModelConfig, ModelId, ProviderConfig, ProviderId};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackEntry {
    pub provider: String,
    pub model: String,
    pub display_name: String,
    pub enabled: bool,
    pub priority: u32,
    pub context_window: u32,
    pub supports_tools: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackResponse {
    pub strategy: String,
    pub entries: Vec<StackEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SaveStackRequest {
    pub entries: Vec<SaveStackEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SaveStackEntry {
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
struct ModelsResponse { data: Vec<ModelRow> }
#[derive(Debug, Deserialize)]
struct ModelRow { id: String }

pub async fn get_stack() -> Json<StackResponse> {
    let config = Config::load().unwrap_or_default();
    Json(stack_response(&config.providers))
}

pub async fn save_stack(Json(req): Json<SaveStackRequest>) -> Result<Json<StackResponse>, axum::http::StatusCode> {
    let mut config = Config::load().unwrap_or_default();

    if let Some(first) = req.entries.first() {
        config.default_provider = Some(ProviderId(first.provider.clone()));
        config.default_model = Some(ModelId(first.model.clone()));
    }

    for provider in config.providers.iter_mut() {
        provider.models.sort_by_key(|model| {
            req.entries.iter().position(|entry| entry.provider == provider.id.0 && entry.model == model.id.0).unwrap_or(usize::MAX)
        });
        if req.entries.iter().any(|entry| entry.provider == provider.id.0) {
            provider.priority = req.entries.iter().position(|entry| entry.provider == provider.id.0).unwrap_or(usize::MAX) as u32;
            provider.enabled = true;
        }
    }

    config.save().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(stack_response(&config.providers)))
}

pub async fn discover_stack() -> Json<StackResponse> {
    let mut config = Config::load().unwrap_or_default();
    let Ok(client) = Client::builder().timeout(Duration::from_secs(20)).build() else {
        return Json(stack_response(&config.providers));
    };

    for provider in config.providers.iter_mut() {
        let Some(token) = provider_token(provider) else { continue; };
        let url = format!("{}/models", provider.api_base.trim_end_matches('/'));
        let Ok(response) = client.get(url).bearer_auth(token).send().await else { continue; };
        if !response.status().is_success() { continue; }
        let Ok(parsed) = response.json::<ModelsResponse>().await else { continue; };

        for row in parsed.data {
            if !provider.models.iter().any(|model| model.id.0 == row.id) {
                provider.models.push(model_from_id(&row.id));
            }
        }
        provider.enabled = true;
    }

    let _ = config.save();
    Json(stack_response(&config.providers))
}

fn stack_response(providers: &[ProviderConfig]) -> StackResponse {
    let mut entries = Vec::new();
    let mut sorted = providers.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|provider| provider.priority);
    for provider in sorted {
        for (index, model) in provider.models.iter().enumerate() {
            entries.push(StackEntry {
                provider: provider.id.0.clone(),
                model: model.id.0.clone(),
                display_name: model.name.clone(),
                enabled: provider.enabled,
                priority: provider.priority.saturating_mul(1000) + index as u32,
                context_window: model.context_window,
                supports_tools: model.supports_tools,
            });
        }
    }
    StackResponse { strategy: "freellmapi_priority_cooldown".to_string(), entries }
}

fn provider_token(provider: &ProviderConfig) -> Option<String> {
    let direct = std::env::var(&provider.api_key_env).ok();
    let nim = if provider.id.0 == "nvidia_nim" { std::env::var("NIM_KEY").ok().or_else(|| std::env::var("NVIDIA_NIM_API_KEY").ok()) } else { None };
    direct.or(nim).filter(|value| !value.trim().is_empty())
}

fn model_from_id(id: &str) -> ModelConfig {
    ModelConfig {
        id: ModelId(id.to_string()),
        name: id.to_string(),
        context_window: if id.to_ascii_lowercase().contains("80k") { 80_000 } else { 128_000 },
        supports_streaming: true,
        supports_tools: true,
        supports_parallel_tools: true,
        max_output_tokens: 8192,
    }
}
