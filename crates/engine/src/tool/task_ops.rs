//! Task, todo, repo info, propose patch, and switch mode tools.

use crate::tool::ToolExecutor;
use crate::types::{ToolKind, ToolRequest, ToolResult};
use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;
use uuid::Uuid;

impl ToolExecutor {
    pub async fn execute_task(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        #[allow(dead_code)]
        struct Args {
            description: Option<String>,
            prompt: String,
            background: Option<bool>,
            tools: Option<Vec<String>>,
            agent: Option<String>,
        }
        let args: Args = serde_json::from_value(request.args)?;
        let task_id = Uuid::new_v4().to_string();
        let agent = args.agent.unwrap_or_else(|| "general".to_string());
        let allowed_tools = args.tools.unwrap_or_else(|| vec!["file_read".to_string(), "file_search".to_string(), "repo_info".to_string()]);
        let description = args.description.unwrap_or_else(|| task_summary(&args.prompt));
        let background = args.background.unwrap_or(false);

        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::Task,
            success: true,
            output: serde_json::json!({
                "task_id": task_id,
                "status": "completed",
                "agent": agent,
                "description": description,
                "prompt": args.prompt,
                "background": background,
                "allowed_tools": allowed_tools,
                "subagent_mode": "opencode_style_delegate_then_report",
                "result": "Subagent card created with bounded prompt and allowed tool scope. Continue by using the returned agent description as focused context, then verify with direct tools before finalizing."
            }).to_string(),
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("task_id".to_string(), serde_json::json!(task_id)),
                ("status".to_string(), serde_json::json!("completed")),
                ("agent".to_string(), serde_json::json!(agent)),
                ("description".to_string(), serde_json::json!(description)),
                ("prompt".to_string(), serde_json::json!(args.prompt)),
                ("background".to_string(), serde_json::json!(background)),
                ("allowed_tools".to_string(), serde_json::json!(allowed_tools)),
                ("opencode_task_source".to_string(), serde_json::json!("packages/opencode/src/tool/task.ts")),
                ("opencode_subagent_behavior".to_string(), serde_json::json!("delegate focused exploration; return concise result; do not replace direct evidence")),
            ]),
        })
    }

    pub async fn execute_todo_write(&self, request: ToolRequest) -> Result<ToolResult> {
        #[derive(serde::Deserialize)]
        struct Args { todos: Vec<TodoItem> }
        #[derive(serde::Deserialize, serde::Serialize, Clone)]
        struct TodoItem { content: String, status: String, priority: Option<String> }

        let args: Args = serde_json::from_value(request.args)?;
        if args.todos.is_empty() { anyhow::bail!("todo_write requires at least one todo"); }
        let valid = ["pending", "in_progress", "completed"];
        for todo in &args.todos {
            if todo.content.trim().is_empty() { anyhow::bail!("todo content cannot be empty"); }
            if !valid.contains(&todo.status.as_str()) { anyhow::bail!("invalid todo status: {}", todo.status); }
        }
        let completed = args.todos.iter().filter(|t| t.status == "completed").count();
        let in_progress = args.todos.iter().filter(|t| t.status == "in_progress").count();
        let pending = args.todos.iter().filter(|t| t.status == "pending").count();
        let output = serde_json::json!({
            "status": "updated",
            "todos": args.todos,
            "counts": { "completed": completed, "in_progress": in_progress, "pending": pending },
            "opencode_behavior": "TodoWrite checklist updated; mark items completed immediately as work finishes."
        }).to_string();
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::TodoWrite,
            success: true,
            output,
            error: None,
            duration_ms: 0,
            metadata: HashMap::from([
                ("opencode_todo_source".to_string(), serde_json::json!("packages/opencode/src/tool/todo.ts")),
                ("todo_count".to_string(), serde_json::json!(completed + in_progress + pending)),
                ("completed".to_string(), serde_json::json!(completed)),
                ("in_progress".to_string(), serde_json::json!(in_progress)),
                ("pending".to_string(), serde_json::json!(pending)),
            ]),
        })
    }

    pub async fn execute_repo_info(&self, request: ToolRequest) -> Result<ToolResult> {
        let mut info = serde_json::Map::new();
        let root = git_text(&self.workspace_root, &["rev-parse", "--show-toplevel"]);
        let branch = git_text(&self.workspace_root, &["symbolic-ref", "--quiet", "--short", "HEAD"])
            .or_else(|| git_text(&self.workspace_root, &["rev-parse", "--abbrev-ref", "HEAD"]));
        let head = git_text(&self.workspace_root, &["rev-parse", "HEAD"]);
        let short_head = git_text(&self.workspace_root, &["rev-parse", "--short", "HEAD"]);
        let remote = git_text(&self.workspace_root, &["remote", "get-url", "origin"]);
        let status_porcelain = git_text(&self.workspace_root, &["status", "--porcelain=v1"]).unwrap_or_default();
        let diff_stat = git_text(&self.workspace_root, &["diff", "--stat"]);
        let worktree_text = git_text(&self.workspace_root, &["worktree", "list", "--porcelain"]).unwrap_or_default();

        info.insert("root".to_string(), serde_json::json!(root));
        info.insert("branch".to_string(), serde_json::json!(branch));
        info.insert("head".to_string(), serde_json::json!(head));
        info.insert("short_head".to_string(), serde_json::json!(short_head));
        info.insert("remote_origin".to_string(), serde_json::json!(remote));
        info.insert("dirty".to_string(), serde_json::json!(!status_porcelain.trim().is_empty()));
        info.insert("status_porcelain".to_string(), serde_json::json!(status_porcelain));
        info.insert("diff_stat".to_string(), serde_json::json!(diff_stat));
        info.insert("worktrees".to_string(), serde_json::json!(parse_worktrees(&worktree_text)));
        info.insert("opencode_parity".to_string(), serde_json::json!({
            "copied_concepts": ["repo_discover", "remote_get", "history_head", "history_branch", "worktree_list", "status_snapshot"],
            "not_yet_copied": ["worktree_create", "worktree_remove", "tree_capture", "patch_restore", "permission_v2"]
        }));

        let output = serde_json::to_string_pretty(&info)?;
        Ok(ToolResult {
            id: request.id,
            kind: ToolKind::RepoInfo,
            success: true,
            output,
            error: None,
            duration_ms: 0,
            metadata: info.into_iter().collect(),
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

fn task_summary(prompt: &str) -> String {
    let mut summary = prompt.lines().next().unwrap_or("subtask").trim().chars().take(96).collect::<String>();
    if summary.is_empty() { summary = "subtask".to_string(); }
    summary
}

fn git_text(cwd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).current_dir(cwd).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_worktrees(input: &str) -> Vec<serde_json::Value> {
    let mut worktrees = Vec::new();
    let mut current = serde_json::Map::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                worktrees.push(serde_json::Value::Object(std::mem::take(&mut current)));
            }
            continue;
        }
        if let Some((key, value)) = line.split_once(' ') {
            current.insert(key.to_string(), serde_json::json!(value));
        } else {
            current.insert(line.to_string(), serde_json::json!(true));
        }
    }

    if !current.is_empty() {
        worktrees.push(serde_json::Value::Object(current));
    }
    worktrees
}
