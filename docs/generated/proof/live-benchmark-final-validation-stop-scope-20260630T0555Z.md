# Live benchmark final validation stop-scope repair — 2026-06-30T05:55Z

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Baseline head inspected before this slice: `84e1cf6e34647e19856cffb2ffd4fbbe425a8a86`
- Previous same-head green baseline: `a9e2fa3ed068da87cc3636f673ad33e6c8fa7a53`, Live WebUI proof artifact `7970696828`.

## Failed workflow evidence inspected

- CI `28421009540` failed `Smoke Test` job `84213953574` because `scripts/smoke/check-opencode-tool-lifecycle-contract.py` looked only in `PROJECT_STATE.md` and reported missing lifecycle source anchors for:
  - `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - `ToolPart`
- Live WebUI Feature Sprint `28421009551` failed job `84213953611` after a real NVIDIA NIM run with provider `nvidia_nim` and model `deepseek-ai/deepseek-v4-flash`. The failed checks were final-report and Phase 4 completion checks, including missing `phase4_real_low_risk_edit`, `phase4_risk_and_rollback_in_answer`, founder/technical report sections, and final summary labels.

## OpenCode source backing used

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`
  - Tools are disabled at a maximum-step boundary and the assistant must respond with text only.
  - Response must summarize work done, remaining tasks, and next recommendations.
- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - Tool calls are tracked by `toolCallID` and updated/completed/finalized through durable `ToolPart` state changes.
- `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - `ToolPart` is modeled with `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, timing, metadata, output/error, and attachments.

Reference only; Forge browser/runtime proof must not expose upstream branding as user-visible app identity.

## Forge paths changed

- `scripts/smoke/check-opencode-tool-lifecycle-contract.py`
- `scripts/smoke/full-agentic-benchmark-prompt.txt`
- `PROJECT_STATE.md`
- `docs/generated/proof/live-benchmark-final-validation-stop-scope-20260630T0555Z.md`

## Feature / proofability slice built

This slice fixes a real proof reliability gap, not a docs-only/cosmetic issue:

1. The lifecycle contract gate now reads durable proof memory from both `PROJECT_STATE.md` and `docs/generated/proof/*.md`, matching the repo’s existing proof-trail pattern and preventing false failures when exact OpenCode anchors are recorded in proof docs.
2. The full benchmark prompt now scopes OpenCode-style max-step finalization to the final Phase 4 validation shell result from step 10. The earlier Phase 2 `bash -n` command inside `batch_parallel` is explicitly marked as inspection evidence, not the stop point.

## Acceptance boundary

Do not claim latest-head acceptance until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact post-slice head and the WebUI proof artifact/screenshots are inspected.
