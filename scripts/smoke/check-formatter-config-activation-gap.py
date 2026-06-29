#!/usr/bin/env python3
"""Guard formatter config/dependency activation claims.

Forge has a source-backed formatter catalog, but OpenCode's formatter runtime also
activates several formatters through project config/dependency probes. Until Forge
has equivalent runtime probes, proof/state files must not claim that config-aware
formatter activation is implemented.
"""
from __future__ import annotations

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
FILE_OPS = ROOT / "crates/engine/src/tool/file_ops.rs"
PROOF_DIR = ROOT / "docs/generated/proof"
STATE_FILES = [ROOT / "PROJECT_STATE.md", ROOT / "FEATURE-AUDIT.md", ROOT / "README.md"]

OPENCODE_SOURCES = [
    "packages/opencode/src/format/index.ts",
    "packages/opencode/src/format/formatter.ts",
]
RUNTIME_MARKERS = [
    "activation_probe",
    "find_up",
    "package.json",
    "pyproject.toml",
    "biome.json",
    "composer.json",
    ".clang-format",
    ".ocamlformat",
]
OVERCLAIM_PATTERNS = [
    re.compile(r"config(?:uration)?[- ]aware formatter activation (?:is )?(?:implemented|complete|done|green)", re.I),
    re.compile(r"dependency[- ]aware formatter activation (?:is )?(?:implemented|complete|done|green)", re.I),
    re.compile(r"full runtime formatter activation parity", re.I),
]


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        return ""


def has_runtime_activation_markers(file_ops: str) -> bool:
    return any(marker in file_ops for marker in RUNTIME_MARKERS)


def documents() -> list[pathlib.Path]:
    docs = [path for path in STATE_FILES if path.exists()]
    if PROOF_DIR.exists():
        docs.extend(sorted(PROOF_DIR.glob("*.md"))[-40:])
    return docs


def main() -> int:
    file_ops = read(FILE_OPS)
    missing_sources = [source for source in OPENCODE_SOURCES if source not in file_ops]
    if missing_sources:
        print(f"missing OpenCode formatter source anchors in file_ops.rs: {missing_sources}", file=sys.stderr)
        return 1

    runtime_supported = has_runtime_activation_markers(file_ops)
    violations: list[str] = []
    for path in documents():
        text = read(path)
        for pattern in OVERCLAIM_PATTERNS:
            if pattern.search(text) and not runtime_supported:
                violations.append(f"{path.relative_to(ROOT)} matches {pattern.pattern}")

    if violations:
        print("formatter activation overclaim detected before runtime config/dependency probes exist:", file=sys.stderr)
        for violation in violations:
            print(f"- {violation}", file=sys.stderr)
        return 1

    print("formatter config/dependency activation claims are guarded; runtime gap remains explicit")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
