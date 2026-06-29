#!/usr/bin/env python3
"""Guard formatter activation evidence against upstream OpenCode source anchors.

This checker is intentionally deterministic: it does not execute external
formatters. It prevents Forge's formatter parity proof trail from dropping the
OpenCode config/dependency activation source backing while the runtime formatter
catalog remains Forge-owned and free of upstream-branded metadata keys.
"""
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FILE_OPS = ROOT / "crates" / "engine" / "src" / "tool" / "file_ops.rs"
STATE = ROOT / "PROJECT_STATE.md"

REQUIRED_FILE_OPS_SNIPPETS = [
    "packages/opencode/src/format/index.ts",
    "packages/opencode/src/format/formatter.ts",
    "formatter_unavailable",
    "formatter_failed_contained",
    "spawn_failed_contained",
    "forge_formatter_contract",
    "forge_file_tool_contract",
]

REQUIRED_STATE_SNIPPETS = [
    "configuration/dependency-aware formatter activation",
    "formatter service, extension matching, command probing/caching, contained formatter execution, status shape, and configuration/dependency-aware formatter activation",
    "built-in formatter catalog, representative extensions, command semantics, and config/dependency-aware formatter enablement",
]

UPSTREAM_ACTIVATION_SNIPPETS = [
    "config.get()",
    "cfg.formatter",
    "Filesystem.findUp",
    "mergeDeep",
    "Format.status",
]


def main() -> int:
    file_ops = FILE_OPS.read_text(encoding="utf-8")
    state = STATE.read_text(encoding="utf-8")
    errors: list[str] = []

    for snippet in REQUIRED_FILE_OPS_SNIPPETS:
        if snippet not in file_ops:
            errors.append(f"file_ops missing formatter proof snippet: {snippet}")

    for snippet in REQUIRED_STATE_SNIPPETS:
        if snippet not in state:
            errors.append(f"PROJECT_STATE missing formatter activation evidence: {snippet}")

    # These exact strings are upstream source semantics that must remain in the
    # proof checker so reviewers can see which OpenCode activation paths were
    # inspected without leaking opencode_* runtime metadata into Forge tools.
    checker_text = Path(__file__).read_text(encoding="utf-8")
    for snippet in UPSTREAM_ACTIVATION_SNIPPETS:
        if snippet not in checker_text:
            errors.append(f"checker missing upstream activation anchor: {snippet}")

    if "opencode_" in file_ops:
        errors.append("Forge runtime formatter metadata must not contain opencode_* keys")

    if errors:
        print("formatter activation evidence check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(
        "formatter activation evidence check passed: OpenCode formatter activation "
        "source anchors remain recorded while Forge runtime metadata stays Forge-owned."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
