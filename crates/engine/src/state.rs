//! Engine state — shared mutable state for the running engine.

use crate::config::Config;
use crate::types::{AgentMode, ConversationId, ModelId, ProviderId, RunId};
use std::collections::{HashMap, HashSet};

pub struct EngineState {
    pub default_provider: ProviderId,
    pub default_model: ModelId,
    pub mode: AgentMode,
    pub paused: HashSet<ConversationId>,
    pub cancelled: HashSet<ConversationId>,
    pub active_runs: HashMap<ConversationId, RunId>,
}

impl EngineState {
    pub fn new(config: &Config) -> Self {
        let first_enabled_provider = config.provider_priority_order().into_iter().next();

        let default_provider = config
            .default_provider
            .clone()
            .or_else(|| first_enabled_provider.map(|provider| provider.id.clone()))
            .unwrap_or_else(|| ProviderId("nvidia_nim".to_string()));

        let first_enabled_model = config
            .provider_priority_order()
            .into_iter()
            .find_map(|provider| provider.models.first().map(|model| model.id.clone()));

        let default_model = config
            .default_model
            .clone()
            .or(first_enabled_model)
            .unwrap_or_else(|| ModelId("mistralai/mistral-small-4-119b-2603".to_string()));

        Self {
            default_provider,
            default_model,
            mode: AgentMode::Chat,
            paused: HashSet::new(),
            cancelled: HashSet::new(),
            active_runs: HashMap::new(),
        }
    }

    pub fn current_provider(&self) -> ProviderId {
        self.default_provider.clone()
    }

    pub fn current_model(&self) -> ModelId {
        self.default_model.clone()
    }

    pub fn set_provider(&mut self, provider: ProviderId) {
        self.default_provider = provider;
    }

    pub fn set_model(&mut self, model: ModelId) {
        self.default_model = model;
    }

    pub fn set_mode(&mut self, mode: AgentMode) {
        self.mode = mode;
    }

    pub fn is_paused(&self, id: &ConversationId) -> bool {
        self.paused.contains(id)
    }

    pub fn is_cancelled(&self, id: &ConversationId) -> bool {
        self.cancelled.contains(id)
    }

    pub fn pause_run(&mut self, id: &ConversationId) {
        self.paused.insert(id.clone());
    }

    pub fn resume_run(&mut self, id: &ConversationId) {
        self.paused.remove(id);
    }

    pub fn cancel_run(&mut self, id: &ConversationId) {
        self.cancelled.insert(id.clone());
    }

    pub fn start_run(&mut self, id: &ConversationId, run_id: RunId) {
        self.active_runs.insert(id.clone(), run_id);
        self.paused.remove(id);
        self.cancelled.remove(id);
    }

    pub fn end_run(&mut self, id: &ConversationId) {
        self.active_runs.remove(id);
        self.paused.remove(id);
        self.cancelled.remove(id);
    }
}
