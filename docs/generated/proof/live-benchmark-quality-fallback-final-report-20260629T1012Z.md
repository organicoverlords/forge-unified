# Live benchmark quality fallback final report fix

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Repository truth

- Repo: `organicoverlords/forge-unified`
- Previous head inspected: `a013b616aac1b390acdc99c97b618a7c7d2cc609`
- New source-fix head: `d7a7067b542af384a087093375e4318170ee41f7`
- Previous Live WebUI Feature Sprint run: `28363463759`
- Previous Live WebUI artifact: `7948375599`, digest `sha256:c191e7bad52c8f1978841ceb9e6492fe714bca59a6b3c9621f76fa58c03ad5af`

## Failure inspected before patching

The latest same-head workflow status for `a013b616aac1b390acdc99c97b618a7c7d2cc609` was:

- CI `28363463733`: success.
- Build Proof `28363463678`: success.
- Live WebUI Feature Sprint `28363463759`: failure.

The Live WebUI job showed:

- `Run live WebUI feature sprint`: success.
- `Check full benchmark evidence and quality score`: failure.
- `Upload feature sprint proof`: success.

This means the app run itself completed and uploaded proof, but the quality gate rejected the proof.

## OpenCode source inspected first

Source anchor:

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant behavior:

- Tools are disabled after max steps.
- Final response must be text only.
- Final response must summarize work done so far.
- Final response must include remaining tasks.
- Final response must recommend what to do next.

## Source-backed diagnosis

Forge already had evidence-ready finalization, but `looks_like_final_report` was too weak. A model response could contain `Founder report` and `Technical report` while missing the exact quality-score contract. The fallback final report also did not use the exact required Markdown headings and lowercase labels checked by `scripts/smoke/score-live-benchmark-quality.py`.

That made the live benchmark fragile: real NIM tool evidence could exist, the hard checker could pass, and the final quality step could still fail because the final answer text was not reviewer-useful enough.

## Patch

Changed `crates/engine/src/orchestrator.rs`:

- Tightened `looks_like_final_report` so weak model final text no longer passes just because it mentions founder and technical reports.
- Required exact final-report labels before accepting model final text.
- Rejected JSON-looking final text and square-bracket placeholder text.
- Updated the no-tools finalization prompt to explicitly include OpenCode max-step behaviors: summary, remaining tasks, and next recommendations.
- Rewrote the fallback final report to emit exact scorer-compatible Markdown:
  - `## Founder report`
  - `## Technical report`
  - `evidence`
  - `assumptions`
  - `failed hypotheses`
  - `rollback strategy`
  - `blast radius`
  - `implementation difficulty`
  - `rollback difficulty`
  - `files created`
  - `files removed`
  - `files modified`
  - `tests run`
  - `unresolved risks`
  - `confidence (0-100)`
  - `VERIFIED`, `LIKELY`, and `UNKNOWN`
- Kept claims evidence-bound so no cargo/build/test/file success is claimed unless present in tool metadata.

## Status

This is a source fix for the failed quality gate. It is not yet same-head proven until CI, Build Proof, and Live WebUI Feature Sprint complete on `d7a7067b542af384a087093375e4318170ee41f7` or a later head containing this change.
