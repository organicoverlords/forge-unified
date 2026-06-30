# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous same-head green proof: all required workflows passed on `62932c83ef2a88cee3bbb090208e3e82ea3f5d52`, including Live WebUI Feature Sprint artifact `7974499647`.
- Current selected baseline before this slice: `62932c83ef2a88cee3bbb090208e3e82ea3f5d52`.
- Latest inspected state on that baseline: CI `28429136936`, Build Proof `28429136942`, App Build Proof `28429136986`, App Multistep Build Proof `28429136999`, Fast WebUI Proof `28429136996`, and Live WebUI Feature Sprint `28429137007` all completed successfully.
- Latest implementation/proof slice: WebUI typed tool-card evidence toggles now use OpenCode-backed disclosure semantics: `aria-controls`, `aria-expanded`, and explicit `data-state` open/closed state for input, preview, and result panes.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_enhancements.html` to add a reusable `disclosureButton(...)` helper for `show input`, `show preview`, and `show result` controls. The controls now bind to pane ids with `aria-controls`, expose `aria-expanded`, and set `data-state=open|closed` so screenshots and DOM proof can distinguish collapsed vs expanded evidence.
- Updated `scripts/smoke/check-webui-proof-part-contract.py` so CI requires `tool-toggle-aria-expanded`, `tool-toggle-state-visible`, `aria-expanded`, `aria-controls`, and proof-trail tokens for accessible disclosure state.
- Added proof doc `docs/generated/proof/webui-accessible-tool-disclosures-20260630T0847Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, `stable-session-receipts`, `typed-tool-renderer`, `tool-target-visible`, `tool-result-toggle`, `tool-args-visible`, `tool-diagnostics-visible`, `tool-count-summary-visible`, `tool-input-toggle`, `tool-diagnostic-copy`, `tool-target-copy`, `tool-preview-visible`, `tool-preview-toggle`, `tool-preview-copy`, `tool-toggle-aria-expanded`, `tool-toggle-state-visible`, `tool-lifecycle-strip`, `tool-state-timeline`, `copy-tool-anchor`, `tool-output-duration-visible`, `tool-receipt-id-visible`, and `copy-tool-receipt`.
- This slice improves WebUI evidence usability and screenshot verifiability. It does not claim full parity, production readiness, backend fork/revert/session checkpoint semantics, or same-head acceptance.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, input toggle, diagnostic copy, target copy, preview pane, preview toggle, copy preview, accessible disclosure state, `aria-expanded`, `aria-controls`.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, and `crates/webui/src/chat_ui_tool_lifecycle.html`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### OpenCode source backing for latest WebUI accessible-disclosures slice

- `anomalyco/opencode:packages/web/src/components/share/part.tsx` uses a `ResultsButton` component with internal open/closed state for collapsible tool results, and uses per-tool renderers for read/write/edit/bash/grep/glob/task, target labels, content previews, diagnostics formatting, `TaskTool`, `ContentBash`, and `flattenToolArgs` fallback argument display.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css` backs the share/session part visual contract and is retained as the CSS/layout source anchor.
- Forge implementation path: `crates/webui/src/chat_ui_enhancements.html`.
