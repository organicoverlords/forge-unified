# Proof — session-control source-map jump

Date: 2026-07-02 17:45 EEST
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Live selection basis

- Source URL branch: `mvp/nim-freellmapi-router-20260626`.
- Selected PR: #3, open, non-draft, mergeable before this slice.
- Previous head inspected: `79cb7246b626d236b5ccf6c36166c834b4d8b74f`.
- Previous exact-head workflows: CI `28595518082`, Build Proof `28595515564`, Fast WebUI Proof `28595518524`, App Build Proof `28595515550`, and App Multistep Build Proof `28595515968` passed; Live WebUI Feature Sprint `28595515733` failed.
- Live WebUI failure inspected: job `84789782356` reached NVIDIA NIM / WebUI benchmark execution with provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, and artifact `8041558915`, but failed because the full six-phase benchmark stream did not finish before the 900 second timeout.

## OpenCode source backing

Exact upstream path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Relevant source anchors:

- `StickyAccordionHeader`
- `Accordion.Trigger`
- `data-slot="session-turn-diff-trigger"`
- `data-slot="session-turn-diff-path"`
- `data-slot="session-turn-diff-meta"`
- expanded diff focus pattern around `data-slot="session-turn-diff-view"`

## Forge implementation

Implemented: **WebUI session-control source-map jump to selected receipt source**.

Files:

- Added `crates/webui/src/chat_ui_session_control_source_jump.html`.
- Updated `crates/webui/src/chat_ui.rs` to include the new helper.
- Updated `PROJECT_STATE.md`.

Behavior:

- When a receipt is selected in the source-map panel, a `jump to receipt source` control appears next to the selected receipt actions.
- Activating it scrolls the underlying session-control ledger row into view, marks it with `data-source-map-jump-target="true"`, and focuses the row.
- This preserves the existing selected receipt panel, lazy details, source/action/status filters, and copy controls.
- Forge remains independent; OpenCode is used only as source behavior reference and no OpenCode branding is exposed.

Proof hooks added:

- `session-control-source-map-jump`
- `session-control-source-map-jump-target`
- `opencode-session-turn-diff-trigger-shape`
- `opencode-session-turn-sticky-accordion-header-shape`
- `opencode-session-turn-diff-path-shape`

## Claim boundary

This slice is code-backed and source-backed, but this new head is not same-head browser/NIM proven until GitHub Actions complete for the final head and the Live WebUI proof artifact/screenshots are inspected.
