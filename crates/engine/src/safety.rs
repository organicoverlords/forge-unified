//! Safety checker — approval gate for tool execution.

use crate::config::ApprovalMode;
use crate::types::ToolKind;
use std::collections::HashSet;

pub struct SafetyChecker {
    mode: ApprovalMode,
    blocked_tools: HashSet<ToolKind>,
    allowed_tools: HashSet<ToolKind>,
}

impl SafetyChecker {
    pub fn new(mode: ApprovalMode) -> Self {
        let blocked_tools = match mode {
            ApprovalMode::ReadOnly => HashSet::from([
                ToolKind::FileWrite,
                ToolKind::FileEdit,
                ToolKind::FileDelete,
                ToolKind::ApplyPatch,
                ToolKind::ShellCommand,
                ToolKind::TerminalRun,
            ]),
            ApprovalMode::Blocked => ToolKind::variant_count().into_iter().collect(),
            _ => HashSet::new(),
        };

        Self {
            mode,
            blocked_tools,
            allowed_tools: HashSet::new(),
        }
    }

    pub async fn check_tool(&self, kind: &ToolKind) -> bool {
        if self.blocked_tools.contains(kind) {
            return false;
        }
        if !self.allowed_tools.is_empty() && !self.allowed_tools.contains(kind) {
            return false;
        }
        true
    }

    pub fn set_mode(&mut self, mode: ApprovalMode) {
        self.mode = mode.clone();
        *self = Self::new(mode);
    }

    pub fn mode(&self) -> &ApprovalMode {
        &self.mode
    }

    pub fn allow_tool(&mut self, kind: ToolKind) {
        self.blocked_tools.remove(&kind);
        self.allowed_tools.insert(kind);
    }

    pub fn block_tool(&mut self, kind: ToolKind) {
        self.allowed_tools.remove(&kind);
        self.blocked_tools.insert(kind);
    }
}

impl ToolKind {
    pub fn variant_count() -> Vec<ToolKind> {
        vec![
            ToolKind::FileRead,
            ToolKind::FileWrite,
            ToolKind::FileEdit,
            ToolKind::FileDelete,
            ToolKind::FileList,
            ToolKind::FileGlob,
            ToolKind::FileSearch,
            ToolKind::WebFetch,
            ToolKind::WebSearch,
            ToolKind::ShellCommand,
            ToolKind::TerminalRun,
            ToolKind::Task,
            ToolKind::BatchParallel,
            ToolKind::RepoInfo,
            ToolKind::ProposePatch,
            ToolKind::ApplyPatch,
            ToolKind::SwitchMode,
            ToolKind::BrowserProof,
            ToolKind::VisionReview,
            ToolKind::GraphBuild,
            ToolKind::GraphQuery,
        ]
    }
}
