# Proof — Action review overflow rail

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`
Prior inspected head: `b8438f015c1811e6ea91e3a574d555db5113602f`

## Selection basis

- The target URL branch remained the source of truth.
- PR #3 remained open, non-draft, and mergeable.
- No newer open PR or non-main branch was found that superseded this target branch for meaningful app work.

## Live workflow state inspected before this slice

Same-head workflows for `b8438f015c1811e6ea91e3a574d555db5113602f`:

- CI `28518796838`: success.
- Build Proof `28518796763`: success.
- App Build Proof `28518796778`: success.
- App Multistep Build Proof `28518796765`: success.
- Fast WebUI Proof `28518796801`: failure, job `84537235175`, artifact `8010551857`.
- Live WebUI Feature Sprint `28518796746`: failure, job `84537234745`, artifact `8010543153`.

Failure summary: Rust/build gates passed. The NVIDIA NIM WebUI stream reached browser proof capture, then failed at readable visual proof capture. No same-head screenshot proof is claimed.

## OpenCode source backing

Source paths recorded exactly:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Used for `showAll`, `overflow`, `visible`, `session-turn-diffs-more`, visible session part filtering, and compact proof/error surface behavior.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - Used for copy/message action behavior.
- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - Used for visible counted summary behavior.

## Forge implementation

Changed:

- `crates/webui/src/chat_ui_action_review.html`
  - Added `action-review-overflow-rail`.
  - Added first-four unreviewed visible actions.
  - Added `+N more unreviewed` overflow count.
  - Added click-to-focus source-card behavior via `focus-unreviewed-action`.
  - Added `copy unreviewed actions` backlog export.
- `PROJECT_STATE.md`
  - Updated current state, source anchors, proof markers, and claim boundary.

Proof markers:

- `action-review-overflow-rail`
- `action-review-overflow-count`
- `focus-unreviewed-action`
- `copy-unreviewed-actions`
- `opencode-showall-overflow-parity`

## Claim boundary

This is a source-backed WebUI behavior slice, not docs-only and not cosmetic-only. It is not a full OpenCode parity claim. Browser proof remains blocked until a same-head workflow artifact contains a readable PNG captured through the browser-real visual path with NVIDIA NIM-only natural-language WebUI execution.
