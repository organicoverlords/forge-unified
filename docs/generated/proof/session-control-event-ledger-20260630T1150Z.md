# Session control event ledger proof

Date: 2026-06-30

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- Selection basis: the target URL names this branch, PR #3 is open/non-draft/mergeable, and the previous branch head `4e2f9a7619403bbadb7f6a4a5784b2f310017dd3` was recorded as same-head green with all required workflows and NVIDIA NIM WebUI artifacts.
- Baseline before this slice: `4e2f9a7619403bbadb7f6a4a5784b2f310017dd3`.

## Feature slice

Implemented a Forge-local WebUI session control event ledger for backend-backed checkpoint, fork, revert-latest-turn, and retry-source actions.

The runtime UI now keeps OpenCode source paths out of browser/control payloads while exposing:

- a visible `backend-session-control-ledger` below the latest receipt;
- per-event `backend-session-control-event-row` entries;
- explicit `data-session-control-event` attributes for DOM-proofable event payloads;
- per-event `copy event` controls via `copy-session-control-event`;
- retained `copy session receipt` and bubbling `forge:session-control` events.

## OpenCode source backing

Exact upstream source paths used as behavioral reference:

- `packages/session-ui/src/components/session-turn.tsx` — visible turn actions, per-turn status/action affordances, and retry semantics.
- `packages/session-ui/src/components/message-part.tsx` — message part grouping around session turns.
- `packages/session-ui/src/components/basic-tool.tsx` — compact copyable tool/action evidence pattern.
- `packages/session-ui/src/components/tool-count-summary.tsx` — concise event/count summary treatment.

These paths are recorded only in docs/proof state. Runtime browser/control files intentionally do not expose upstream source-path metadata.

## Forge files changed

- `crates/webui/src/chat_ui_session_controls.html`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-event-ledger-20260630T1150Z.md`

## Deterministic proof expectations

The CI session-control smoke gate now requires these durable tokens:

- `backend-session-control-ledger`
- `backend-session-control-event-row`
- `copy-session-control-event`
- `data-session-control-event`
- existing backend controls: checkpoint, fork, revert latest turn, retry source, receipt, and `forge:session-control`.

## Proof boundary

This commit is an implementation/proof-surface update. Do not mark the latest head accepted or parity-proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof pass on the exact final head and WebUI screenshot artifacts are inspected. Browser proof must use NVIDIA NIM only.
