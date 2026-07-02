# Proof — session-control source focus follow lock

Date: 2026-07-02 20:46 EEST
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Live selection basis

- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected PR #3 because it is the current open, non-draft, mergeable app-change PR for this branch.
- Previous exact head inspected before this slice: `140d549e85a110c9cbcdf7b2be7cd7478466e15c`.
- Same-head workflows for that previous head were all successful:
  - CI `28607037161`.
  - Build Proof `28607037156`.
  - Fast WebUI Proof `28607037151`.
  - Live WebUI Feature Sprint `28607037171`.
  - App Build Proof `28607037153`.
  - App Multistep Build Proof `28607037199`.
- Live WebUI proof inspected for previous head:
  - Run `28607037171`.
  - Job `84829830399`.
  - Artifact `8046194713` (`live-webui-feature-sprint-proof`).

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Relevant OpenCode structures copied as behavior shape, without exposing OpenCode branding in Forge UI:

- `toggleAll()` calls `autoScroll.pause()` before changing expanded/visible state.
- `showAll`, `overflow`, and `visible` keep an explicit, user-controlled view state.
- `data-slot="session-turn-diff-trigger"`, `data-slot="session-turn-diff-path"`, and `data-slot="session-turn-diff-meta"` structure the diff/receipt interaction affordance.
- `StickyAccordionHeader` keeps the active receipt context visible while the user inspects details.

## Forge slice built

Implemented: **session-control source focus follow lock**.

Forge path changed:

- `crates/webui/src/chat_ui_session_control_source_focus_trail.html`

Behavior:

- Adds a sticky `pause focus follow` / `resume live focus` control to the selected source focus trail.
- When paused, the trail locks to the current receipt by `receipt_id`, `sequence`, or `data-receipt-id`.
- While paused, ledger mutations do not silently move the focus trail to a newly selected/jump-targeted receipt.
- The UI marks the state with `data-source-focus-follow-paused="true"` on `body` and shows a `follow: paused` metadata chip.
- Existing previous/next source receipt navigation, source/action grouping, selected receipt return, jump target, source map filtering, selected detail disclosure, lazy detail rendering, and copy controls are preserved.

## Browser/proof markers added

- `session-control-source-focus-lock`
- `session-control-source-focus-follow-paused`
- `opencode-session-turn-autoscroll-pause-shape`

Existing proof markers preserved:

- `session-control-source-focus-trail`
- `session-control-source-focus-return`
- `session-control-source-focus-nav`
- `session-control-source-focus-position`
- `opencode-session-turn-sticky-accordion-header-shape`
- `opencode-session-turn-diff-path-shape`
- `opencode-session-turn-diff-meta-shape`

## Claim boundary

This proof records the source-backed implementation slice. The new head created by this slice is **not same-head browser/NIM proven yet** until the exact-head GitHub Actions workflows complete and the Live WebUI artifact/screenshots are inspected.
