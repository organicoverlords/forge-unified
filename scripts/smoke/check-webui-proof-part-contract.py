#!/usr/bin/env python3
"""Validate Forge's browser WebUI proof-part rendering contract.

OpenCode source anchors are developer-only references for behavior and
structure, not Forge runtime branding:
- anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx
- anomalyco/opencode:packages/session-ui/src/components/message-part.tsx
- anomalyco/opencode:packages/session-ui/src/components/basic-tool.tsx
- anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx

This guard is intentionally source-backed and UI-facing: final browser proof
must render stable, readable session turns, assistant parts, tool cards,
provider/model route proof, copy/retry affordances, file receipts, turn receipt
summaries, timeline actions, and collapsed technical details instead of relying
on raw JSON or raw tool identifiers as the primary user-visible evidence.
"""

from __future__ import annotations

import json
from pathlib import Path

CHAT_UI_PATHS = [
    Path("crates/webui/src/chat_ui.rs"),
    Path("crates/webui/src/chat_ui.html"),
    Path("crates/webui/src/chat_ui_enhancements.html"),
]
PROJECT_STATE = Path("PROJECT_STATE.md")
PROOF_DOC_DIR = Path("docs/generated/proof")
OPENCODE_SOURCES = [
    "packages/session-ui/src/components/session-turn.tsx",
    "packages/session-ui/src/components/message-part.tsx",
    "packages/session-ui/src/components/basic-tool.tsx",
    "packages/session-ui/src/components/tool-count-summary.tsx",
]

REQUIRED_UI_TOKENS = [
    "proof-digest-visible",
    "final-answer-visible",
    "provider-model-visible",
    "human-tool-label",
    "session-turn-central",
    "session-turn-diffs-group",
    "assistant-parts",
    "message-part",
    "opencode-tool-result-card",
    "opencode-live-toolpart",
    "collapsible-tool-card",
    "deferred-technical-content",
    "copy-retry-actions",
    "provider-executed action:",
    "technical details",
    "Session timeline",
    "Thinking / working",
    "Changed files / file receipts",
    "todo-status-summary",
    "todo-counts",
    "Plan updated",
    "timeline-file-diff-groups",
    "timeline-action-groups",
    "turn-receipt-toolbar",
    "file-diff-summary-visible",
    "stable-session-receipts",
    "Receipts grouped by turn",
    "copy timeline",
    "copy receipts",
]

REQUIRED_HUMAN_LABELS = [
    "Read file",
    "Write file",
    "Edit file",
    "Run command",
    "Run tools in parallel",
    "Delegate subtask",
    "Apply patch",
    "Update plan",
]

FORBIDDEN_PRIMARY_MARKERS = [
    "raw tool:",
    "Raw tool",
    "OpenCode-style",
]

REQUIRED_PROOF_TRAIL_TOKENS = [
    "packages/session-ui/src/components/session-turn.tsx",
    "packages/session-ui/src/components/message-part.tsx",
    "packages/session-ui/src/components/basic-tool.tsx",
    "packages/session-ui/src/components/tool-count-summary.tsx",
    "proof-final",
    "session turn",
    "assistant parts",
    "copy/retry",
    "changed files",
    "collapsed technical details",
    "turn receipt grouping",
    "stable session receipts",
    "timeline action groups",
    "file diff summary",
]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def ui_text() -> str:
    return "\n".join(read(path) for path in CHAT_UI_PATHS)


def proof_trail_text() -> str:
    chunks = [read(PROJECT_STATE)]
    if PROOF_DOC_DIR.exists():
        chunks.extend(read(path) for path in PROOF_DOC_DIR.glob("*.md"))
    return "\n".join(chunks).lower()


def main() -> int:
    ui = ui_text()
    trail = proof_trail_text()
    checks: list[dict[str, object]] = []

    for token in REQUIRED_UI_TOKENS:
        checks.append({"name": f"ui_token:{token}", "passed": token in ui})
    for label in REQUIRED_HUMAN_LABELS:
        checks.append({"name": f"human_label:{label}", "passed": label in ui})
    for marker in FORBIDDEN_PRIMARY_MARKERS:
        checks.append({"name": f"forbidden_primary_marker_absent:{marker}", "passed": marker not in ui})
    for token in REQUIRED_PROOF_TRAIL_TOKENS:
        checks.append({"name": f"proof_trail:{token}", "passed": token.lower() in trail})

    checks.append({
        "name": "ui_bundle_is_reviewable_html_not_giant_rust_string",
        "passed": all(token in ui for token in [
            "include_str!(\"chat_ui.html\")",
            "include_str!(\"chat_ui_enhancements.html\")",
            "<body data-proof=",
        ]),
    })
    checks.append({
        "name": "final_proof_has_digest_before_session_turns",
        "passed": "if(proofFinal)box.appendChild(proofDigest());" in ui,
    })
    checks.append({
        "name": "technical_details_are_collapsed_and_hidden_from_final_proof",
        "passed": "className='technical-details'" in ui and "body.proof-final .technical-details{display:none}" in ui,
    })
    checks.append({
        "name": "todo_write_gets_status_summary_not_generic_json",
        "passed": "todo-status-summary" in ui and "if(n==='todo_write')return'Plan updated.'" in ui,
    })
    checks.append({
        "name": "central_session_turn_has_actions_and_file_receipts",
        "passed": all(token in ui for token in ["function turn(t,i,active)", "copy turn", "retry", "Changed files / file receipts"]),
    })
    checks.append({
        "name": "turn_receipt_enhancer_groups_files_tools_and_statuses",
        "passed": all(token in ui for token in [
            "function enhanceTurn(turn)",
            "Receipts grouped by turn",
            "file receipts",
            "tool cards",
            "status chips",
            "copy receipts",
            "copy files",
        ]),
    })
    checks.append({
        "name": "session_timeline_has_real_copy_retry_actions",
        "passed": all(token in ui for token in [
            "copy timeline",
            "retry latest prompt",
            "copy latest files",
            "Stable session receipts",
        ]),
    })

    failed = [check for check in checks if not check["passed"]]
    report = {
        "passed": not failed,
        "opencode_sources": OPENCODE_SOURCES,
        "forge_paths": [str(path) for path in CHAT_UI_PATHS] + [str(PROJECT_STATE), "docs/generated/proof/*.md"],
        "checks": checks,
        "failed_checks": failed,
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
