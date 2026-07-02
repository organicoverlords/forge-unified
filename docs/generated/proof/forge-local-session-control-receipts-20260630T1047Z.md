# Proof — Forge-local session control receipts

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

- Target URL source of truth: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected PR: #3, `mvp router slice`, open/non-draft/mergeable.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- Starting head inspected for this slice: `9715d89fbd8ed2da474bf9d44af8b0b103a3e3c4`.
- Same-head workflows for that starting head were queued/running when inspected:
  - CI `28438744566`: queued.
  - App Multistep Build Proof `28438744585`: queued.
  - Build Proof `28438744559`: queued.
  - App Build Proof `28438744560`: queued.
  - Fast WebUI Proof `28438744615`: in progress.
  - Live WebUI Feature Sprint `28438744564`: in progress.

## Feature built

Added Forge-local, copyable session-control receipts for backend-backed WebUI session actions:

- Backend routes now return a structured `receipt` object for checkpoint, fork, revert-latest-turn, and retry-source actions.
- WebUI renders the latest backend receipt in a visible receipt strip.
- WebUI exposes `copy session receipt` and dispatches a bubbling `forge:session-control` event with the same receipt payload.
- Session-control runtime payloads now use Forge-local receipt semantics and do not expose upstream OpenCode source paths in the browser/control API.
- CI smoke gate now requires receipt/event tokens and rejects upstream source paths inside session-control runtime files.

## OpenCode source backing

Recorded source references used for behavior only, not runtime branding:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `SessionRetry` integration and turn-level actions.
  - active/working turn state and visible action placement.
  - assistant copy/diff action ergonomics near session turns.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - action model for user/assistant message parts.
- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - visible part evidence and copyable result behavior.

## Files changed

- `crates/engine/src/agent.rs`
- `crates/webui/src/conversation_controls.rs`
- `crates/webui/src/chat_ui_session_controls.html`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/forge-local-session-control-receipts-20260630T1047Z.md`

## Proof boundary

This commit is a source-backed implementation slice. It does not claim full OpenCode parity, production readiness, or latest-head acceptance until CI, Build Proof, App Build Proof, App Multistep Build Proof, Fast WebUI Proof, and Live WebUI Feature Sprint complete on the final head and the artifacts/screenshots are inspected.
