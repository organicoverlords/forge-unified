# Max-step command-claim scorer tightening proof — 2026-06-29T11:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft/mergeable at inspection time
- Inspected head before this slice: `64892c27b403b80a019cf410228bad9660defaa0`

## Failed workflows inspected

- CI `28368200120`: failed in `Smoke Test` / `Validate WebUI proof harness`.
- Build Proof `28368200124`: success.
- Live WebUI Feature Sprint `28368200123`: failed.

Observed failure details:

- CI max-step parity failed on `fallback report includes rollback strategy`; the source already contained lowercase `rollback strategy`, so the gate was case-sensitive rather than semantic.
- Live WebUI preserved real NVIDIA NIM evidence with provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, 19 tool-call events, 19 tool-result events, passing hard benchmark checker, and passing OpenCode workflow checker.
- Live WebUI still failed because the stream did not finish before the outer timeout and quality scoring still treated negative or human-readable command mentions such as `cargo build` and `bash -n ...` as unproven command claims.

## OpenCode source backing

Exact upstream source inspected:

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant source semantics:

- Maximum-step finalization disables tools.
- The final response must be text-only.
- The final response must summarize completed work, list remaining tasks, and recommend next steps.
- Tool use after max steps is a violation.

Forge-specific implication:

- CI and live proof scoring should enforce the no-tools / text-only / evidence-bound finalization contract without false-failing equivalent heading case or negative command disclaimers.

## Code changes

- `scripts/smoke/check-max-step-finalization-parity.py`
  - Changed the rollback-strategy check to use the already-computed lowercase source text.
  - Preserves the OpenCode-backed requirements for no-tools finalization, summary of work done, final headings, confidence labels, conservative evidence claims, and proof docs.

- `scripts/smoke/score-live-benchmark-quality.py`
  - Added a shared negative-command context detector.
  - Tightened `bash -n` command claim extraction so generic prose like `bash -n validation passed` is not treated as an executable command claim.
  - Normalized command claims and tool metadata more conservatively.
  - Deduplicated command claims before scoring.

## Proof status

This slice is a source-backed scorer/harness fix. It does not itself prove latest-head WebUI completion.

Required next proof before claiming latest-head pass:

- CI success on the new head.
- Build Proof success on the new head.
- Live WebUI Feature Sprint success on the new head.
- Live artifact containing natural-language prompt, stream/transcript, checker JSON, workflow JSON, quality-score JSON, conversation JSON, and browser PNG screenshot proof using NVIDIA NIM only.
