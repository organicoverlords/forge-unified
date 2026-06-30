# WebUI session-control hidden overflow row proof

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

- Source-of-truth branch remains `mvp/nim-freellmapi-router-20260626` from PR #3.
- PR #3 is open, non-draft, and mergeable.
- Other open PRs are older and do not supersede the target branch for the current OpenCode parity sprint.

## Inspected CI blocker

- Same-head proof for `2bd70f8ac1cc3bb7fe9059fe78610768c7af3a16` showed Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof passed.
- CI `28461375967` failed only in Smoke Test job `84349818167`.
- The failed gate showed route/UI behavior checks passing but failed the exact `ui:show older` token and stale state phrase `checkpoint, fork, revert latest turn, and retry source`.

## OpenCode source backing

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `MAX_FILES`, `showAll`, `overflow`, and `visible` split long changed-file evidence into compact vs expanded sets.
  - `session-turn-diffs-more` renders a visible collapsed-overflow row when not all changed-file evidence is shown.
  - `turnDurationMs` keeps turn evidence tied to visible session lifecycle timing.

## Forge implementation slice

- `crates/webui/src/chat_ui_session_controls.html`
  - Keeps newest four session-control receipts visible by default.
  - Uses an exact visible `show older (...)` control and `show less` expanded state.
  - Adds `session-control-hidden-overflow-row` so screenshots can prove there are hidden older receipts before expansion, mirroring OpenCode's visible `more` row pattern.
  - Keeps Forge-local runtime payloads free of upstream OpenCode source-path metadata.
- `scripts/smoke/check-session-controls-contract.py`
  - Requires the hidden overflow row token, exact `show older`, and the current state phrasing.
- `PROJECT_STATE.md`
  - Records the inspected CI blocker and new source-backed parity slice.

## Claim boundary

This is a meaningful OpenCode-backed WebUI behavior increment, not full OpenCode parity. Latest-head same-head proof requires CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof to complete successfully on the final head, and WebUI artifacts/screenshots must be inspected before claiming browser proof.