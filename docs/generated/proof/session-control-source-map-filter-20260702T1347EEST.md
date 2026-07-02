# Session-control source-map filter proof note

Date: 2026-07-02 13:47 EEST
Branch: `mvp/nim-freellmapi-router-20260626`
Pre-slice verified head: `c5c50c65884db253bc1d1a545d71dda54f761e3c`
Implementation commit: `25ddbd392eef76d7a0d583615100245c76dbb42a`
State/proof commit follows this note.

## Selection basis

The provided source-of-truth URL points to `organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
Open PR #3 remains the active app-change PR for this branch. Older open PRs #2 and #1 are superseded by PR #3 for this work.

## Live state before this slice

For head `c5c50c65884db253bc1d1a545d71dda54f761e3c`, the checked workflows were all green:

- CI `28581028645`: success.
- Build Proof `28581028643`: success.
- Fast WebUI Proof `28581028634`: success.
- Live WebUI Feature Sprint `28581028638`: success.
- App Build Proof `28581028651`: success.
- App Multistep Build Proof `28581028661`: success.

Live WebUI proof inspected:

- Job `84741079091`: `live-webui-feature-sprint`, success.
- Artifact `8035266302`: `live-webui-feature-sprint-proof`, branch `mvp/nim-freellmapi-router-20260626`, head `c5c50c65884db253bc1d1a545d71dda54f761e3c`.
- The job reported success for the live WebUI feature sprint, the natural feature-build prompt through WebUI, full benchmark evidence and quality score check, and proof upload.

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Exact source anchors used:

- `data-slot="session-turn-diffs-header"`
- `data-slot="session-turn-diffs-label"`
- `data-slot="session-turn-diffs-toggle"`
- `data-slot="session-turn-diff-trigger"`
- `data-slot="session-turn-diff-path"`
- `data-slot="session-turn-diff-directory"`
- `data-slot="session-turn-diff-filename"`
- `data-slot="session-turn-diff-meta"`
- `showAll()` / `toggleAll()`
- `overflow()` / `visible()`
- `getDirectory()` / `getFilename()`

## Forge slice built

Implemented a functional source-map filter on backend session-control receipt rows.

Changed behavior:

- Source, action, and status metadata chips now act as row filters.
- Active filter chips expose `aria-pressed="true"`.
- The session head shows a filter toolbar with shown/hidden event counts.
- A `clear source-map filter` action restores the full event ledger.
- Existing `copy source map` receipt export remains preserved.
- Event rows hidden by the source-map filter are marked with `data-source-map-hidden="true"`.

Implementation path:

- `crates/webui/src/chat_ui_session_control_source_map.html`

Proof markers added/preserved:

- `session-control-source-map-filter`
- `session-control-source-map-clear`
- `source-map-filtered-event-count`
- `opencode-session-turn-diff-trigger-shape`
- `session-control-source-map`
- `session-control-source-map-source`
- `session-control-source-map-action`
- `session-control-source-map-receipt`
- `copy-session-control-source-map`

## Claim boundary

This slice is source-backed and committed, but the latest head created by this proof/state update is not same-head browser/NIM proven yet.
Do not claim Live WebUI parity for this new head until exact-head CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete and the Live WebUI artifact/screenshots are inspected.
