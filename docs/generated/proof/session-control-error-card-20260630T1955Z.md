# Proof — WebUI session-control error card

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

- Source-of-truth branch remains `mvp/nim-freellmapi-router-20260626`.
- PR #3 remains the selected open WebUI/NIM router parity work.
- Before this slice, head `99d28c7d5496508c35354680d53e773054be03b6` had same-head green workflows:
  - CI `28468483053`
  - Build Proof `28468483068`
  - Fast WebUI Proof `28468483112`
  - Live WebUI Feature Sprint `28468483083`
  - App Build Proof `28468483121`
  - App Multistep Build Proof `28468483114`
- Live WebUI artifact inspected: `7990943905` / `live-webui-feature-sprint-proof`.

## OpenCode source backing

Inspected upstream source:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `unwrap()` extracts readable error text from structured/stringified error payloads.
  - `errorText` derives a displayable error string from assistant message error data.
  - `<Card variant="error" class="error-card">` renders visible turn-level error state.

## Forge implementation slice

Added a Forge-local WebUI session-control error card in `crates/webui/src/chat_ui_session_controls.html`:

- Listens to `forge:session-control` browser events.
- Captures session-control receipts with `status === "error"`.
- Renders a visible `Latest session-control error: ...` card near the session receipt.
- Adds `copy latest error` to copy the latest error receipt as JSON.
- Keeps OpenCode source paths in this proof file only; runtime payloads do not expose upstream source metadata.

Updated deterministic gate:

- `scripts/smoke/check-session-controls-contract.py` now requires:
  - `session-control-error-card`
  - `backend-session-control-error-card`
  - `copy-session-control-error`
  - `copy latest error`
  - `latest-session-control-error`

## Files changed

- `crates/webui/src/chat_ui_session_controls.html`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-error-card-20260630T1955Z.md`

## Claim boundary

This is a small but real OpenCode-backed UI behavior increment. It is not a full OpenCode parity claim. Same-head CI/WebUI artifacts for the final head must complete before claiming this commit is proven by browser screenshots.
