# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous same-head green proof: all required workflows passed on `f0d1fd6818b67f8d41d3885ed616002e90c5868b`, including Live WebUI Feature Sprint artifact `7972230611`.
- Current selected baseline before this slice: `f0d1fd6818b67f8d41d3885ed616002e90c5868b`.
- Latest inspected state on that baseline: CI `28423428951`, Build Proof `28423428937`, App Build Proof `28423428922`, App Multistep Build Proof `28423428921`, Fast WebUI Proof `28423428944`, and Live WebUI Feature Sprint `28423428936` all completed successfully.
- Latest implementation/proof slice: WebUI typed tool cards now expose OpenCode-backed evidence controls that separate target, public input, diagnostics, and result output; input is collapsed behind a `show input` toggle, and browser-visible copy controls include `copy target` and `copy diagnostics`.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_enhancements.html` to add tool evidence controls inside typed tool cards: `show input`, `copy target`, `copy diagnostics`, retained `show result`, `copy result`, and `copy input`.
- Added UI proof markers `tool-input-toggle`, `tool-diagnostic-copy`, and `tool-target-copy` so browser proof can verify the controls are present without exposing raw tool JSON as the primary evidence.
- Added proof doc `docs/generated/proof/webui-tool-evidence-controls-20260630T0650Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, `stable-session-receipts`, `typed-tool-renderer`, `tool-target-visible`, `tool-result-toggle`, `tool-args-visible`, `tool-diagnostics-visible`, `tool-count-summary-visible`, `tool-input-toggle`, `tool-diagnostic-copy`, `tool-target-copy`, `tool-lifecycle-strip`, `tool-state-timeline`, `copy-tool-anchor`, `tool-output-duration-visible`, `tool-receipt-id-visible`, and `copy-tool-receipt`.
- This slice improves WebUI evidence usability and screenshot verifiability. It does not claim full parity, production readiness, backend fork/revert/session checkpoint semantics, or same-head acceptance.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, input toggle, diagnostic copy, target copy.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, and `crates/webui/src/chat_ui_tool_lifecycle.html`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### OpenCode source backing for latest WebUI evidence-controls slice

- `anomalyco/opencode:packages/web/src/components/share/part.tsx` uses per-tool rendering for read/write/edit/bash/grep/glob/task, target labels, collapsible `ResultsButton`, diagnostics formatting, and `flattenToolArgs` fallback argument display.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css` backs the share/session part visual contract and is retained as the CSS/layout source anchor.
- Forge implementation path: `crates/webui/src/chat_ui_enhancements.html`.
