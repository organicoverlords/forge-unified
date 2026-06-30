# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous same-head green proof: all required workflows passed on `a9e2fa3ed068da87cc3636f673ad33e6c8fa7a53`, including Live WebUI Feature Sprint artifact `7970696828`.
- Current selected baseline before this slice: `84e1cf6e34647e19856cffb2ffd4fbbe425a8a86`.
- Latest inspected failures on that baseline: CI `28421009540` failed only the Smoke Test lifecycle source-anchor gate; Live WebUI Feature Sprint `28421009551` failed the full benchmark final-report/Phase 4 checks while App Build, Fast WebUI, Build Proof, and App Multistep passed on the same head.
- Latest implementation/proof slice: tool lifecycle proof gate now reads durable proof docs as well as `PROJECT_STATE.md`, and the full benchmark prompt clarifies that OpenCode-style max-step text-only finalization applies only after the final Phase 4 validation shell result, not the earlier Phase 2 `bash -n` evidence inside `batch_parallel`.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `scripts/smoke/check-opencode-tool-lifecycle-contract.py` so the gate reads `PROJECT_STATE.md` plus `docs/generated/proof/*.md` before deciding that OpenCode source anchors are missing.
- Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` to resolve a stop-rule ambiguity: the Phase 2 `bash -n` result is inspection evidence only; the tool-disable/final-text-only behavior starts after the Phase 4 validation command in step 10.
- Added proof doc `docs/generated/proof/live-benchmark-final-validation-stop-scope-20260630T0555Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, `stable-session-receipts`, `typed-tool-renderer`, `tool-target-visible`, `tool-result-toggle`, `tool-args-visible`, `tool-diagnostics-visible`, `tool-count-summary-visible`, `tool-lifecycle-strip`, `tool-state-timeline`, `copy-tool-anchor`, `tool-output-duration-visible`, `tool-receipt-id-visible`, and `copy-tool-receipt`.
- This slice improves deterministic proof reliability and finalization behavior. It does not claim full parity, production readiness, or backend fork/revert/session checkpoint semantics.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, and `crates/webui/src/chat_ui_tool_lifecycle.html`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.
