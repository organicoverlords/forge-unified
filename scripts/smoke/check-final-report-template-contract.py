#!/usr/bin/env python3
"""CI gate for Forge max-step final-report contract.

Source-backed parity anchor:
- anomalyco/opencode:packages/core/src/session/runner/max-steps.ts

OpenCode's max-step prompt disables tools and requires text only containing:
summary of work done, remaining tasks, and next recommendations. Forge's live
benchmark additionally requires its final Markdown contract to be deterministic
when NVIDIA NIM returns an empty or malformed final answer.
"""
from __future__ import annotations

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[2]
ORCH = ROOT / "crates" / "engine" / "src" / "orchestrator.rs"
PROMPT = ROOT / "scripts" / "smoke" / "full-agentic-benchmark-prompt.txt"

REQUIRED_FINAL_LABELS = [
    "## Founder report",
    "## Technical report",
    "confidence (0-100)",
    "VERIFIED",
    "LIKELY",
    "UNKNOWN",
    "evidence",
    "assumptions",
    "failed hypotheses",
    "rollback strategy",
    "blast radius",
    "implementation difficulty",
    "rollback difficulty",
    "files created",
    "files removed",
    "files modified",
    "tests run",
    "unresolved risks",
]

OPENCODE_MAX_STEP_MARKERS = [
    "tools are disabled",
    "summarize work done so far",
    "remaining tasks",
    "recommendations",
    "text-only",
]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def main() -> int:
    source = ORCH.read_text(encoding="utf-8")
    prompt = PROMPT.read_text(encoding="utf-8")

    for marker in REQUIRED_FINAL_LABELS:
        require(marker in source, f"orchestrator finalization is missing required final label: {marker}")
        require(marker in prompt, f"benchmark prompt is missing required final label: {marker}")

    lower_source = source.lower()
    for marker in OPENCODE_MAX_STEP_MARKERS:
        require(marker in lower_source, f"orchestrator is missing OpenCode max-step behavior marker: {marker}")

    require(
        "packages/core/src/session/runner/max-steps.ts" in source,
        "orchestrator must record exact OpenCode max-step source path",
    )
    require(
        "tools: None" in source and "tool_choice: None" in source,
        "forced finalization must disable tools for the final model call",
    )
    require(
        "fallback_final_report(provider, model" in source,
        "forced finalization must fall back to deterministic Markdown if provider final text is malformed",
    )
    require(
        "looks_like_final_report" in source,
        "forced finalization must validate final report shape before accepting provider output",
    )
    require(
        not re.search(r"cargo (build|check|test).*VERIFIED", source),
        "fallback must not hard-claim cargo commands as VERIFIED",
    )

    print("final report template contract gate passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
