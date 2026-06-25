//! Shared application state for the webui.

use forge_engine::Agent;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub agent: Arc<Agent>,
}

impl AppState {
    pub fn new(agent: Agent) -> Self {
        Self {
            agent: Arc::new(agent),
        }
    }
}
