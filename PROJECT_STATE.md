# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous same-head green proof: all required workflows passed on `bee2dfe3530f8a3b854430c13ad528bfda436c11`, including Live WebUI Feature Sprint artifact `7973188948`.
- Current selected baseline before this slice: `bee2dfe3530f8a3b854430c13ad528bfda436c11`.
- Latest inspected state on that baseline: CI `28426015609`, Build Proof `28426015594`, App Build Proof `28426015585`, App Multistep Build Proof `28426015598`, Fast WebUI Proof `28426015591`, and Live WebUI Feature Sprint `28426015584` all completed successfully.
- Latest implementation/proof slice: WebUI typed tool cards now expose an OpenCode-backed preview pane with `show preview` and `copy preview`, separate from `show input`, `show result`, `copy target`, `copy result`, `copy input`, and `copy diagnostics`.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_enhancements.html` to add `previewFrom(...)`, `.tool-preview-pane`, and browser-visible `show preview` / `copy preview` controls for read/write/edit/bash/grep/glob/task/tool output evidence.
- Updated `scripts/smoke/check-webui-proof-part-contract.py` so CI requires `tool-preview-visible`, `tool-preview-toggle`, and `tool-preview-copy` proof markers plus durable proof-trail tokens for preview pane, preview toggle, and copy preview.
- Added proof doc `docs/generated/proof/webui-tool-preview-evidence-controls-20260630T0755Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, `stable-session-receipts`, `typed-tool-renderer`, `tool-target-visible`, `tool-result-toggle`, `tool-args-visible`, `tool-diagnostics-visible`, `tool-count-summary-visible`, `tool-input-toggle`, `tool-diagnostic-copy`, `tool-target-copy`, `tool-preview-visible`, `tool-preview-toggle`, `tool-preview-copy`, `tool-lifecycle-strip`, `tool-state-timeline`, `copy-tool-anchor`, `tool-output-duration-visible`, `tool-receipt-id-visible`, and `copy-tool-receipt`.
- This slice improves WebUI evidence usability and screenshot verifiability. It does not claim full parity, production readiness, backend fork/revert/session checkpoint semantics, or same-head acceptance.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, input toggle, diagnostic copy, target copy, preview pane, preview toggle, copy preview.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, and `crates/webui/src/chat_ui_tool_lifecycle.html`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### OpenCode source backing for latest WebUI preview-controls slice

- `anomalyco/opencode:packages/web/src/components/share/part.tsx` uses per-tool renderers for read/write/edit/bash/grep/glob/task, target labels, collapsible `ResultsButton`, content previews, diagnostics formatting, `TaskTool`, `ContentBash`, and `flattenToolArgs` fallback argument display.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css` backs the share/session part visual contract and is retained as the CSS/layout source anchor.
- Forge implementation path: `crates/webui/src/chat_ui_enhancements.html`.
