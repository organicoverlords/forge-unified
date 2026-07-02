#!/usr/bin/env python3
"""Deterministic gate for WebUI conversation controls."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FILES = {
    "agent": ROOT / "crates/engine/src/agent.rs",
    "routes": ROOT / "crates/webui/src/conversation_controls.rs",
    "lib": ROOT / "crates/webui/src/lib.rs",
    "ui": ROOT / "crates/webui/src/chat_ui_session_controls.html",
    "search_ui": ROOT / "crates/webui/src/chat_ui_session_control_search.html",
    "bundle": ROOT / "crates/webui/src/chat_ui.rs",
    "state": ROOT / "PROJECT_STATE.md",
}

REQUIRED = {
    "agent": ["pub async fn retry_source", "pub async fn fork_conversation", "pub async fn revert_last_turn", "session_control_receipt", "SESSION_CONTROL_BEHAVIOR", "forge.session_control"],
    "routes": ["pub async fn checkpoint", "pub async fn fork", "pub async fn revert_last_turn", "pub async fn retry_source", "backend_backed", "control_receipt", "forge.session_control", "SESSION_CONTROL_RECEIPT_SEQUENCE", "receipt_id", "sequence", "forge.webui.session_controls"],
    "lib": ["pub mod conversation_controls;", "/api/conversations/:id/checkpoint", "/api/conversations/:id/fork", "/api/conversations/:id/revert-last-turn", "/api/conversations/:id/retry-source"],
    "ui": ["backend-session-controls", "backend-checkpoint-action", "backend-fork-action", "backend-revert-action", "backend-retry-source-action", "/checkpoint", "/fork", "/revert-last-turn", "/retry-source", "backend-session-control-receipt", "copy-session-control-receipt", "forge:session-control", "session-control-event-ledger", "backend-session-control-ledger", "backend-session-control-event-row", "copy-session-control-event", "data-session-control-event", "session-control-event-disclosure", "backend-session-control-event-detail", "show-session-control-event", "aria-expanded", "aria-controls", "session-control-count-summary", "backend-session-control-summary", "backend-session-control-count", "session-control-filter", "session-control-filter-all", "session-control-filter-ok", "session-control-filter-error", "aria-pressed", "session-control-diff-summary", "backend-session-control-diff-summary", "backend-session-control-diff-chip", "session-control-diff-before", "session-control-diff-after", "session-control-diff-removed", "removed_messages", "session-control-duration-summary", "backend-session-control-duration-summary", "backend-session-control-duration-chip", "session-control-duration-ms", "session-control-started-at", "session-control-completed-at", "ui_timing", "duration_ms", "started_at", "completed_at", "session-control-ledger-overflow", "backend-session-control-overflow-toggle", "session-control-show-all", "session-control-show-less", "session-control-visible-count", "ledgerExpanded", "show older", "show less", "session-control-hidden-overflow-row", "session-control-ledger-export", "copy-session-control-ledger", "copy all events", "forge.session_control_ledger", "session-control-error-card", "backend-session-control-error-card", "copy-session-control-error", "copy latest error", "latest-session-control-error"],
    "search_ui": ["session-control-ledger-search", "session-control-ledger-search-input", "session-control-ledger-search-count", "session-control-search-query", "data-session-search-hidden", "backend-session-control-event-row", "data-session-control-event", "search session events", "aria-label", "MutationObserver"],
    "bundle": ["include_str!(\"chat_ui_session_controls.html\")", "include_str!(\"chat_ui_session_control_search.html\")"],
    "state": ["backend-backed session controls", "checkpoint, fork, revert latest turn, and retry source", "Forge-local session control receipts", "session control event ledger", "copy session control event", "session-control event disclosure", "session-control count summary", "status filters", "session-control diff summary", "before/after/removed message chips", "session-control duration summary", "start/completion/duration chips", "session-control ledger overflow", "show older/show less overflow toggle", "visible ledger count", "hidden older receipt row", "session-control ledger export", "copy all events", "session-control error card", "copy latest error", "latest session-control error", "stable session-control receipt identity", "receipt_id", "sequence", "forge.webui.session_controls", "session-control ledger search", "search session events", "data-session-search-hidden", "crates/webui/src/conversation_controls.rs", "crates/webui/src/chat_ui_session_controls.html", "crates/webui/src/chat_ui_session_control_search.html"],
}

FORBIDDEN_RUNTIME_SOURCE_PATHS = {
    "agent": ["SESSION_CONTROL_SOURCE", "packages/session-ui/src/components/session-turn.tsx"],
    "routes": ["packages/session-ui/src/components/session-turn.tsx"],
    "ui": ["packages/session-ui/src/components/session-turn.tsx", "opencode_source", "opencode_runtime_source"],
    "search_ui": ["packages/session-ui/src/components/session-turn.tsx", "opencode_source", "opencode_runtime_source"],
}


def main() -> int:
    checks = []
    for name, path in FILES.items():
        text = path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""
        for token in REQUIRED[name]:
            checks.append({"name": f"{name}:{token}", "passed": token in text})
        for token in FORBIDDEN_RUNTIME_SOURCE_PATHS.get(name, []):
            checks.append({"name": f"{name}:no-runtime-source:{token}", "passed": token not in text})
    failed = [check for check in checks if not check["passed"]]
    report = {"passed": not failed, "checks": checks, "failed_checks": failed}
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
