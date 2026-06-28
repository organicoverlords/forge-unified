#!/usr/bin/env python3
"""Validate that a Live WebUI Feature Sprint proof directory is complete.

This is intentionally artifact-first: a green workflow is not enough unless the
uploaded directory contains the browser screenshot, browser JSON, stream,
conversation, status, and both checker JSON files that prove the natural WebUI
run used NVIDIA NIM and completed the full benchmark.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

REQUIRED_FILES = [
    "full-benchmark-webui.png",
    "full-benchmark-browser-proof.json",
    "full-benchmark-stream.sse",
    "full-benchmark-conversation.json",
    "full-benchmark-checker.json",
    "opencode-workflow-checker.json",
    "live-proof-status.txt",
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


def checker_passed(path: Path) -> bool:
    try:
        data = load_json(path)
    except Exception:
        return False
    return data.get("passed") is True and not data.get("failed_checks")


def status_value(status_text: str, key: str) -> str:
    prefix = f"{key}="
    for line in status_text.splitlines():
        if line.startswith(prefix):
            return line[len(prefix) :].strip()
    return ""


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: check-live-webui-proof-manifest.py PROOF_DIR OUTPUT_JSON", file=sys.stderr)
        return 2

    proof_dir = Path(sys.argv[1])
    output_path = Path(sys.argv[2])

    checks: list[dict[str, Any]] = []

    for name in REQUIRED_FILES:
        path = proof_dir / name
        checks.append({"name": f"required_file:{name}", "passed": non_empty(path), "size": path.stat().st_size if path.exists() else 0})

    stream_path = proof_dir / "full-benchmark-stream.sse"
    conversation_path = proof_dir / "full-benchmark-conversation.json"
    status_path = proof_dir / "live-proof-status.txt"
    stream_text = stream_path.read_text(encoding="utf-8", errors="replace") if stream_path.exists() else ""
    status_text = status_path.read_text(encoding="utf-8", errors="replace") if status_path.exists() else ""

    checks.append({"name": "full_checker_passed", "passed": checker_passed(proof_dir / "full-benchmark-checker.json")})
    checks.append({"name": "workflow_checker_passed", "passed": checker_passed(proof_dir / "opencode-workflow-checker.json")})
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
    checks.append({"name": "conversation_provider_is_nvidia_nim", "passed": provider == "nvidia_nim", "provider": provider})
    checks.append({"name": "conversation_model_recorded", "passed": bool(model), "model": model})
    checks.append({"name": "status_records_benchmark_screenshot", "passed": bool(status_value(status_text, "benchmark_screenshot"))})
    checks.append({"name": "status_records_workflow_checker", "passed": bool(status_value(status_text, "workflow_checker"))})

    failed = [check for check in checks if not check["passed"]]
    report = {
        "passed": not failed,
        "proof_dir": str(proof_dir),
        "provider": provider,
        "model": model,
        "required_files": REQUIRED_FILES,
        "checks": checks,
        "failed_checks": failed,
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
