# Proof — session-control source focus context copy

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection basis

- Source URL branch: `mvp/nim-freellmapi-router-20260626`.
- PR #3 remains the active meaningful app-change PR for this branch.
- Older open PRs #1 and #2 are superseded by PR #3 for the current WebUI/NIM/router work.

## Pre-slice verified state

Previous head: `cd1a0990ea2aecff1077040be7ec7431d374cbb9`.

Same-head workflows for that previous head:

- CI: `28610454277` — success.
- Build Proof: `28610454250` — success.
- Fast WebUI Proof: `28610454244` — success.
- Live WebUI Feature Sprint: `28610454253` — success.
- App Build Proof: `28610454330` — success.
- App Multistep Build Proof: `28610454298` — success.

Live WebUI proof for previous head:

- Run: `28610454253`.
- Job: `84841316194`.
- Artifact: `8047607199` (`live-webui-feature-sprint-proof`).
- Job steps included `Run natural feature-build prompt through WebUI`, `Check full benchmark evidence and quality score`, and `Upload feature sprint proof`.

## OpenCode source backing

Exact upstream path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Source-backed behaviors copied/shaped:

- `showAssistantCopyPartID` and `assistantCopyPartID` expose a stable copy target only after assistant output is no longer working.
- `data-slot="session-turn-assistant-content"` keeps assistant copy behavior attached to the visible assistant/session content.
- `toggleAll()` calls `autoScroll.pause()` before user-controlled expansion state changes.
- `showAll`, `overflow`, and `visible` keep explicit user-controlled view state.
- `StickyAccordionHeader`, `Accordion.Trigger`, `data-slot="session-turn-diff-trigger"`, `data-slot="session-turn-diff-path"`, and `data-slot="session-turn-diff-meta"` establish path/meta-oriented receipt affordances.

## Forge implementation

Changed path:

- `crates/webui/src/chat_ui_session_control_source_focus_trail.html`

State/proof paths:

- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-source-focus-context-copy-20260702T2150EEST.md`

Behavior added:

- Added `copy focus context` to the sticky source-focus trail.
- The copied context includes receipt id, source, action, status, position in the source/action group, path when available, and live/follow-paused mode.
- Copying focus context pauses live focus and locks to the current receipt before writing the context, preventing incoming ledger changes from shifting the inspected receipt during review.
- Added clipboard fallback using a temporary textarea when `navigator.clipboard.writeText` is unavailable.
- Preserved existing source focus navigation, follow lock, jump/return, source-map selection, detail disclosure, lazy detail rendering, and source/action/status filtering.

## Proof markers added

- `session-control-source-focus-copy`
- `session-control-source-focus-context`
- `opencode-session-turn-assistant-copy-shape`

## Claim boundary

This commit records the source-backed slice and state update. Same-head CI/browser/NIM proof is still required for the new head before claiming this slice is browser-proven or parity-complete.
