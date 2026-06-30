# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current selected baseline before this slice: `a9e2fa3ed068da87cc3636f673ad33e6c8fa7a53`
- Latest same-head green proof before this slice: all required workflows passed on `a9e2fa3ed068da87cc3636f673ad33e6c8fa7a53`, including Live WebUI Feature Sprint artifact `7970696828`.
- Latest implementation/proof slice: WebUI tool lifecycle cards now expose stable browser-visible tool receipt ids and a `copy tool receipt` action in `crates/webui/src/chat_ui_tool_lifecycle.html`.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_tool_lifecycle.html`.
- Added stable per-tool receipt ids from tool id/status/target/ordinal.
- Added visible `tool-life-receipt` chips carrying `data-proof="tool-receipt-id-visible"`.
- Added `data-tool-receipt` on lifecycle strips and tool cards so browser proof screenshots can identify exact tool cards.
- Added `copy tool receipt` to copy a compact receipt plus status/target.
- Preserved existing lifecycle state/timing/copy-link behavior.
- Added proof doc `docs/generated/proof/webui-tool-receipt-id-20260630T0452Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, `stable-session-receipts`, `typed-tool-renderer`, `tool-target-visible`, `tool-result-toggle`, `tool-args-visible`, `tool-diagnostics-visible`, `tool-count-summary-visible`, `tool-lifecycle-strip`, `tool-state-timeline`, `copy-tool-anchor`, `tool-output-duration-visible`, `tool-receipt-id-visible`, and `copy-tool-receipt`.
- This slice improves screenshot auditability and tool-card lifecycle parity. It does not claim full parity, production readiness, or backend fork/revert/session checkpoint semantics.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, and `crates/webui/src/chat_ui_tool_lifecycle.html`.
