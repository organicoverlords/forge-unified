//! Model capability registry and tool strategy controller.
//! Based on ForgeStack's Tool Strategy Controller with no-keyword routing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::{ProviderId, ModelId, ToolKind};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelCapabilityMatrix {
    pub capabilities: HashMap<(ProviderId, ModelId), ModelCapabilities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub provider_id: ProviderId,
    pub model_id: ModelId,
    pub native_parallel_tool_calls: bool,
    pub function_calling: bool,
    pub supports_strict_json: bool,
    pub supports_forced_tool_choice: bool,
    pub synthetic_batch_supported: bool,
    pub max_batch_size: u32,
    pub max_concurrency: u32,
    pub context_window: u32,
    pub max_output_tokens: u32,
    pub notes: Option<String>,
}

impl ModelCapabilityMatrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nim_catalog() -> Self {
        let mut caps = Self::new();
        
        // NVIDIA NIM models
        let nim_models = vec![
            ("nvidia_nim", "openai/gpt-oss-120b", false, 100, 10),
            ("nvidia_nim", "meta/llama-3.1-405b-instruct", false, 50, 8),
            ("nvidia_nim", "meta/llama-3.1-70b-instruct", true, 50, 8),
            ("nvidia_nim", "meta/llama-3.1-8b-instruct", true, 20, 8),
            ("nvidia_nim", "microsoft/phi-3.5-mini-instruct", true, 20, 8),
            ("nvidia_nim", "google/gemma-2-9b-it", true, 20, 8),
            ("nvidia_nim", "mistralai/mistral-nemo-12b-instruct", true, 20, 8),
            ("nvidia_nim", "nvidia/nemotron-3-ultra", true, 20, 8),
        ];

        for (provider, model, native_parallel, batch_size, concurrency) in nim_models {
            caps.capabilities.insert(
                (ProviderId(provider.to_string()), ModelId(model.to_string())),
                ModelCapabilities {
                    provider_id: ProviderId(provider.to_string()),
                    model_id: ModelId(model.to_string()),
                    native_parallel_tool_calls: native_parallel,
                    function_calling: true,
                    supports_strict_json: true,
                    supports_forced_tool_choice: true,
                    synthetic_batch_supported: true,
                    max_batch_size: batch_size,
                    max_concurrency: concurrency,
                    context_window: 128000,
                    max_output_tokens: 8192,
                    notes: Some("NVIDIA NIM".to_string()),
                },
            );
        }

        // Local models
        caps.capabilities.insert(
            (ProviderId("local".to_string()), ModelId("local".to_string())),
            ModelCapabilities {
                provider_id: ProviderId("local".to_string()),
                model_id: ModelId("local".to_string()),
                native_parallel_tool_calls: false,
                function_calling: false,
                supports_strict_json: false,
                supports_forced_tool_choice: false,
                synthetic_batch_supported: true,
                max_batch_size: 10,
                max_concurrency: 4,
                context_window: 32768,
                max_output_tokens: 4096,
                notes: Some("Local fallback".to_string()),
            },
        );

        caps
    }

    pub fn get(&self, provider: &ProviderId, model: &ModelId) -> Option<&ModelCapabilities> {
        self.capabilities.get(&(provider.clone(), model.clone()))
    }

    pub fn get_or_default(&self, provider: &ProviderId, model: &ModelId) -> ModelCapabilities {
        self.get(provider, model).cloned().unwrap_or_else(|| ModelCapabilities {
            provider_id: provider.clone(),
            model_id: model.clone(),
            native_parallel_tool_calls: false,
            function_calling: true,
            supports_strict_json: false,
            supports_forced_tool_choice: false,
            synthetic_batch_supported: true,
            max_batch_size: 10,
            max_concurrency: 4,
            context_window: 32768,
            max_output_tokens: 4096,
            notes: None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolStrategy {
    Auto,
    Serial,
    Batch,
    BatchFromPlan,
}

#[derive(Debug, Clone)]
pub struct StrategyDecision {
    pub strategy: ToolStrategy,
    pub reason: String,
    pub parallel_groups: Vec<Vec<ToolKind>>,
}

pub struct ToolStrategyController {
    caps: ModelCapabilityMatrix,
}

impl ToolStrategyController {
    pub fn new(caps: ModelCapabilityMatrix) -> Self {
        Self { caps }
    }

    pub fn decide(&self, provider: &ProviderId, model: &ModelId, tools: &[ToolKind]) -> StrategyDecision {
        let caps = self.caps.get_or_default(provider, model);
        
        // Check for independent parallelizable tools
        let parallelizable: Vec<_> = tools.iter()
            .filter(|t| is_parallelizable(*t))
            .cloned()
            .collect();

        if parallelizable.len() >= 2 {
            // Group by type for batching
            let mut groups: HashMap<ToolKind, Vec<ToolKind>> = HashMap::new();
            for tool in &parallelizable {
                groups.entry(tool.clone()).or_default().push(tool.clone());
            }
            
            let parallel_groups: Vec<_> = groups.into_values().collect();
            
            if caps.native_parallel_tool_calls {
                StrategyDecision {
                    strategy: ToolStrategy::Batch,
                    reason: format!("Model supports native parallel tools; {} independent tools detected", parallelizable.len()),
                    parallel_groups,
                }
            } else if caps.synthetic_batch_supported {
                StrategyDecision {
                    strategy: ToolStrategy::Batch,
                    reason: format!("Using synthetic batch tool; {} independent tools detected", parallelizable.len()),
                    parallel_groups,
                }
            } else {
                StrategyDecision {
                    strategy: ToolStrategy::Serial,
                    reason: "Model lacks parallel/batch support".to_string(),
                    parallel_groups: vec![],
                }
            }
        } else {
            StrategyDecision {
                strategy: ToolStrategy::Serial,
                reason: "Insufficient independent parallelizable tools".to_string(),
                parallel_groups: vec![],
            }
        }
    }
}

fn is_parallelizable(kind: &ToolKind) -> bool {
    matches!(
        kind,
        ToolKind::FileRead
        | ToolKind::FileList
        | ToolKind::FileGlob
        | ToolKind::FileSearch
        | ToolKind::WebFetch
        | ToolKind::WebSearch
        | ToolKind::RepoInfo
    )
}

pub fn batch_tools(tools: &[ToolKind], max_batch_size: u32) -> Vec<Vec<ToolKind>> {
    let mut batches = Vec::new();
    let mut current_batch = Vec::new();
    
    for tool in tools {
        if is_parallelizable(tool) {
            current_batch.push(tool.clone());
            if current_batch.len() >= max_batch_size as usize {
                batches.push(current_batch);
                current_batch = Vec::new();
            }
        } else {
            if !current_batch.is_empty() {
                batches.push(current_batch);
                current_batch = Vec::new();
            }
            batches.push(vec![tool.clone()]);
        }
    }
    
    if !current_batch.is_empty() {
        batches.push(current_batch);
    }
    
    batches
}