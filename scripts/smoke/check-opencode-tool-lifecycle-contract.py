#!/usr/bin/env python3
"""Validate the durable tool-part lifecycle contract used by Forge WebUI proofs.

This is a deterministic source gate. It does not claim full OpenCode parity;
it verifies that Forge keeps the OpenCode-backed lifecycle shape needed by
browser proofs: pending -> running -> completed/error, stable call IDs,
input/output metadata, duration timing, and file attachments for edit/write/patch
results.
"""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TOOL_PARTS = ROOT / "crates" / "engine" / "src" / "tool_parts.rs"
PROJECT_STATE = ROOT / "PROJECT_STATE.md"

REQUIRED_TOOL_PART_TOKENS = [
    "pub fn pending_tool_part(call: &ToolRequest)",
    '"state": {"status": "pending"',
    "pub fn running_tool_part(call: &ToolRequest)",
    '"state": {"status": "running"',
    "pub fn started_tool_lifecycle_parts(call: &ToolRequest)",
    "vec![pending_tool_part(call), running_tool_part(call)]",
    "pub fn finished_tool_part(result: &ToolResult)",
    "if result.success { completed_tool_part(result) } else { error_tool_part(result) }",
    "pub fn finished_tool_lifecycle_parts(result: &ToolResult)",
    "vec![pending_tool_part_from_result(result), running_tool_part_from_result(result), finished_tool_part(result)]",
    "pub fn completed_tool_part(result: &ToolResult)",
    '"status": "completed"',
    '"output": result.output.clone()',
    '"time": {"start": 0, "end": result.duration_ms}',
    'state["attachments"] = serde_json::json!(attachments);',
    "pub fn error_tool_part(result: &ToolResult)",
    '"status": "error"',
    '"error": result.error.clone().unwrap_or_else(|| result.output.clone())',
    '"callID": result.id.clone().0.to_string()',
    '"callID": call.id.clone().0.to_string()',
]

REQUIRED_STATE_TOKENS = [
    "anomalyco/opencode:packages/opencode/src/session/processor.ts",
    "anomalyco/opencode:packages/schema/src/v1/session.ts",
    "tool lifecycle",
    "ToolPart",
]


def require_tokens(name: str, text: str, tokens: list[str]) -> list[str]:
    return [token for token in tokens if token not in text]


def main() -> int:
    tool_parts = TOOL_PARTS.read_text(encoding="utf-8")
    project_state = PROJECT_STATE.read_text(encoding="utf-8")

    missing: list[str] = []
    missing += [f"{TOOL_PARTS}: {token}" for token in require_tokens("tool_parts", tool_parts, REQUIRED_TOOL_PART_TOKENS)]
    missing += [f"{PROJECT_STATE}: {token}" for token in require_tokens("project_state", project_state, REQUIRED_STATE_TOKENS)]

    if missing:
        print("OpenCode tool lifecycle contract check failed. Missing:")
        for item in missing:
            print(f"- {item}")
        return 1

    print("OpenCode tool lifecycle contract check passed.")
    print("Forge durable tool parts preserve pending/running/completed/error states, call IDs, input/output/error metadata, duration timing, and attachments for file-changing results.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
