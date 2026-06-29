#!/usr/bin/env python3
"""Guard Forge max-step/evidence-ready finalization against OpenCode source semantics.

OpenCode source anchor:
- anomalyco/opencode:packages/core/src/session/runner/max-steps.ts

This checker is deliberately static and deterministic for CI. The live WebUI workflow
still owns provider/browser proof; this gate prevents the Forge finalization prompt
from silently regressing back into tool calls, unsupported success claims, or blank
JSON-like final output.
"""

from __future__ import annotations

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
ORCHESTRATOR = ROOT / "crates/engine/src/orchestrator.rs"
PROOF_DIR = ROOT / "docs/generated/proof"
OPENCODE_SOURCE = "packages/core/src/session/runner/max-steps.ts"


def require(name: str, condition: bool) -> None:
    if not condition:
        raise SystemExit(f"max-step finalization parity check failed: {name}")


def main() -> int:
    source = ORCHESTRATOR.read_text(encoding="utf-8")
    lower = source.lower()

    require("OpenCode max-step source path recorded", OPENCODE_SOURCE in source)
    require("finalizer disables tools in ChatRequest", "tools: none" in lower and "tool_choice: none" in lower)
    require("system prompt says tools are disabled", "tools are disabled" in lower)
    require("system prompt requires summary of work done", "summarize work done so far" in lower)
    require("final prompt blocks unproven file/test/build claims", "do not claim tests, builds, file operations, or fixes succeeded" in lower)
    require("final prompt requires Founder report", "founder report" in lower)
    require("final prompt requires Technical report", "technical report" in lower)
    require("final prompt requires confidence labels", all(label in source for label in ("VERIFIED", "LIKELY", "UNKNOWN")))
    require("fallback final report exists", "fn fallback_final_report" in source)
    require(
        "fallback report is conservative",
        "Only successful tool results in the digest are counted" in source
        or "Only successful tool results listed in the evidence digest are counted" in source,
    )
    require("fallback report includes rollback strategy", "Rollback strategy" in source)
    require("fallback report includes unresolved risks", "unresolved risks" in source)
    require("benchmark evidence-ready finalization exists", "benchmark_evidence_ready" in source and "forge_evidence_ready_finalized" in source)

    proof_hits = []
    if PROOF_DIR.exists():
        for path in PROOF_DIR.glob("*.md"):
            text = path.read_text(encoding="utf-8", errors="replace")
            if OPENCODE_SOURCE in text and "max-step" in text.lower():
                proof_hits.append(path)
    require("proof trail records max-step OpenCode source", bool(proof_hits))

    print("max-step finalization parity: ok")
    print(f"source={ORCHESTRATOR}")
    print(f"opencode_source={OPENCODE_SOURCE}")
    print("proof_docs=" + ",".join(str(path.relative_to(ROOT)) for path in proof_hits[:5]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
