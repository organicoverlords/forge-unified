#!/usr/bin/env python3
"""Write a compact human-readable Live WebUI proof turn summary.

OpenCode source anchor:
- anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts

OpenCode emits a final turn summary with agent, model, and duration. Forge's
Live WebUI proof needs the same reviewer-friendly terminal summary, plus proof
status for screenshots/checkers because the user should not need to unzip and
inspect every JSON file manually.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

OPENCODE_SOURCE = "packages/opencode/src/cli/cmd/run/turn-summary.ts"


def load_json(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return {}
    return data if isinstance(data, dict) else {}


def parse_sse_counts(path: Path) -> dict[str, int]:
    text = path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""
    return {
        "tool_call_events": text.count("event: tool-call"),
        "tool_result_events": text.count("event: tool-result"),
        "text_delta_events": text.count("event: text-delta"),
        "run_finish_events": text.count("event: run-finish"),
    }


def screenshot_status(path: Path) -> dict[str, Any]:
    return {
        "path": str(path),
        "present": path.is_file(),
        "size_bytes": path.stat().st_size if path.exists() else 0,
        "png_header": path.is_file() and path.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n",
    }


def checker_status(path: Path) -> dict[str, Any]:
    data = load_json(path)
    failed = data.get("failed_checks") if isinstance(data.get("failed_checks"), list) else []
    return {
        "path": str(path),
        "present": path.is_file(),
        "passed": data.get("passed") is True,
        "failed_count": len(failed),
        "failed_checks": [item.get("name") for item in failed if isinstance(item, dict)],
    }


def natural_feature_status(proof_dir: Path) -> dict[str, Any]:
    root = proof_dir / "natural-feature-work"
    summary = checker_status(root / "summary.json")
    screenshot = screenshot_status(root / "webui.png")
    stream_counts = parse_sse_counts(root / "chat-stream.sse")
    data = load_json(root / "summary.json")
    return {
        "summary": summary,
        "screenshot": screenshot,
        "stream_counts": stream_counts,
        "provider": data.get("provider"),
        "model": data.get("model"),
        "tool_call_events": data.get("tool_call_events"),
        "tool_result_events": data.get("tool_result_events"),
        "normal_webui_path": data.get("normal_webui_path") is True,
    }


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: summarize-live-webui-proof.py PROOF_DIR", file=sys.stderr)
        return 2

    proof_dir = Path(sys.argv[1])
    conversation = load_json(proof_dir / "full-benchmark-conversation.json")
    quality = load_json(proof_dir / "quality-score.json")
    manifest = load_json(proof_dir / "live-webui-proof-manifest.json")
    stream_counts = parse_sse_counts(proof_dir / "full-benchmark-stream.sse")
    hard_checker = checker_status(proof_dir / "full-benchmark-checker.json")
    workflow_checker = checker_status(proof_dir / "opencode-workflow-checker.json")
    quality_checker = checker_status(proof_dir / "quality-score.json")
    manifest_checker = checker_status(proof_dir / "live-webui-proof-manifest.json")
    natural_feature = natural_feature_status(proof_dir)

    screenshots = {
        "full_benchmark": screenshot_status(proof_dir / "full-benchmark-webui.png"),
        "tool_lifecycle": screenshot_status(proof_dir / "tool-lifecycle-webui.png"),
        "event_rail": screenshot_status(proof_dir / "event-rail.png"),
        "home": screenshot_status(proof_dir / "webui.png"),
    }
    checkers = {
        "full_benchmark": hard_checker,
        "opencode_workflow": workflow_checker,
        "quality_score": quality_checker,
        "manifest": manifest_checker,
    }
    natural_ok = natural_feature["summary"]["passed"] and natural_feature["screenshot"]["present"] and natural_feature["screenshot"]["png_header"]
    passed = (
        all(item["passed"] for item in checkers.values())
        and all(item["present"] and item["png_header"] for item in screenshots.values())
        and natural_ok
    )
    summary = {
        "passed": passed,
        "provider": conversation.get("provider"),
        "model": conversation.get("model"),
        "quality_percent": quality.get("percent"),
        "tool_results": conversation.get("tool_results") or quality.get("tool_results"),
        "stream_counts": stream_counts,
        "screenshots": screenshots,
        "checkers": checkers,
        "natural_feature_build": natural_feature,
        "manifest_passed": manifest.get("passed") is True,
        "opencode_sources": [OPENCODE_SOURCE],
    }
    (proof_dir / "live-webui-turn-summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    md = [
        "# Live WebUI turn summary",
        "",
        f"- passed: `{str(summary['passed']).lower()}`",
        f"- provider: `{summary.get('provider')}`",
        f"- model: `{summary.get('model')}`",
        f"- quality: `{summary.get('quality_percent')}`",
        f"- tool results: `{summary.get('tool_results')}`",
        f"- tool-call events: `{stream_counts['tool_call_events']}`",
        f"- tool-result events: `{stream_counts['tool_result_events']}`",
        "",
        "## Checkers",
    ]
    for name, item in checkers.items():
        md.append(f"- {name}: `{str(item['passed']).lower()}` failed_count=`{item['failed_count']}`")
    md.extend(["", "## Natural feature-build prompt"])
    md.append(f"- passed: `{str(natural_feature['summary']['passed']).lower()}`")
    md.append(f"- provider: `{natural_feature.get('provider')}`")
    md.append(f"- model: `{natural_feature.get('model')}`")
    md.append(f"- tool-call events: `{natural_feature.get('tool_call_events')}`")
    md.append(f"- tool-result events: `{natural_feature.get('tool_result_events')}`")
    md.append(f"- screenshot: `{natural_feature['screenshot']['path']}` present=`{str(natural_feature['screenshot']['present']).lower()}` size=`{natural_feature['screenshot']['size_bytes']}`")
    md.extend(["", "## Screenshots"])
    for name, item in screenshots.items():
        md.append(f"- {name}: `{item['path']}` present=`{str(item['present']).lower()}` size=`{item['size_bytes']}`")
    md.extend(["", "## OpenCode source", f"- `{OPENCODE_SOURCE}`", ""])
    (proof_dir / "live-webui-turn-summary.md").write_text("\n".join(md), encoding="utf-8")

    print(json.dumps(summary, indent=2, sort_keys=True))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
