# Session error history rail proof

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`
Baseline head inspected before this slice: `6769f96f4a3e945d4ab05bc7466fb95c07253206`

## Live state inspected before implementation

- Source-of-truth repo URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open, non-draft, mergeable, base `master`.
- Baseline same-head workflows for `6769f96f4a3e945d4ab05bc7466fb95c07253206` were all green:
  - CI `28556582482`.
  - Build Proof `28556582475`.
  - Fast WebUI Proof `28556582470`.
  - App Build Proof `28556582469`.
  - App Multistep Build Proof `28556582480`.
  - Live WebUI Feature Sprint `28556582473`.
- Baseline proof artifacts inspected:
  - Fast WebUI Proof artifact `8025862263`.
  - Live WebUI Feature Sprint artifact `8026081210`.

## OpenCode source backing

Exact upstream reference paths used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `unwrap()` error extraction path.
  - visible error `Card` rendering.
  - `showAll`, `overflow`, and `visible` session-turn diff group pattern.

Forge implementation paths:

- `crates/webui/src/chat_ui_error_history.html`.
- `crates/webui/src/chat_ui.rs`.
- `PROJECT_STATE.md`.
- `docs/generated/proof/session-error-history-rail-20260702T0355EEST.md`.

## Feature built

Added a browser WebUI session-control error history rail:

- tracks recent `forge:session-control` error receipts in browser state;
- renders a readable `Session error history` rail below the latest readable error card;
- shows the newest three errors by default;
- provides `show N older errors` / `show fewer errors` overflow behavior;
- exposes per-error raw receipt data through `data-session-control-event`;
- adds `copy error history` to copy readable summaries plus raw event data;
- keeps Forge UI independent and does not display OpenCode branding.

## Proof markers added

- `session-error-history-rail`
- `session-error-history-overflow`
- `copy-session-control-error-history`
- `opencode-showall-overflow-parity`
- `opencode-error-card-parity`
- `packages/session-ui/src/components/session-turn.tsx`

## Validation boundary

This commit records source-backed implementation proof. Same-head browser/NIM screenshot proof for the new head must be checked from the GitHub Actions artifacts after CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the final head.
