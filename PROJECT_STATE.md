# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous same-head green proof: all required workflows passed on `4e2f9a7619403bbadb7f6a4a5784b2f310017dd3`, including Live WebUI Feature Sprint artifact `7978708658` and Fast WebUI Proof artifact `7978348458`.
- Current selected baseline before this slice: `4e2f9a7619403bbadb7f6a4a5784b2f310017dd3`.
- Latest implementation/proof slice: Forge-local session control event ledger for backend-backed checkpoint, fork, revert latest turn, and retry source actions.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Kept backend-backed session operations in `crates/engine/src/agent.rs`: `retry_source`, `fork_conversation`, `revert_last_turn`, and `session_control_receipt`.
- Replaced session-control runtime source-path exposure with Forge-local `forge.session_control` receipts and `SESSION_CONTROL_BEHAVIOR` metadata.
- Updated backend route handlers in `crates/webui/src/conversation_controls.rs` so checkpoint, fork, revert-latest-turn, and retry-source all return structured receipt payloads.
- Updated browser control bundle `crates/webui/src/chat_ui_session_controls.html` with a visible receipt strip, `copy session receipt`, bubbling `forge:session-control` events, a visible session control event ledger, per-event status rows, `data-session-control-event`, and `copy session control event` affordances.
- Strengthened CI smoke gate `scripts/smoke/check-session-controls-contract.py` to require ledger/event-copy tokens and reject upstream source paths inside session-control runtime files.
- Added proof doc `docs/generated/proof/session-control-event-ledger-20260630T1150Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `backend-session-controls`, `backend-checkpoint-action`, `backend-fork-action`, `backend-revert-action`, `backend-retry-source-action`, `backend-session-control-status`, `backend-session-control-receipt`, `copy-session-control-receipt`, `session-control-event-ledger`, `backend-session-control-ledger`, `backend-session-control-event-row`, `copy-session-control-event`, `data-session-control-event`, and `forge-local-control-receipt`.
- This slice upgrades backend session controls with a visible/copyable Forge-local session control event ledger. It does not claim full parity, production readiness, or same-head acceptance until workflows finish.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, lifecycle event ledger, copy tool event, tool event JSON, input toggle, diagnostic copy, target copy, preview pane, preview toggle, copy preview, accessible disclosure state, `aria-expanded`, `aria-controls`, backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, session control event ledger, copy session control event, and `data-session-control-event`.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, `crates/webui/src/chat_ui_tool_lifecycle.html`, `crates/webui/src/chat_ui_session_controls.html`, and `crates/webui/src/conversation_controls.rs`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### Backend session controls contract

- Source anchor: `packages/session-ui/src/components/session-turn.tsx` for visible session actions, per-turn status, retry affordance, and action history semantics.
- Required behavior tokens: backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, copy session receipt, session control event ledger, copy session control event, `data-session-control-event`, and `forge:session-control` browser events.
- Forge implementation paths under guard: `crates/engine/src/agent.rs`, `crates/webui/src/conversation_controls.rs`, `crates/webui/src/chat_ui_session_controls.html`, and `scripts/smoke/check-session-controls-contract.py`.
