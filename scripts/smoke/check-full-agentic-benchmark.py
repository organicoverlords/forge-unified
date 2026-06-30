#!/usr/bin/env python3
"""Validate the full six-phase WebUI benchmark from recorded artifacts.

The checker trusts tool-result/SSE evidence over final prose. It only requires
matching cargo evidence when the final answer makes a non-negated success claim
for that exact cargo command.
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

AGENT_TEST = ".agent_test"
EXPECTED_AGENT_FILES = {
    f"{AGENT_TEST}/repo_summary.md",
    f"{AGENT_TEST}/investigation.md",
    f"{AGENT_TEST}/action_plan.json",
}


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def parse_sse(path: Path) -> list[dict[str, Any]]:
    text = path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""
    events: list[dict[str, Any]] = []
    event_name = "message"
    data_lines: list[str] = []
    for raw in text.splitlines() + [""]:
        line = raw.rstrip("\n")
        if not line:
            if data_lines:
                payload = "\n".join(data_lines)
                try:
                    data: Any = json.loads(payload)
                except json.JSONDecodeError:
                    data = payload
                events.append({"event": event_name, "data": data})
            event_name = "message"
            data_lines = []
            continue
        if line.startswith("event: "):
            event_name = line[len("event: ") :]
        elif line.startswith("data: "):
            data_lines.append(line[len("data: ") :])
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


def final_assistant_text(conv: dict[str, Any]) -> str:
    for msg in reversed(conv.get("messages", [])):
        if msg.get("role") == "Assistant":
            return msg.get("content") or ""
    return ""


def result_path(result: dict[str, Any]) -> str:
    return str((result.get("metadata") or {}).get("path") or "")


def result_command(result: dict[str, Any]) -> str:
    return str((result.get("metadata") or {}).get("command") or "")


def result_output(result: dict[str, Any]) -> str:
    return str(result.get("output") or "")


def ok_result(
    results: list[dict[str, Any]],
    *,
    kind: str | None = None,
    path: str | None = None,
    command_re: str | None = None,
    output_re: str | None = None,
) -> list[dict[str, Any]]:
    found: list[dict[str, Any]] = []
    for result in results:
        if result.get("success") is not True:
            continue
        if kind is not None and result.get("kind") != kind:
            continue
        if path is not None and result_path(result) != path:
            continue
        if command_re is not None and not re.search(command_re, result_command(result), re.I):
            continue
        if output_re is not None and not re.search(output_re, result_output(result), re.I | re.S):
            continue
        found.append(result)
    return found


def count_words(text: str) -> int:
    return len(re.findall(r"\b[\w'-]+\b", text))


def founder_section(text: str) -> str:
    match = re.search(r"founder report\s*(.*?)(technical report|$)", text, re.I | re.S)
    return match.group(1) if match else ""


def add(checks: list[dict[str, Any]], name: str, passed: bool, evidence: Any = None) -> None:
    checks.append({"name": name, "passed": bool(passed), "evidence": evidence})


def final_has_required_summary_labels(final: str) -> bool:
    required_patterns = [
        r"files created",
        r"files (deleted|removed)",
        r"files modified",
        r"tests run",
        r"unresolved risks",
        r"confidence\s*\(?0?[-–]100\)?",
    ]
    return all(re.search(pattern, final, re.I) for pattern in required_patterns)


def has_actual_event(events: list[dict[str, Any]], name: str) -> bool:
    return any(event.get("event") == name for event in events)


def has_provider_local_event(events: list[dict[str, Any]]) -> bool:
    for event in events:
        data = event.get("data")
        if isinstance(data, dict) and data.get("provider") == "local":
            return True
    return False


def prose_for_claim_detection(line: str) -> str:
    """Normalize Markdown/code punctuation before claim/negation matching."""
    text = re.sub(r"[`*_]", "", line.lower())
    text = re.sub(r"\s+", " ", text)
    return text


def non_negated_claim_line(line: str) -> bool:
    lower = prose_for_claim_detection(line)
    negations = [
        "not run",
        "not executed",
        "was not run",
        "were not run",
        "no cargo build",
        "no cargo check",
        "no cargo test",
        "cargo test was not",
        "cargo check was not",
        "cargo build was not",
        "not required",
        "unknown whether",
        "next logical step",
        "would be to run",
    ]
    return not any(negation in lower for negation in negations)


def success_claim(line: str) -> bool:
    return bool(re.search(r"\b(pass(?:ed|es)?|success(?:ful|fully)?|green|succeeds?)\b", line, re.I))


def claims_cargo_tests_success(final: str) -> bool:
    for line in final.splitlines():
        if not non_negated_claim_line(line):
            continue
        if re.search(r"\ball\s+(cargo\s+)?tests\s+pass(?:ed|es)?\b", line, re.I):
            return True
        if re.search(r"\bcargo\s+test\b", line, re.I) and success_claim(line):
            return True
    return False


def claims_build_check_success(final: str) -> bool:
    for line in final.splitlines():
        if not non_negated_claim_line(line):
            continue
        if re.search(r"\bcargo\s+(?:check|build)\b", line, re.I) and success_claim(line):
            return True
    return False


def main() -> int:
    if len(sys.argv) != 4:
        print("usage: check-full-agentic-benchmark.py conversation.json stream.sse output.json", file=sys.stderr)
        return 2

    conversation_path = Path(sys.argv[1])
    stream_path = Path(sys.argv[2])
    output_path = Path(sys.argv[3])
    conv = load_json(conversation_path)
    events = parse_sse(stream_path)
    results = all_tool_results(conv)
    final = final_assistant_text(conv)
    stream_text = stream_path.read_text(encoding="utf-8", errors="replace") if stream_path.exists() else ""
    run_finish = [event.get("data") for event in events if event.get("event") == "run-finish" and isinstance(event.get("data"), dict)]
    last_run = run_finish[-1] if run_finish else {}

    checks: list[dict[str, Any]] = []
    add(checks, "provider_is_real_nvidia_nim", conv.get("provider") == "nvidia_nim", {"provider": conv.get("provider"), "model": conv.get("model")})
    add(checks, "model_is_recorded", isinstance(conv.get("model"), str) and len(conv.get("model")) > 0, conv.get("model"))
    add(checks, "no_local_or_scripted_shortcut", not has_provider_local_event(events) and not has_actual_event(events, "benchmark-phase") and not re.search(r'"local_shortcut"\s*:\s*true', stream_text), None)
    add(checks, "run_finished_without_tool_cap_forced_final", last_run.get("metadata", {}).get("forced_final_after_tool_cap") is not True, last_run.get("metadata"))
    add(checks, "tool_calls_and_results_present", stream_text.count("event: tool-call") >= 8 and stream_text.count("event: tool-result") >= 8, {"tool_call_events": stream_text.count("event: tool-call"), "tool_result_events": stream_text.count("event: tool-result")})

    repo_shell = ok_result(results, kind="ShellCommand", command_re=r"pwd|rev-parse|git status|git remote|git log|branch")
    size_shell = ok_result(results, kind="ShellCommand", command_re=r"du\s|find .*-exec du|wc -l|largest|sort")
    build_config_reads = ok_result(results, kind="FileRead", path="Cargo.toml")
    repo_map = ok_result(results, kind="FileList") or ok_result(results, kind="RepoInfo")
    add(checks, "phase1_repo_identity_evidence", bool(repo_shell), [result_command(r) for r in repo_shell[:2]])
    add(checks, "phase1_size_or_largest_evidence", bool(size_shell), [result_command(r) for r in size_shell[:3]])
    add(checks, "phase1_build_config_and_repo_map_evidence", bool(build_config_reads) and bool(repo_map), {"cargo_reads": len(build_config_reads), "repo_map_results": len(repo_map)})

    first_file_write_idx = next((i for i, r in enumerate(results) if r.get("kind") == "FileWrite" and result_path(r).startswith(f"{AGENT_TEST}/")), len(results))
    pre_file_success = [r for r in results[:first_file_write_idx] if r.get("success") is True and r.get("kind") in {"ShellCommand", "FileRead", "FileList", "FileSearch", "RepoInfo"}]
    add(checks, "phase2_long_tool_loop_has_at_least_8_pre_file_tool_results", len(pre_file_success) >= 8, {"count": len(pre_file_success)})
    add(checks, "phase2_used_code_config_search_and_test_evidence", bool(ok_result(results, kind="FileRead")) and bool(ok_result(results, kind="FileSearch") or ok_result(results, kind="ShellCommand", command_re=r"grep|rg|find")) and bool(ok_result(results, kind="ShellCommand", command_re=r"cargo check|cargo test|bash -n|git diff|git status")), None)
    add(checks, "phase2_confidence_labels_in_answer", all(label in final for label in ["VERIFIED", "LIKELY", "UNKNOWN"]), None)

    for path in sorted(EXPECTED_AGENT_FILES):
        add(checks, f"phase3_file_write_{path}", bool(ok_result(results, kind="FileWrite", path=path)), None)
        add(checks, f"phase3_file_read_{path}", bool(ok_result(results, kind="FileRead", path=path)), None)
    deletes = [r for r in results if r.get("kind") == "FileDelete" and r.get("success") is True]
    deleted_paths = [result_path(r) for r in deletes]
    add(checks, "phase3_deleted_only_investigation_md", deleted_paths == [f"{AGENT_TEST}/investigation.md"], deleted_paths)
    verify_after_delete = any(r.get("success") is True and r.get("kind") in {"FileList", "ShellCommand"} and f"{AGENT_TEST}/repo_summary.md" in result_output(r) and f"{AGENT_TEST}/action_plan.json" in result_output(r) and f"{AGENT_TEST}/investigation.md" not in result_output(r) for r in results)
    add(checks, "phase3_verified_remaining_files_after_delete", verify_after_delete, None)

    real_edits = [r for r in results if r.get("success") is True and r.get("kind") in {"FileEdit", "FileWrite", "ApplyPatch"} and not result_path(r).startswith(f"{AGENT_TEST}/")]
    diff_or_status = ok_result(results, kind="ShellCommand", command_re=r"git diff|git status")
    validation = ok_result(results, kind="ShellCommand", command_re=r"cargo check|cargo test|bash -n|cargo build|cargo fmt|cargo clippy")
    add(checks, "phase4_real_low_risk_edit", bool(real_edits), [{"kind": r.get("kind"), "path": result_path(r)} for r in real_edits])
    add(checks, "phase4_diff_or_status_inspected", bool(diff_or_status), [result_command(r) for r in diff_or_status])
    add(checks, "phase4_validation_command_succeeded", bool(validation), [result_command(r) for r in validation])
    add(checks, "phase4_risk_and_rollback_in_answer", all(re.search(term, final, re.I) for term in ["blast radius", "difficulty", "rollback"]), None)

    founder = founder_section(final)
    add(checks, "phase5_founder_report_present_and_under_180_words", bool(founder.strip()) and count_words(founder) <= 180, {"word_count": count_words(founder)})
    add(checks, "phase5_technical_report_has_required_sections", all(re.search(term, final, re.I) for term in ["Technical report", "evidence", "assumptions", "failed hypoth", "confidence", "rollback"]), None)

    if claims_cargo_tests_success(final):
        add(checks, "claimed_cargo_tests_are_tool_proven", bool(ok_result(results, kind="ShellCommand", command_re=r"cargo test")), None)
    if claims_build_check_success(final):
        add(checks, "claimed_build_check_is_tool_proven", bool(ok_result(results, kind="ShellCommand", command_re=r"cargo check|cargo build")), None)

    cleanup_shell = ok_result(results, kind="ShellCommand", command_re=r"git status|find .*agent_test|ls .*agent_test|grep -R .*SECRET|grep -R .*TOKEN")
    add(checks, "phase6_cleanup_or_state_check_present", bool(cleanup_shell), [result_command(r) for r in cleanup_shell])
    add(checks, "final_reports_files_tests_risks_confidence", final_has_required_summary_labels(final), None)

    failed = [check for check in checks if not check["passed"]]
    report = {
        "passed": not failed,
        "conversation": str(conversation_path),
        "stream": str(stream_path),
        "provider": conv.get("provider"),
        "model": conv.get("model"),
        "tool_results": len(results),
        "tool_call_events": stream_text.count("event: tool-call"),
        "tool_result_events": stream_text.count("event: tool-result"),
        "checks": checks,
        "failed_checks": failed,
    }
    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
