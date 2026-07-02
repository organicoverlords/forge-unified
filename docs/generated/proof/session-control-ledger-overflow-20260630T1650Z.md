# Proof — Session-control ledger overflow

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`
Baseline before slice: `0e5fa741c341e84f3c43016166ca07c8f8576329`

## Selection basis

- Target URL branch remains the selected work branch.
- PR #3 is open, non-draft, and mergeable.
- No newer open PR or recently pushed non-main branch with meaningful app changes was selected over the target branch.

## Same-head proof inspected before this slice

Baseline `0e5fa741c341e84f3c43016166ca07c8f8576329` completed all required workflows successfully:

- CI `28457433840`
- Build Proof `28457433730`
- Fast WebUI Proof `28457433734`
- Live WebUI Feature Sprint `28457433806`
- App Build Proof `28457433765`
- App Multistep Build Proof `28457433751`

Live WebUI browser proof artifact:

- `7986299221` / `live-webui-feature-sprint-proof`

## OpenCode source backing

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Uses a `MAX_FILES` limit and `showAll` state for long changed-file lists.
  - Renders a visible overflow toggle that switches between `showAll` and compact display.
  - Computes visible diff rows separately from full diff count.

Forge adaptation:

- `crates/webui/src/chat_ui_session_controls.html`
  - Adds `ledgerExpanded` state.
  - Filtered ledgers show the newest four receipts by default.
  - Adds visible `show older` / `show less` toggle when more than four filtered rows exist.
  - Adds `showing visible/filtered` count to the session-control summary.
  - Keeps event disclosure, copy event, status filters, diff chips, duration chips, and `forge:session-control` browser events.

## Deterministic gate update

- `scripts/smoke/check-session-controls-contract.py` now requires:
  - `session-control-ledger-overflow`
  - `backend-session-control-overflow-toggle`
  - `session-control-show-all`
  - `session-control-show-less`
  - `session-control-visible-count`
  - `ledgerExpanded`
  - `show older`
  - `show less`

## Files changed

- `crates/webui/src/chat_ui_session_controls.html`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-ledger-overflow-20260630T1650Z.md`

## Claim boundary

This is an OpenCode-backed WebUI behavior increment. It does not claim full OpenCode parity, production readiness, or same-head acceptance for the new head until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact final head and WebUI artifacts/screenshots are inspected.
