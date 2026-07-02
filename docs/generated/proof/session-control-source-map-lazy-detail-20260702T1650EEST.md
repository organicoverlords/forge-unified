# Proof — session-control source-map lazy selected receipt detail

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`
Implementation commit: `9f3dd2dee04cf5bed8130738d05c408a366c5f4b`

## Live selection basis

- Source-of-truth URL branch: `mvp/nim-freellmapi-router-20260626`.
- PR #3 is still the meaningful current app-change PR for this work; older open PRs #2 and #1 are superseded.
- Previous selected head before this slice: `d8ce1c0007b4f35c538d7e8e0ceb8d1abc8055c0`.
- PR #3 was verified open, non-draft, and mergeable before this slice.

## Baseline proof inspected before this slice

The previous head `d8ce1c0007b4f35c538d7e8e0ceb8d1abc8055c0` had all six expected workflows passing:

- Fast WebUI Proof `28591240628`: success.
- Build Proof `28591240654`: success.
- App Multistep Build Proof `28591240727`: success.
- App Build Proof `28591240642`: success.
- CI `28591240646`: success.
- Live WebUI Feature Sprint `28591240673`: success.

Live WebUI proof inspected:

- Run: `28591240673`.
- Job: `84775071486`.
- Artifact: `8039752400` (`live-webui-feature-sprint-proof`).
- Artifact head: `d8ce1c0007b4f35c538d7e8e0ceb8d1abc8055c0`.
- Job steps showed successful live WebUI feature sprint, natural feature-build prompt through WebUI, full benchmark evidence and quality check, and proof upload.

## Source-backed OpenCode parity slice

Built: **lazy selected receipt detail rendering** for the WebUI session-control source-map selected receipt panel.

OpenCode source backing used exactly:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `createSignal(false)` for per-expanded diff view readiness.
  - `createEffect(on(active, ... requestAnimationFrame(... setShown(true))))`.
  - `Accordion.Trigger` and `StickyAccordionHeader` wrapping the compact diff row.
  - `data-slot="session-turn-diff-view"` rendering only after the expanded view is ready.

Forge implementation:

- `crates/webui/src/chat_ui_session_control_source_map.html`
  - Adds `detailRenderable` and `detailFrame` state.
  - Uses `requestAnimationFrame` before rendering raw selected receipt detail.
  - Shows a bounded `preparing receipt details…` pending row while the detail disclosure is open but not yet renderable.
  - Keeps raw receipt details out of `copy selected receipt` until the lazy detail body has rendered.
  - Preserves selected receipt metadata, filters, source/action/status chips, selected-row highlighting, and per-row copy behavior.

## Proof hooks added/preserved

- `session-control-source-map-lazy-detail`
- `session-control-source-map-selected-detail-pending`
- `opencode-session-turn-diff-view-lazy-shape`
- Existing selected detail hooks remain: `session-control-source-map-selected-detail`, `session-control-source-map-selected-detail-toggle`, and `opencode-session-turn-diff-view-shape`.

## Claim boundary

This implementation commit is source-backed and committed, but the latest post-proof/state head is not same-head browser/NIM proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and the Live WebUI artifact/screenshots are inspected.
