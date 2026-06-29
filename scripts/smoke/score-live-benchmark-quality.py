#!/usr/bin/env python3
"""Score live WebUI benchmark proof quality.

This is intentionally stricter than the pass/fail evidence checker. It grades
whether the uploaded proof bundle is useful to a human reviewer: final answer
quality, claim/evidence alignment, browser proof usefulness, NIM-only routing,
and OpenCode-style stop-and-summarize behavior.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

REQUIRED_FINAL_LABELS = [
    "confidence (0-100)",
    "VERIFIED",
    "LIKELY",
    "UNKNOWN",
    "## Founder report",
    "## Technical report",
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

EXPECTED_FILES = {
    ".agent_test/repo_summary.md",
    ".agent_test/investigation.md",
    ".agent_test/action_plan.json",
}


def load_json(path: Path) -> Any:
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


def expand_batch_results(result: dict[str, Any]) -> list[dict[str, Any]]:
    if result.get("kind") != "BatchParallel":
        return []
    output = result.get("output")
    if not isinstance(output, str) or not output.strip():
        return []
    try:
        nested = json.loads(output)
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


def all_tool_results(conv: dict[str, Any]) -> list[dict[str, Any]]:
    results: list[dict[str, Any]] = []
    for msg in conv.get("messages", []):
        for result in msg.get("tool_results") or []:
            results.append(result)
            results.extend(expand_batch_results(result))
    return results


def final_text(conv: dict[str, Any]) -> str:
    for msg in reversed(conv.get("messages", [])):
        if msg.get("role") == "Assistant":
            return msg.get("content") or ""
    return ""


def metadata_path(result: dict[str, Any]) -> str:
    meta = result.get("metadata") or {}
    path = meta.get("path")
    if isinstance(path, str):
        return path
    input_value = meta.get("forge_tool_input")
    if isinstance(input_value, dict) and isinstance(input_value.get("path"), str):
        return input_value["path"]
    return ""


def metadata_command(result: dict[str, Any]) -> str:
    meta = result.get("metadata") or {}
    command = meta.get("command")
    if isinstance(command, str):
        return command
    input_value = meta.get("forge_tool_input")
    if isinstance(input_value, dict) and isinstance(input_value.get("command"), str):
        return input_value["command"]
    return ""


def ok_results(results: list[dict[str, Any]], kind: str | None = None, path: str | None = None, command_re: str | None = None) -> list[dict[str, Any]]:
    found: list[dict[str, Any]] = []
    for result in results:
        if result.get("success") is not True:
            continue
        if kind is not None and result.get("kind") != kind:
            continue
        if path is not None and metadata_path(result) != path:
            continue
        if command_re is not None and not re.search(command_re, metadata_command(result), re.I):
            continue
        found.append(result)
    return found


def word_count(text: str) -> int:
    return len(re.findall(r"\b[\w'-]+\b", text))


def add(scores: list[dict[str, Any]], name: str, points: int, earned: bool, evidence: Any = None) -> None:
    scores.append({"name": name, "points": points, "earned": points if earned else 0, "passed": bool(earned), "evidence": evidence})


def partial(scores: list[dict[str, Any]], name: str, points: int, earned: int, evidence: Any = None) -> None:
    earned = max(0, min(points, earned))
    scores.append({"name": name, "points": points, "earned": earned, "passed": earned == points, "evidence": evidence})


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: score-live-benchmark-quality.py proof_dir output.json", file=sys.stderr)
        return 2

    proof_dir = Path(sys.argv[1])
    output_path = Path(sys.argv[2])
    conv_path = proof_dir / "full-benchmark-conversation.json"
    stream_path = proof_dir / "full-benchmark-stream.sse"
    checker_path = proof_dir / "full-benchmark-checker.json"
    workflow_path = proof_dir / "opencode-workflow-checker.json"
    browser_json_path = proof_dir / "full-benchmark-browser-proof.json"
    browser_png_path = proof_dir / "full-benchmark-webui.png"
    prompt_path = Path("scripts/smoke/full-agentic-benchmark-prompt.txt")

    conv = load_json(conv_path)
    events = parse_sse(stream_path)
    checker = load_json(checker_path)
    workflow = load_json(workflow_path)
    browser_json = load_json(browser_json_path) if browser_json_path.exists() else {}
    final = final_text(conv)
    results = all_tool_results(conv)
    stream_text = stream_path.read_text(encoding="utf-8", errors="replace") if stream_path.exists() else ""
    browser_text = json.dumps(browser_json, sort_keys=True)

    scores: list[dict[str, Any]] = []

    add(scores, "hard_checker_passed", 10, checker.get("passed") is True, checker.get("failed_checks"))
    add(scores, "opencode_workflow_checker_passed", 10, workflow.get("passed") is True, workflow.get("failed_checks"))
    add(scores, "nvidia_nim_only_with_model", 10, conv.get("provider") == "nvidia_nim" and isinstance(conv.get("model"), str) and bool(conv.get("model")) and "provider\":\"local" not in stream_text, {"provider": conv.get("provider"), "model": conv.get("model")})

    add(scores, "browser_proof_complete_and_useful", 10, browser_png_path.exists() and browser_json_path.exists() and all(marker in browser_text for marker in ["Full six-phase agentic benchmark prompt", "## Founder report", "## Technical report", "apply_patch", ".agent_test/repo_summary.md"]), {"png": browser_png_path.exists(), "json": browser_json_path.exists()})

    tool_events = stream_text.count("event: tool-call")
    tool_results = stream_text.count("event: tool-result")
    partial(scores, "long_tool_loop_depth", 10, min(10, tool_events // 3 + tool_results // 3), {"tool_call_events": tool_events, "tool_result_events": tool_results})

    labels_present = [label for label in REQUIRED_FINAL_LABELS if label in final]
    partial(scores, "final_markdown_contract", 10, int(10 * len(labels_present) / len(REQUIRED_FINAL_LABELS)), {"present": labels_present, "missing": sorted(set(REQUIRED_FINAL_LABELS) - set(labels_present))})

    founder = re.search(r"## Founder report\s*(.*?)(?:## Technical report|$)", final, re.I | re.S)
    founder_text = founder.group(1).strip() if founder else ""
    add(scores, "founder_report_concise_and_human_readable", 5, bool(founder_text) and 30 <= word_count(founder_text) <= 130 and not founder_text.lstrip().startswith("{"), {"word_count": word_count(founder_text)})

    add(scores, "phase3_file_claims_are_tool_proven", 10, all(ok_results(results, "FileWrite", path) and ok_results(results, "FileRead", path) for path in EXPECTED_FILES) and ok_results(results, "FileDelete", ".agent_test/investigation.md"), None)

    add(scores, "phase4_edit_claim_is_tool_proven", 10, bool([r for r in results if r.get("success") is True and r.get("kind") in {"FileEdit", "FileWrite", "ApplyPatch"} and not metadata_path(r).startswith(".agent_test/")]) and bool(ok_results(results, command_re=r"git diff|git status")), None)

    claimed_tests = re.findall(r"(?:cargo\s+(?:test|check|build|clippy|fmt)|bash\s+-n\s+[^\n`]+)", final, re.I)
    unproven = []
    for claim in claimed_tests:
        normalized = re.sub(r"\s+", " ", claim.strip()).lower()
        if not any(normalized in re.sub(r"\s+", " ", metadata_command(r)).lower() for r in results if r.get("success") is True):
            unproven.append(claim)
    add(scores, "test_and_build_claims_match_tool_commands", 10, not unproven, {"claimed_commands": claimed_tests, "unproven": unproven})

    add(scores, "semantic_repo_summary_not_placeholder", 5, all(term in final.lower() for term in ["forge", "webui", "benchmark", "risk"]) and "[" not in final and "]" not in final, None)

    add(scores, "proof_bundle_contains_prompt_transcript_checkers", 5, prompt_path.exists() and conv_path.exists() and stream_path.exists() and checker_path.exists() and workflow_path.exists(), None)

    total = sum(item["earned"] for item in scores)
    possible = sum(item["points"] for item in scores)
    percent = round(total * 100 / possible, 2) if possible else 0.0
    min_score = 85
    failed = [item for item in scores if not item["passed"] and item["points"] >= 10]
    passed = checker.get("passed") is True and workflow.get("passed") is True and percent >= min_score and not failed

    report = {
        "passed": passed,
        "score": total,
        "possible": possible,
        "percent": percent,
        "minimum_percent": min_score,
        "provider": conv.get("provider"),
        "model": conv.get("model"),
        "tool_results": len(results),
        "tool_call_events": tool_events,
        "tool_result_events": tool_results,
        "scores": scores,
        "failed_high_weight_checks": failed,
        "opencode_sources": ["packages/core/src/session/runner/max-steps.ts"],
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
