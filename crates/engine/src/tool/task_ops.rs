//! Task, repo info, propose patch, and switch mode tools.

use crate::tool::ToolExecutor;
use crate::types::{ToolRequest, ToolResult, ToolKind};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

impl ToolExecutor {
    pub async fn execute_task(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        #[allow(dead_code)]
        struct Args { description: String, background: Option<bool>, tools: Option<Vec<String>> }
        let args: Args = serde_json::from_value(request.args)?;
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::Task,
            success: true,
            output: format!("Subagent task created: {}", args.description),
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("description".to_string(), serde_json::json!(args.description)),
                ("background".to_string(), serde_json::json!(args.background.unwrap_or(false))),
                ("task_id".to_string(), serde_json::json!(Uuid::new_v4().to_string())),
            ]),
        })
    }

    pub async fn execute_repo_info(&self, request: ToolRequest) -> Result<ToolResult> {
        let mut info = HashMap::new();
        
        // Get git repo info
        if let Ok(output) = std::process::Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(&self.workspace_root)
            .output()
        {
            if output.status.success() {
                let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
                info.insert("root".to_string(), serde_json::json!(root));
            }
        }
        
        if let Ok(output) = std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(&self.workspace_root)
            .output()
        {
            if output.status.success() {
                let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                info.insert("branch".to_string(), serde_json::json!(branch));
            }
        }
        
        if let Ok(output) = std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .current_dir(&self.workspace_root)
            .output()
        {
            if output.status.success() {
                let head = String::from_utf8_lossy(&output.stdout).trim().to_string();
                info.insert("head".to_string(), serde_json::json!(head));
            }
        }
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::RepoInfo,
            success: true,
            output: serde_json::to_string_pretty(&info)?,
            error: None,
            duration_ms: 0,
            metadata: info,
        })
    }

    pub async fn execute_propose_patch(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        #[allow(dead_code)]
        struct Args { summary: String, diff: String, run_id: Option<String> }
        let args: Args = serde_json::from_value(request.args)?;
        
        if args.summary.is_empty() {
            anyhow::bail!("Patch summary cannot be empty");
        }
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::ProposePatch,
            success: true,
            output: format!("Patch proposed: {}", args.summary),
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("summary".to_string(), serde_json::json!(args.summary)),
                ("diff_length".to_string(), serde_json::json!(args.diff.len())),
            ]),
        })
    }

    pub async fn execute_switch_mode(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { mode: String }
        let args: Args = serde_json::from_value(request.args)?;
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::SwitchMode,
            success: true,
            output: format!("Switched to mode: {}", args.mode),
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([("mode".to_string(), serde_json::json!(args.mode))]),
        })
    }
}