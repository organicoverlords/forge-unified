# Proof note — WebUI action review checklist

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`
Prior inspected head: `ad425044946860c45ee317814f885ffffbf458e3`

## Selection basis

The source-of-truth target URL remains `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`. PR #3 remains open, non-draft, and mergeable. No newer open PR or branch superseded the target branch for meaningful app changes.

## Pre-slice workflow state inspected

Same-head workflows for `ad425044946860c45ee317814f885ffffbf458e3`:

- CI `28515560373`: success.
- Build Proof `28515560300`: success.
- App Build Proof `28515560298`: success.
- App Multistep Build Proof `28515560343`: success.
- Fast WebUI Proof `28515560292`: failure in job `84526315427` at readable browser proof capture after the fast NVIDIA NIM WebUI stream.
- Live WebUI Feature Sprint `28515560294`: failure in job `84526315313`; the natural feature-build prompt step was skipped after the earlier failure and the quality/evidence checker failed.
- Fast artifact inspected/recorded: `8009167241`.

## OpenCode source backing

Exact upstream source paths used:

- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - `writeClipboard` fallback copy behavior.
  - `MessageActionButton` action-control shape.
  - `Checkbox` import pattern for reviewable binary UI state.
- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - counted visible summary behavior.
- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - compact visible-part session surface and delayed proof visibility pattern.

## Implemented Forge slice

Added real browser UI behavior for long tool/action runs:

- `crates/webui/src/chat_ui_action_review.html`
  - Adds a per-action `reviewed` checkbox to each `.action-summary-card`.
  - Adds an action digest reviewed-count chip: `X/Y reviewed`.
  - Adds `mark visible reviewed` so the current filtered tool subset can be audited in one click.
  - Adds `copy reviewed actions` for proof/final-answer audit receipts.
  - Adds proof markers: `action-review-checklist`, `action-reviewed-count`, `mark-visible-reviewed`, `copy-reviewed-actions`, `reviewed-action-summary`, `reviewed-action-checkbox`, `opencode-checkbox-parity`, `opencode-message-action-parity`.
- `crates/webui/src/chat_ui.rs`
  - Wires `chat_ui_action_review.html` into the bundled WebUI.
- `PROJECT_STATE.md`
  - Records selected branch, live PR state, failed workflow state, source backing, claim boundary, and changed files.

## Claim boundary

This is not a same-head browser-proof acceptance claim. The known remaining blocker is readable browser visual capture in GitHub Actions. A same-head Fast WebUI / Live WebUI artifact must contain valid browser-rendered screenshots before this branch can be described as screenshot-proven.
