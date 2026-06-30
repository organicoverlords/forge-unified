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
    "bundle": ROOT / "crates/webui/src/chat_ui.rs",
    "state": ROOT / "PROJECT_STATE.md",
}

REQUIRED = {
    "agent": ["pub async fn retry_source", "pub async fn fork_conversation", "pub async fn revert_last_turn", "session_control_receipt", "SESSION_CONTROL_BEHAVIOR", "forge.session_control"],
    "routes": ["pub async fn checkpoint", "pub async fn fork", "pub async fn revert_last_turn", "pub async fn retry_source", "backend_backed", "control_receipt", "forge.session_control"],
    "lib": ["pub mod conversation_controls;", "/api/conversations/:id/checkpoint", "/api/conversations/:id/fork", "/api/conversations/:id/revert-last-turn", "/api/conversations/:id/retry-source"],
    "ui": ["backend-session-controls", "backend-checkpoint-action", "backend-fork-action", "backend-revert-action", "backend-retry-source-action", "/checkpoint", "/fork", "/revert-last-turn", "/retry-source", "backend-session-control-receipt", "copy-session-control-receipt", "forge:session-control", "session-control-event-ledger", "backend-session-control-ledger", "backend-session-control-event-row", "copy-session-control-event", "data-session-control-event", "session-control-event-disclosure", "backend-session-control-event-detail", "show-session-control-event", "aria-expanded", "aria-controls"],
    "bundle": ["include_str!(\"chat_ui_session_controls.html\")"],
    "state": ["backend-backed session controls", "checkpoint, fork, revert latest turn, and retry source", "Forge-local session control receipts", "session control event ledger", "copy session control event", "session-control event disclosure", "crates/webui/src/conversation_controls.rs", "crates/webui/src/chat_ui_session_controls.html"],
}

FORBIDDEN_RUNTIME_SOURCE_PATHS = {
    "agent": ["SESSION_CONTROL_SOURCE", "packages/session-ui/src/components/session-turn.tsx"],
    "routes": ["packages/session-ui/src/components/session-turn.tsx"],
    "ui": ["packages/session-ui/src/components/session-turn.tsx", "opencode_source", "opencode_runtime_source"],
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
