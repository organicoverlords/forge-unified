//! Shell command execution tools.

use crate::tool::ToolExecutor;
use crate::types::{ToolRequest, ToolResult, ToolKind};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::process::Command;

impl ToolExecutor {
    pub async fn execute_shell(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { command: String, cwd: Option<String>, timeout_ms: Option<u64> }
        let args: Args = serde_json::from_value(request.args)?;
        
        let cwd = args.cwd.map(|p| self.resolve_path(&p).unwrap_or_else(|_| PathBuf::from(p)))
            .unwrap_or_else(|| PathBuf::from(&self.workspace_root));
        
        let timeout = std::time::Duration::from_millis(args.timeout_ms.unwrap_or(self.timeout_ms));
        
        let output = tokio::time::timeout(timeout, async {
            Command::new("sh")
                .arg("-c")
                .arg(&args.command)
                .current_dir(&cwd)
                .output()
                .await
        }).await
        .context("Command timed out")?
        .context("Failed to execute command")?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let success = output.status.success();
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::ShellCommand,
            success,
            output: if success { stdout } else { stderr.clone() },
            error: if success { None } else { Some(stderr) },
            duration_ms: 0,
            metadata: HashMap::from([
                ("command".to_string(), serde_json::json!(args.command)),
                ("exit_code".to_string(), serde_json::json!(output.status.code().unwrap_or(-1))),
                ("cwd".to_string(), serde_json::json!(cwd.to_string_lossy())),
            ]),
        })
    }

    pub async fn execute_terminal(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { command: String, cwd: Option<String> }
        let args: Args = serde_json::from_value(request.args)?;
        
        // terminal.run has no allowlist - runs any command
        let cwd = args.cwd.map(|p| self.resolve_path(&p).unwrap_or_else(|_| PathBuf::from(p)))
            .unwrap_or_else(|| PathBuf::from(&self.workspace_root));
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&args.command)
            .current_dir(&cwd)
            .output()
            .await
            .context("Failed to execute terminal command")?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let success = output.status.success();
        
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::TerminalRun,
            success,
            output: if success { stdout } else { stderr.clone() },
            error: if success { None } else { Some(stderr) },
            duration_ms: 0,
            metadata: HashMap::from([
                ("command".to_string(), serde_json::json!(args.command)),
                ("exit_code".to_string(), serde_json::json!(output.status.code().unwrap_or(-1))),
                ("cwd".to_string(), serde_json::json!(cwd.to_string_lossy())),
            ]),
        })
    }
}