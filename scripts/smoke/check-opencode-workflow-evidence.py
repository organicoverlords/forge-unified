#!/usr/bin/env python3
"""Validate OpenCode-style todo, subagent, and parallel-tool evidence."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def parse_sse(path: Path) -> list[dict[str, Any]]:
    text = path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""
    events: list[dict[str, Any]] = []
    name = "message"
    data_lines: list[str] = []
    for raw in text.splitlines() + [""]:
        if not raw:
            if data_lines:
                payload = "\n".join(data_lines)
                try:
                    data: Any = json.loads(payload)
                except json.JSONDecodeError:
                    data = payload
                events.append({"event": name, "data": data})
            name = "message"
            data_lines = []
            continue
        if raw.startswith("event: "):
            name = raw[len("event: ") :]
        elif raw.startswith("data: "):
            data_lines.append(raw[len("data: ") :])
    return events


def tool_results(conv: dict[str, Any]) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    for msg in conv.get("messages", []):
        for result in msg.get("tool_results") or []:
            out.append(result)
            out.extend(expand_batch_results(result))
    return out


def expand_batch_results(result: dict[str, Any]) -> list[dict[str, Any]]:
    if result.get("kind") != "BatchParallel":
        return []
    output_text = result.get("output")
    if not isinstance(output_text, str) or not output_text.strip():
        return []
    try:
        nested = json.loads(output_text)
    except json.JSONDecodeError:
        return []
    if not isinstance(nested, list):
        return []
    out: list[dict[str, Any]] = []
    for item in nested:
        if isinstance(item, dict):
            out.append(item)
            out.extend(expand_batch_results(item))
    return out


def meta(result: dict[str, Any], key: str) -> Any:
    return (result.get("metadata") or {}).get(key)


def output(result: dict[str, Any]) -> str:
    return str(result.get("output") or "")


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: check-opencode-workflow-evidence.py conversation.json stream.sse output.json", file=sys.stderr)
        return 2
    conv_path, stream_path, out_path = map(Path, sys.argv[1:])
    conv = load_json(conv_path)
    events = parse_sse(stream_path)
    results = tool_results(conv)
    stream_text = stream_path.read_text(encoding="utf-8", errors="replace") if stream_path.exists() else ""

    todo = [r for r in results if r.get("success") is True and r.get("kind") == "Task" and (meta(r, "tool_alias") == "todo_write" or "todo_write" in output(r))]
    subagents = [r for r in results if r.get("success") is True and r.get("kind") == "Task" and meta(r, "tool_alias") != "todo_write"]
    batches = [r for r in results if r.get("success") is True and r.get("kind") == "BatchParallel"]
    batch_events = [e for e in events if e.get("event") in {"tool-call", "tool-result"} and "batch_parallel" in json.dumps(e.get("data"), sort_keys=True)]

    checks = [
        {"name": "todo_write_result_present", "passed": bool(todo), "evidence": [{"todo_count": meta(r, "todo_count"), "completed": meta(r, "completed")} for r in todo[:3]]},
        {"name": "task_subagent_result_present", "passed": bool(subagents), "evidence": [{"agent": meta(r, "agent"), "description": meta(r, "description")} for r in subagents[:3]]},
        {"name": "batch_parallel_result_present", "passed": bool(batches), "evidence": [{"total_calls": meta(r, "total_calls"), "successful": meta(r, "successful")} for r in batches[:3]]},
        {"name": "batch_parallel_stream_visible", "passed": bool(batch_events) or "batch_parallel" in stream_text, "evidence": {"events": len(batch_events)}},
    ]
    failed = [c for c in checks if not c["passed"]]
    report = {"passed": not failed, "provider": conv.get("provider"), "model": conv.get("model"), "checks": checks, "failed_checks": failed}
    out_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2, sort_keys=True))
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
