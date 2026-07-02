# Session-control grouped ledger show-all proof

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

Started from the provided source-of-truth URL:

`https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

Open PR #3 remains the current meaningful app-change PR for this branch. Older open PRs #1 and #2 are superseded for this work.

## Baseline verified before this slice

Same-head green baseline inspected for `ff1f376044b2b05299acf62ec9e7a87d0d09dae9`:

- CI `28568577285`: success
- Build Proof `28568577296`: success
- Fast WebUI Proof `28568577300`: success
- Live WebUI Feature Sprint `28568577288`: success
- App Build Proof `28568577298`: success
- App Multistep Build Proof `28568577286`: success

Live WebUI job inspected:

- Run: `28568577288`
- Job: `84701004571`
- Artifact: `8030287282` (`live-webui-feature-sprint-proof`)
- Steps passed: live WebUI feature sprint, natural feature-build prompt through WebUI, full benchmark evidence and quality score check, proof upload.

## Source-backed parity slice

Built: **show-all/show-less overflow control for the WebUI session-control grouped ledger**.

OpenCode source paths used exactly:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `showAll()`
  - `toggleAll()`
  - `overflow()`
  - `visible()`
  - `data-component="session-turn-diffs-group"`
  - `data-slot="session-turn-diffs-more"`

Forge implementation path:

- `crates/webui/src/chat_ui_session_control_groups.html`

## Behavior added

- Keeps the existing grouped session-control receipt rows.
- Adds a toolbar summarizing the number of grouped receipt classes.
- Adds `show all N groups` when more than four groups exist.
- Adds `show fewer groups` when expanded.
- Keeps the existing per-group expand/collapse receipt details.
- Preserves Forge identity; no OpenCode branding or metadata is exposed in the UI.

## Proof hooks added/preserved

- `session-control-group-show-all`
- `session-control-group-show-less`
- `session-control-group-toolbar`
- `opencode-session-turn-show-all-toggle`
- Existing grouped-ledger hooks remain: `session-control-grouped-ledger`, `session-control-group-strip`, `session-control-group-row`, `session-control-group-toggle`, `session-control-group-detail`, `session-control-group-count`, `session-control-group-visible`, `session-control-group-overflow`, `opencode-session-turn-diffs-group-shape`.

## Claim boundary

This slice is source-backed and committed, but the new post-slice head is not same-head browser/NIM proven until GitHub Actions complete and the Live WebUI proof artifact is inspected for this exact head.
