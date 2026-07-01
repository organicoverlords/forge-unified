# Proof — action review visibility + bounded DOM PNG fallback

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

PR #3 remains the selected work because the target branch from the provided source URL is still the active open PR branch. No newer open PR was found that supersedes PR #3.

## Workflow state inspected before this slice

Same-head `bf4644c3447384aa6dfca9e918d645772ee59bd6`:

- CI `28522568895`: success.
- Build Proof `28522568582`: success.
- App Build Proof `28522568588`: success.
- App Multistep Build Proof `28522568751`: success.
- Fast WebUI Proof `28522568686`: failed at `capture readable browser proof` after the NIM/WebUI stream; job `84550379297`; artifact `8012189474`.
- Live WebUI Feature Sprint `28522569041`: failed at `browser proof tool lifecycle`; job `84550381067`; checker then reported `missing full benchmark conversation or stream artifact`; artifact `8012182001`.

## Source-backed OpenCode parity slice

Implemented Forge WebUI action-review visibility controls:

- `hide reviewed actions`.
- `show reviewed`.
- `reviewed-action-hidden-count`.
- Reviewed action cards receive `data-action-review-hidden=true` when hidden.
- Existing review checklist, unreviewed rail, and copy actions remain intact.

OpenCode source backing:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`: `showAll`, `overflow`, `visible`, and `session-turn-diffs-more` behavior.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`: message/action button behavior.
- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`: counted tool/action summary behavior.

Forge implementation paths:

- `crates/webui/src/chat_ui_action_review.html`
- `scripts/smoke/capture-browser-proof.sh`
- `PROJECT_STATE.md`
- this proof file

## Browser-proof harness change

Added a bounded CI-only DOM summary PNG fallback after Chrome screenshot/PDF visual capture fails. This is intentionally labeled in JSON metadata:

- `dom_summary_png_fallback: true`
- `dom_summary_png_is_browser_rendered: false`
- `not_full_browser_rendered_visual_proof: true`

Claim boundary:

- This fallback should prevent proof workflows from losing all readable PNG artifacts when GitHub Actions Chrome visual capture fails.
- It is not a full browser-rendered screenshot and must not be claimed as browser-rendered parity.
- Full browser screenshot proof remains required before claiming WebUI browser parity.

## New static/proof markers

- `hide-reviewed-actions`
- `show-reviewed-actions`
- `reviewed-action-visibility-filter`
- `reviewed-action-hidden-count`
- `opencode-visible-filter-parity`
- `dom-summary-png`
- `not_full_browser_rendered_visual_proof`
