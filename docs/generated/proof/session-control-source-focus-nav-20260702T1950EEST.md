# Proof — session-control source focus trail navigation

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`
Repo: `organicoverlords/forge-unified`

## Live state before this slice

Selected from the provided source-of-truth URL branch because PR #3 remains the open, non-draft, mergeable app-change PR into `master`.

Same-head status inspected for previous head `e5f90c4571aeceaed91a28f3781dd122621ed4a4`:

- CI `28603378106`: success.
- Build Proof `28603378102`: success.
- Fast WebUI Proof `28603378277`: success.
- Live WebUI Feature Sprint `28603378181`: success.
- App Build Proof `28603378110`: success.
- App Multistep Build Proof `28603378215`: success.

Live WebUI proof for previous head:

- Run: `28603378181`.
- Job: `84817170882`.
- Artifact: `8044786382` (`live-webui-feature-sprint-proof`).
- The job steps include natural feature-build prompt through WebUI and full benchmark evidence/quality check.

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Source-backed behavior copied/adapted:

- `StickyAccordionHeader` keeps active diff context visible while scrolling.
- `Accordion.Trigger` and `data-slot="session-turn-diff-trigger"` create navigable receipt headers.
- `data-slot="session-turn-diff-path"` and `data-slot="session-turn-diff-meta"` keep path/meta context attached to each receipt row.
- `showAll`, `overflow`, `visible`, and `session-turn-diffs-more` patterns support moving through a filtered/visible group without losing context.

## Feature built

Added source-group navigation to `crates/webui/src/chat_ui_session_control_source_focus_trail.html`:

- `previous source receipt` moves to the prior receipt with the same source/action group.
- `next source receipt` moves to the next receipt with the same source/action group.
- `position: N/M` shows the selected receipt's position in the current source/action group.
- Existing `return to selected receipt` remains intact.
- The implementation updates `data-source-map-jump-target` and `data-source-map-selected`, scrolls/focuses the row, and refreshes the sticky focus trail.

## Proof hooks added/preserved

- `session-control-source-focus-nav`
- `session-control-source-focus-position`
- `session-control-source-focus-trail`
- `session-control-source-focus-return`
- `opencode-session-turn-diffs-toggle`
- `opencode-session-turn-diffs-more`
- `opencode-session-turn-diffs-group-shape`
- `opencode-session-turn-diff-path-shape`
- `opencode-session-turn-diff-meta-shape`

## Claim boundary

This file records the source-backed implementation slice. The new post-slice head is not same-head browser/NIM proven until the GitHub Actions workflows complete on that exact commit and the Live WebUI proof artifact/screenshots are inspected.
