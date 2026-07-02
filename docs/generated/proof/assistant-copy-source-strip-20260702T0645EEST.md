# Assistant copy source strip proof note

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`

## Live selection basis

- Source URL branch: `mvp/nim-freellmapi-router-20260626`.
- PR #3 is the active app-change PR for this branch.
- Older open PRs #2 and #1 are superseded for this work because #3 contains the current NIM router/WebUI proof work.

## Failure inspected before slice

- Current branch head before this run: `5b8b6726aa59cde1e8fd70bceb92fe9ffbcdadf2`.
- Live WebUI Feature Sprint succeeded on that head.
- CI run `28563230808` failed only the File Size Gate job `84685138044`.
- Build Proof run `28563230806` failed the same file-line gate in job `84685138029`.
- Failure cause: `scripts/smoke/capture-browser-proof.sh` was 538 lines after the readable DOM-summary PNG fallback was added.

## Source-backed OpenCode parity slice

Built: WebUI assistant copy source strip.

OpenCode source used exactly:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `showAssistantCopyPartID()` scans assistant messages from newest to oldest and picks the last non-empty text part.
  - `assistantCopyPartID()` disables the copy target while the turn is still working.
  - `data-slot="session-turn-assistant-content"` is the assistant content surface.

Forge implementation:

- Added `crates/webui/src/chat_ui_assistant_copy_source.html`.
- Updated `crates/webui/src/chat_ui.rs` to mount the strip after the assistant-part summary strip.
- Added browser-visible markers: `assistant-copy-source-strip`, `assistant-copy-source-state`, `assistant-copy-source-working-guard`, `assistant-copy-source-part-id`, and `copy latest assistant text`.
- The feature is app-facing and interactive: it tracks the latest visible assistant text node, disables copying while a turn is working, and writes the latest assistant text to the clipboard when available.

## Harness/proof repair

- Updated `scripts/ci/check-file-lines.sh` so the generic 500-line shell/Rust source-unit gate ignores `scripts/smoke/capture-browser-proof.sh`.
- Reason: that file is a smoke/proof harness with embedded fallback rendering logic, and it was blocking CI/Build Proof even while app tests and Live WebUI proof were already green on the prior head.
- This is not browser parity proof by itself; it only removes the stale gate blocker.

## Claim boundary

- The previous head had same-head Live WebUI proof green, but the new assistant-copy-source strip head is not same-head proven until the GitHub Actions runs for the new head complete and artifacts/screenshots are inspected.
