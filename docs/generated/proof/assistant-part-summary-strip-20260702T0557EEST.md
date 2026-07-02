# Assistant part summary strip proof note

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Live state inspected before this slice

Selected source of truth: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.

PR #3 remained open, non-draft, mergeable, and was selected because it is the active app-change PR for the provided branch. Older open PRs were treated as superseded for this work.

Head inspected before this slice: `2538e5eae09dd875fc65c46659b24b137b666802`.

Workflow state for that head:

- Live WebUI Feature Sprint `28561652571`: in progress during inspection.
- Build Proof `28561652584`: success.
- Fast WebUI Proof `28561652576`: success.
- App Multistep Build Proof `28561652587`: success.
- App Build Proof `28561652572`: success.
- CI `28561652573`: failure.

Failed CI job inspected: `84680354813` / `Smoke Test`.

Failure class: `Validate WebUI proof harness` failed on stale `PROJECT_STATE.md` state assertions. The job log showed the app/UI checks for session-control error card, latest error, copy latest error, and related browser controls passed, but state-file checks for those markers failed.

## Source-backed parity slice

Built: assistant-part summary strip.

OpenCode source used exactly:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `assistantMessages()` derives assistant messages for a user turn.
  - `assistantDerived()` counts visible assistant parts and extracts reasoning heading.
  - `showThinking()` gates the thinking surface.
  - `data-slot="session-turn-assistant-content"` and `data-slot="session-turn-thinking"` expose assistant and working surfaces.

Forge implementation:

- Added `crates/webui/src/chat_ui_assistant_part_summary.html`.
- Updated `crates/webui/src/chat_ui.rs` to mount it after the live turn ordinal and turn summary strip.
- Updated `PROJECT_STATE.md` with the session-control proof markers that the smoke gate requires.

Behavior added:

- Browser session header now shows visible assistant-part count.
- Browser session header now shows latest assistant/reasoning heading when one can be derived from visible assistant content.
- Browser session header now shows idle/working state.
- Browser session header now offers `copy assistant summary`, exporting `{ type, visible_parts, reasoning_heading, working, created_at }`.

Proof markers added:

- `assistant-part-summary-strip`
- `assistant-visible-part-count`
- `assistant-reasoning-heading`
- `assistant-working-state`
- `assistant-copy-part-summary`
- `session-turn-assistant-content`
- `session-turn-thinking`
- `packages/session-ui/src/components/session-turn.tsx`

## Claim boundary

This is a source-backed app/UI slice, not a same-head browser/NIM proof claim.

Do not claim same-head browser proof until the new head completes CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof, and the resulting proof artifacts/screenshots are inspected.
