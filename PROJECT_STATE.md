# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous same-head green proof: all required workflows passed on `d97a1851189bd243afc2c499e6a03a6293b7ee5c`, including Live WebUI Feature Sprint artifact `7975940477`.
- Current selected baseline before this slice: `d97a1851189bd243afc2c499e6a03a6293b7ee5c`.
- Latest inspected state on that baseline: CI `28432259806`, Build Proof `28432259855`, App Build Proof `28432259822`, App Multistep Build Proof `28432259808`, Fast WebUI Proof `28432259807`, and Live WebUI Feature Sprint `28432259837` all completed successfully.
- Latest implementation/proof slice: WebUI tool lifecycle strips now expose a structured, copyable lifecycle event ledger per tool card via `data-tool-event`, visible event JSON, `copy tool event`, and bubbling `forge:tool-lifecycle` browser events.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_tool_lifecycle.html` so each visible tool lifecycle strip stores `data-tool-status`, `data-tool-target`, and `data-tool-event`.
- Added a compact `.tool-life-event` evidence pill containing JSON with `type`, `receipt`, `status`, `target`, `duration`, and `index`.
- Added `copy tool event` so browser proof and users can copy the exact lifecycle payload for a tool card.
- Added a bubbling `forge:tool-lifecycle` `CustomEvent` for each enhanced tool so WebUI proof code can observe real lifecycle evidence instead of scraping only visual text.
- Added proof doc `docs/generated/proof/webui-tool-lifecycle-event-ledger-20260630T0945Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, `stable-session-receipts`, `typed-tool-renderer`, `tool-target-visible`, `tool-result-toggle`, `tool-args-visible`, `tool-diagnostics-visible`, `tool-count-summary-visible`, `tool-input-toggle`, `tool-diagnostic-copy`, `tool-target-copy`, `tool-preview-visible`, `tool-preview-toggle`, `tool-preview-copy`, `tool-toggle-aria-expanded`, `tool-toggle-state-visible`, `tool-lifecycle-strip`, `tool-state-timeline`, `copy-tool-anchor`, `tool-output-duration-visible`, `tool-receipt-id-visible`, `copy-tool-receipt`, `tool-lifecycle-event-ledger`, `copy-tool-event`, and `tool-event-json-visible`.
- This slice improves WebUI evidence semantics and screenshot/DOM verifiability. It does not claim full parity, production readiness, backend fork/revert/session checkpoint semantics, or same-head acceptance.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, lifecycle event ledger, copy tool event, tool event JSON, input toggle, diagnostic copy, target copy, preview pane, preview toggle, copy preview, accessible disclosure state, `aria-expanded`, `aria-controls`.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, and `crates/webui/src/chat_ui_tool_lifecycle.html`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### OpenCode source backing for latest WebUI lifecycle-event slice

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` is the source anchor for ToolPart lifecycle transitions and durable tool events.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` is the source anchor for typed pending/running/completed/error tool states.
- `anomalyco/opencode:packages/web/src/components/share/part.tsx` is the source anchor for rendering tool state/result evidence into visible session parts.
- Forge implementation path: `crates/webui/src/chat_ui_tool_lifecycle.html`.
