# Backend-backed WebUI session controls

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`

## Slice

Visible WebUI controls for session actions are now wired to Forge backend APIs instead of being UI-only affordances.

## Backend operations

Engine implementation path:

- `crates/engine/src/agent.rs`

Added operations:

- `retry_source` — resolves the latest user prompt from backend conversation state.
- `fork_conversation` — clones the current conversation into a new conversation id and records a session-control receipt.
- `revert_last_turn` — removes the latest user turn and later messages, then records a session-control receipt.
- `session_control_receipt` — records system metadata proving the session control was backend-backed.

## API routes

Route implementation path:

- `crates/webui/src/conversation_controls.rs`

Wired routes:

- `POST /api/conversations/:id/checkpoint`
- `POST /api/conversations/:id/fork`
- `POST /api/conversations/:id/revert-last-turn`
- `POST /api/conversations/:id/retry-source`

## Browser controls

Browser implementation path:

- `crates/webui/src/chat_ui_session_controls.html`

Visible proof tokens:

- `backend-session-controls`
- `backend-checkpoint-action`
- `backend-fork-action`
- `backend-revert-action`
- `backend-retry-source-action`
- `backend-session-control-status`

## Deterministic guard

CI now runs:

- `scripts/smoke/check-session-controls-contract.py`

The checker validates that the engine methods, WebUI routes, browser controls, bundled HTML include, and proof trail tokens all exist together.

## Claim boundary

This slice implements backend-backed checkpoint/fork/revert/retry-source control plumbing. It does not claim complete OpenCode parity, production readiness, or acceptance before same-head workflows and artifacts are inspected.
