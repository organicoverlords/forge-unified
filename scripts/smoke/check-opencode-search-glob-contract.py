#!/usr/bin/env python3
"""Gate OpenCode-backed search/glob tool contract evidence.

This is intentionally deterministic CI smoke coverage. The live WebUI workflow
proves behavior through NVIDIA NIM; this gate keeps the source/proof contract from
regressing between expensive live runs.
"""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FILE_OPS = ROOT / "crates/engine/src/tool/file_ops.rs"
PROJECT_STATE = ROOT / "PROJECT_STATE.md"
PROOF_DIR = ROOT / "docs/generated/proof"

REQUIRED_SOURCE_TOKENS = [
    "execute_file_glob",
    "execute_file_search",
    "pattern",
    "count",
    "matches",
    "forge_search_glob_contract",
    "packages/opencode/src/tool/glob.ts",
    "packages/opencode/src/tool/grep.ts",
]

REQUIRED_OUTPUT_CONTRACT_TOKENS = [
    "No files found",
    "Results are truncated",
    "Found {total} matches",
    "Line {line}",
    "truncated",
]

REQUIRED_PROOF_TOKENS = [
    "packages/opencode/src/tool/glob.ts",
    "packages/opencode/src/tool/grep.ts",
    "result count metadata",
    "No files found",
    "path resolution",
    "bounded output",
    "human-readable output",
]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def main() -> int:
    source = read(FILE_OPS)
    missing_source = [token for token in REQUIRED_SOURCE_TOKENS if token not in source]
    if missing_source:
        print("Missing file_ops search/glob source tokens:")
        for token in missing_source:
            print(f"- {token}")
        return 1

    missing_output_contract = [token for token in REQUIRED_OUTPUT_CONTRACT_TOKENS if token not in source]
    if missing_output_contract:
        print("Missing OpenCode-style search/glob output contract tokens in file_ops:")
        for token in missing_output_contract:
            print(f"- {token}")
        return 1

    proof_text = read(PROJECT_STATE)
    if PROOF_DIR.exists():
        for path in sorted(PROOF_DIR.glob("*.md")):
            proof_text += "\n" + read(path)

    missing_proof = [token for token in REQUIRED_PROOF_TOKENS if token not in proof_text]
    if missing_proof:
        print("Missing OpenCode search/glob proof tokens:")
        for token in missing_proof:
            print(f"- {token}")
        return 1

    print("OpenCode search/glob contract evidence is present.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
