# Live WebUI nested batch finalization proof note

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Source inspected first

- Upstream OpenCode source: `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Relevant behavior: when maximum steps are reached, tools are disabled and the agent must respond with text only, summarize completed work, list remaining tasks, and recommend next steps.

## Failed run inspected before patch

- Head: `9e1bab911ff13ec599fe3fc068fa3bc08dacf5c0`.
- Build Proof `28388644197`: success.
- CI `28388644209`: failure in `Smoke Test` / `Validate WebUI proof harness`.
- Live WebUI Feature Sprint `28388644200`: failure.
- Live job `84109733346` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- Live artifact: `7959103317`, `live-webui-feature-sprint-proof`, digest `sha256:dd29545f142a0cbec4ae2f2d75df89282c1b728da12f69db3c17ef3978fdea6d`.

## Patch

`crates/engine/src/orchestrator.rs` now uses a shared flattened tool-result view for benchmark finalization and final evidence reporting:

- Direct conversation tool results are preserved.
- Nested `BatchParallel` child results are parsed from JSON output and recursively included.
- `benchmark_evidence_ready()` now sees the same nested batch evidence class as the hard checker.
- `final_evidence_digest()` now includes nested results so fallback/final Markdown is evidence-bound to the full tool ledger.

## Expected improvement

The live WebUI benchmark should stop sooner after the real evidence-completing tool result instead of continuing to max rounds because engine finalization missed proof that was nested inside `batch_parallel`.

## Verification required

Do not mark the latest head proven until CI, Build Proof, and Live WebUI Feature Sprint are all green on the latest PR head and the Live artifact includes browser screenshot proof plus NVIDIA NIM provider/model/tool evidence.
