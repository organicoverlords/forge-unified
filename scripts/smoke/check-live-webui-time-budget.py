#!/usr/bin/env python3
"""Validate the Live WebUI benchmark budget against final-report streaming needs.

Source-backed rationale:
OpenCode's max-step runner disables tools and requires a text-only response that
summarizes completed work, remaining work, and next recommendations. Forge's
full WebUI benchmark deliberately asks for that final Markdown report after the
last validation tool result, so the workflow budget must leave room for that
text-only finalization instead of timing out immediately after tool evidence is
ready.
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/live-webui-feature-sprint.yml"
PROMPT = ROOT / "scripts/smoke/full-agentic-benchmark-prompt.txt"

MIN_TIMEOUT_SECONDS = 840
MIN_ROUNDS = 36
REQUIRED_PROMPT_MARKERS = [
    "STOP USING TOOLS after the validation shell result",
    "Respond with text only",
    "## Founder report",
    "## Technical report",
    "rollback strategy",
    "unresolved risks",
]


def workflow_env_value(text: str, key: str) -> int:
    match = re.search(rf"^\s*{re.escape(key)}:\s*[\"']?(\d+)[\"']?\s*$", text, re.M)
    if not match:
        raise SystemExit(f"missing {key} in {WORKFLOW}")
    return int(match.group(1))


def main() -> int:
    workflow_text = WORKFLOW.read_text(encoding="utf-8")
    prompt_text = PROMPT.read_text(encoding="utf-8")

    timeout_seconds = workflow_env_value(workflow_text, "FORGE_BENCH_TIMEOUT_SECONDS")
    max_rounds = workflow_env_value(workflow_text, "FORGE_BENCH_MAX_ROUNDS")

    if timeout_seconds < MIN_TIMEOUT_SECONDS:
        raise SystemExit(
            f"FORGE_BENCH_TIMEOUT_SECONDS={timeout_seconds} is below "
            f"OpenCode-style finalization budget floor {MIN_TIMEOUT_SECONDS}"
        )
    if max_rounds < MIN_ROUNDS:
        raise SystemExit(f"FORGE_BENCH_MAX_ROUNDS={max_rounds} is below required floor {MIN_ROUNDS}")

    missing = [marker for marker in REQUIRED_PROMPT_MARKERS if marker not in prompt_text]
    if missing:
        raise SystemExit(f"full benchmark prompt missing finalization markers: {missing}")

    print(
        "live WebUI time budget ok: "
        f"timeout={timeout_seconds}s max_rounds={max_rounds}; "
        "prompt retains text-only finalization/report markers"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
