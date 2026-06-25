//! Tool strategy controller with no-keyword routing.
//! Wraps model_caps::ToolStrategyController for engine-level use.

use crate::model_caps::{ModelCapabilityMatrix, ToolStrategyController, ToolStrategy, StrategyDecision};
use crate::types::{ProviderId, ModelId, ToolKind, ToolRequest};
use std::collections::HashMap;

pub struct StrategyEngine {
    controller: ToolStrategyController,
    overrides: HashMap<(ProviderId, ModelId), ToolStrategy>,
}

impl StrategyEngine {
    pub fn new(caps: ModelCapabilityMatrix) -> Self {
        Self {
            controller: ToolStrategyController::new(caps),
            overrides: HashMap::new(),
        }
    }

    pub fn with_nim_catalog() -> Self {
        Self::new(ModelCapabilityMatrix::with_nim_catalog())
    }

    pub fn decide(
        &self,
        provider: &ProviderId,
        model: &ModelId,
        tools: &[ToolKind],
    ) -> StrategyDecision {
        if let Some(override_strategy) = self.overrides.get(&(provider.clone(), model.clone())) {
            return StrategyDecision {
                strategy: *override_strategy,
                reason: "User-configured override".to_string(),
                parallel_groups: vec![],
            };
        }
        self.controller.decide(provider, model, tools)
    }

    pub fn decide_for_requests(
        &self,
        provider: &ProviderId,
        model: &ModelId,
        requests: &[ToolRequest],
    ) -> StrategyDecision {
        let kinds: Vec<ToolKind> = requests.iter().map(|r| r.kind.clone()).collect();
        self.decide(provider, model, &kinds)
    }

    pub fn set_override(&mut self, provider: ProviderId, model: ModelId, strategy: ToolStrategy) {
        self.overrides.insert((provider, model), strategy);
    }

    pub fn clear_override(&mut self, provider: &ProviderId, model: &ModelId) {
        self.overrides.remove(&(provider.clone(), model.clone()));
    }

    pub fn batch_plan(
        &self,
        _provider: &ProviderId,
        _model: &ModelId,
        tools: &[ToolKind],
        max_batch_size: u32,
    ) -> Vec<Vec<ToolKind>> {
        crate::model_caps::batch_tools(tools, max_batch_size)
    }
}
