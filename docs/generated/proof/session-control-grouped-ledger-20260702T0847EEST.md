# Session-control grouped ledger parity slice — 2026-07-02 08:47 EEST

## Selection

- Repo: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open, non-draft, mergeable at pre-slice verification
- Baseline head before this slice: `894bc2f130555561d00ab7fe9e3833e7110981d7`

## Baseline workflow proof inspected

Same-head workflows for `894bc2f130555561d00ab7fe9e3833e7110981d7` were all green before this slice:

- CI: `28566244880`
- Build Proof: `28566245035`
- Fast WebUI Proof: `28566244911`
- Live WebUI Feature Sprint: `28566244891`
- App Build Proof: `28566244908`
- App Multistep Build Proof: `28566244886`

Live WebUI job inspected: `84694041284`.
Live WebUI artifact inspected: `8029419908` (`live-webui-feature-sprint-proof`).

## Feature built

Added a browser-visible grouped session-control ledger surface:

- Groups current session-control receipt rows by action and status.
- Shows the top four groups with event counts.
- Supports expand/collapse for grouped receipt details.
- Shows overflow when more than four groups exist.
- Preserves Forge-local identity and avoids exposing OpenCode branding in the UI.

## Forge paths changed

- `crates/webui/src/chat_ui_session_control_groups.html`
- `crates/webui/src/chat_ui.rs`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-grouped-ledger-20260702T0847EEST.md`

## OpenCode source backing

Exact upstream source paths used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-component="session-turn-diffs-group"`
  - `showAll()` / `toggleAll()`
  - `overflow()`
  - `visible()`
  - accordion-style expanded state for per-file diff groups

The Forge slice adapts the same interaction shape to session-control receipts instead of file diffs: summarized grouped rows, bounded default visibility, explicit overflow, and expandable detail.

## Browser proof status

This commit has not yet completed same-head browser/NIM proof. GitHub Actions are expected to run after push. Do not claim same-head parity until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete for the final head and proof artifacts/screenshots are inspected.
