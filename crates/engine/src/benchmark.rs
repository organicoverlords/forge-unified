//! Benchmark adapter — capability report for the engine.

use crate::types::BenchmarkCapabilities;
use crate::config::Config;

pub struct BenchmarkAdapter {
    capabilities: BenchmarkCapabilities,
}

impl BenchmarkAdapter {
    pub fn from_config(config: &Config) -> Self {
        let capabilities = BenchmarkCapabilities {
            chat: true,
            natural_tasks: true,
            repo_read: true,
            repo_write: true,
            shell: true,
            tool_calls: true,
            native_tool_traces: false,
            parallel_tool_calls: true,
            browser_self_report: false,
            browser_artifact_proof: false,
            screenshot: false,
            dom_snapshot: false,
            console_logs: false,
            network_logs: false,
            cancel: true,
            pause: true,
            resume: true,
            context_compaction: config.auto_compact,
            session_export: true,
            repo_root_inventory: true,
        };
        Self { capabilities }
    }

    pub fn capabilities(&self) -> &BenchmarkCapabilities {
        &self.capabilities
    }

    pub fn report(&self) -> Vec<(&'static str, bool)> {
        let c = &self.capabilities;
        vec![
            ("chat", c.chat),
            ("natural_tasks", c.natural_tasks),
            ("repo_read", c.repo_read),
            ("repo_write", c.repo_write),
            ("shell", c.shell),
            ("tool_calls", c.tool_calls),
            ("native_tool_traces", c.native_tool_traces),
            ("parallel_tool_calls", c.parallel_tool_calls),
            ("cancel", c.cancel),
            ("pause", c.pause),
            ("resume", c.resume),
            ("context_compaction", c.context_compaction),
            ("session_export", c.session_export),
            ("repo_root_inventory", c.repo_root_inventory),
        ]
    }

    pub fn score(&self) -> u32 {
        self.report().iter().filter(|(_, v)| *v).count() as u32
    }
}
