#!/usr/bin/env python3
"""Validate WebUI proof-part rendering keeps OpenCode-style readable parts.

OpenCode source anchors:
- anomalyco/opencode:packages/web/src/components/share/part.tsx
- anomalyco/opencode:packages/web/src/components/share/part.module.css

This guard is intentionally source-backed and UI-facing: final browser proof must
render stable, readable part cards and route/metadata proof instead of relying on
raw JSON or raw tool identifiers as the primary user-visible evidence.
"""

from __future__ import annotations

import json
from pathlib import Path

CHAT_UI = Path("crates/webui/src/chat_ui.rs")
PROJECT_STATE = Path("PROJECT_STATE.md")
PROOF_DOC_DIR = Path("docs/generated/proof")
OPENCODE_SOURCES = [
    "packages/web/src/components/share/part.tsx",
    "packages/web/src/components/share/part.module.css",
]

REQUIRED_UI_TOKENS = [
    "proof-digest-visible",
    "final-answer-visible",
    "provider-model-visible",
    "human-tool-label",
    "opencode-tool-result-card",
    "opencode-live-toolpart",
    "provider-executed action:",
    "technical details",
    "if(proofFinal)return;",
    "body.proof-final details{display:none}",
]

REQUIRED_HUMAN_LABELS = [
    "Read file",
    "Write file",
    "Edit file",
    "Run command",
    "Run tools in parallel",
    "Delegate subtask",
    "Apply patch",
]

FORBIDDEN_PRIMARY_MARKERS = [
    "raw tool:",
    "Raw tool",
]

REQUIRED_PROOF_TRAIL_TOKENS = [
    "packages/web/src/components/share/part.tsx",
    "packages/web/src/components/share/part.module.css",
    "proof-final",
    "human label",
]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def proof_trail_text() -> str:
    chunks = [read(PROJECT_STATE)]
    if PROOF_DOC_DIR.exists():
        chunks.extend(read(path) for path in PROOF_DOC_DIR.glob("*.md"))
    return "\n".join(chunks).lower()


def main() -> int:
    ui = read(CHAT_UI)
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
        "name": "details_are_collapsed_outside_final_proof_only",
        "passed": "function addDetails(el,obj){if(proofFinal)return;" in ui and "body.proof-final details{display:none}" in ui,
    })
    checks.append({
        "name": "final_proof_has_digest_before_messages",
        "passed": "if(proofFinal)box.appendChild(proofDigest());" in ui,
    })

    failed = [check for check in checks if not check["passed"]]
    report = {
        "passed": not failed,
        "opencode_sources": OPENCODE_SOURCES,
        "forge_paths": [str(CHAT_UI), str(PROJECT_STATE), "docs/generated/proof/*.md"],
        "checks": checks,
        "failed_checks": failed,
    }
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
