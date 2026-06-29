#!/usr/bin/env python3
"""Validate that a Live WebUI Feature Sprint proof directory is complete.

OpenCode source anchor:
- anomalyco/opencode:packages/core/src/session/runner/max-steps.ts

This is intentionally artifact-first: a green workflow is not enough unless the
uploaded directory contains the browser screenshot, browser JSON, stream,
conversation, status, and checker JSON files that prove the natural WebUI run
used NVIDIA NIM and completed the full benchmark. The OpenCode max-step source
requires a text-only final response after tools are disabled, so Forge must keep
browser-visible final-report evidence instead of accepting hidden local success.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

OPENCODE_SOURCE = "packages/core/src/session/runner/max-steps.ts"

REQUIRED_FILES = [
    "full-benchmark-webui.png",
    "full-benchmark-browser-proof.json",
    "full-benchmark-stream.sse",
    "full-benchmark-conversation.json",
    "full-benchmark-checker.json",
    "opencode-workflow-checker.json",
    "quality-score.json",
    "live-webui-proof-manifest.json",
    "live-proof-status.txt",
]

REQUIRED_BROWSER_MARKERS = [
    "Full six-phase agentic benchmark prompt",
    "Phase 1",
    "Phase 2",
    "Founder report",
    "Technical report",
    ".agent_test/repo_summary.md",
    ".agent_test/action_plan.json",
]

FORBIDDEN_RUNTIME_MARKERS = [
    "\"provider\":\"local\"",
    "\"provider\": \"local\"",
    "\"local_shortcut\":true",
    "\"local_shortcut\": true",
    "event: benchmark-phase",
    "packages/opencode/",
    "opencode_",
]


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def non_empty(path: Path) -> bool:
    return path.is_file() and path.stat().st_size > 0


def png_is_nonempty_screenshot(path: Path) -> bool:
    if not path.is_file() or path.stat().st_size < 1024:
        return False
    return path.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n"


def checker_passed(path: Path) -> bool:
    try:
        data = load_json(path)
    except Exception:
        return False
    return data.get("passed") is True and not data.get("failed_checks")


def quality_passed(path: Path) -> bool:
    try:
        data = load_json(path)
    except Exception:
        return False
    return data.get("passed") is True and float(data.get("percent") or 0) >= 85


def status_value(status_text: str, key: str) -> str:
    prefix = f"{key}="
    for line in status_text.splitlines():
        if line.startswith(prefix):
            return line[len(prefix) :].strip()
    return ""


def status_path_exists(status_text: str, key: str, expected_name: str) -> bool:
    value = status_value(status_text, key)
    return bool(value) and Path(value).name == expected_name


def manifest_self_reference_ok(proof_dir: Path, output_path: Path) -> bool:
    return output_path.name == "live-webui-proof-manifest.json" and output_path.parent == proof_dir


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: check-live-webui-proof-manifest.py PROOF_DIR OUTPUT_JSON", file=sys.stderr)
        return 2

    proof_dir = Path(sys.argv[1])
    output_path = Path(sys.argv[2])

    if manifest_self_reference_ok(proof_dir, output_path):
        output_path.write_text("{}\n", encoding="utf-8")

    checks: list[dict[str, Any]] = []

    for name in REQUIRED_FILES:
        path = proof_dir / name
        checks.append({"name": f"required_file:{name}", "passed": non_empty(path), "size": path.stat().st_size if path.exists() else 0})

    stream_path = proof_dir / "full-benchmark-stream.sse"
    conversation_path = proof_dir / "full-benchmark-conversation.json"
    status_path = proof_dir / "live-proof-status.txt"
    browser_proof_path = proof_dir / "full-benchmark-browser-proof.json"
    screenshot_path = proof_dir / "full-benchmark-webui.png"
    stream_text = stream_path.read_text(encoding="utf-8", errors="replace") if stream_path.exists() else ""
    status_text = status_path.read_text(encoding="utf-8", errors="replace") if status_path.exists() else ""
    browser_text = browser_proof_path.read_text(encoding="utf-8", errors="replace") if browser_proof_path.exists() else ""

    checks.append({"name": "full_checker_passed", "passed": checker_passed(proof_dir / "full-benchmark-checker.json")})
    checks.append({"name": "workflow_checker_passed", "passed": checker_passed(proof_dir / "opencode-workflow-checker.json")})
    checks.append({"name": "quality_score_passed", "passed": quality_passed(proof_dir / "quality-score.json")})
    checks.append({"name": "screenshot_is_png", "passed": png_is_nonempty_screenshot(screenshot_path), "size": screenshot_path.stat().st_size if screenshot_path.exists() else 0})
    checks.append({"name": "browser_proof_has_required_markers", "passed": all(marker in browser_text for marker in REQUIRED_BROWSER_MARKERS), "markers": REQUIRED_BROWSER_MARKERS})
    checks.append({"name": "stream_has_run_finish", "passed": "event: run-finish" in stream_text})
    checks.append({"name": "stream_has_tool_calls", "passed": stream_text.count("event: tool-call") >= 8, "count": stream_text.count("event: tool-call")})
    checks.append({"name": "stream_has_tool_results", "passed": stream_text.count("event: tool-result") >= 8, "count": stream_text.count("event: tool-result")})
    checks.append({"name": "runtime_has_no_shortcut_or_upstream_identity", "passed": not any(marker in stream_text for marker in FORBIDDEN_RUNTIME_MARKERS)})

    provider = ""
    model = ""
    try:
        conversation = load_json(conversation_path)
        provider = str(conversation.get("provider") or "")
        model = str(conversation.get("model") or "")
    except Exception:
        conversation = {}
    tool_results = int(conversation.get("tool_results") or 0) if isinstance(conversation, dict) else 0
    checks.append({"name": "conversation_provider_is_nvidia_nim", "passed": provider == "nvidia_nim", "provider": provider})
    checks.append({"name": "conversation_model_recorded", "passed": bool(model), "model": model})
    checks.append({"name": "conversation_has_tool_results", "passed": tool_results >= 8, "tool_results": tool_results})
    checks.append({"name": "status_records_benchmark_screenshot", "passed": status_path_exists(status_text, "benchmark_screenshot", "full-benchmark-webui.png")})
    checks.append({"name": "status_records_workflow_checker", "passed": status_path_exists(status_text, "workflow_checker", "opencode-workflow-checker.json")})

    failed = [check for check in checks if not check["passed"]]
    report = {
        "passed": not failed,
        "proof_dir": str(proof_dir),
        "provider": provider,
        "model": model,
        "required_files": REQUIRED_FILES,
        "required_browser_markers": REQUIRED_BROWSER_MARKERS,
        "opencode_sources": [OPENCODE_SOURCE],
        "checks": checks,
        "failed_checks": failed,
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
