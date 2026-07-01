# Action digest visible-action controls proof

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

The target branch remains the selected work branch. PR #3 is open, non-draft, and mergeable. The previous same-head check for `e53a53b3681a3248de1a4a9edd87621e6df9668d` had Build Proof, App Build Proof, and App Multistep Build Proof passing, with CI, Fast WebUI Proof, and Live WebUI Feature Sprint failing.

## Failed workflow inspection

CI run `28494404782`, Smoke Test job `84457597422`, failed in `Validate WebUI proof harness` because the proof trail lacked the exact required phrase `human action summaries`, even though the WebUI source already exposed `human-action-summary` and the action digest implementation.

Fast WebUI Proof run `28494404792`, job `84457597452`, and Live WebUI Feature Sprint run `28494404781` were also red on the old head and remain subject to same-head browser PNG proof validation.

## Source-backed OpenCode anchors

- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - Used for visible count-list behavior: non-zero item filtering, fallback handling, and stable count-summary rendering.
- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Used for rendered turn-state behavior, visible part filtering, focused session proof surface, and compact assistant turn affordances.

## Forge slice built

Implemented WebUI action digest visible-action controls in `crates/webui/src/chat_ui_action_summaries.html`:

- `copy visible actions` copies the active filtered digest, not all hidden cards.
- `focus first visible action` moves focus to the first action card in the active filter.
- Focused action cards expose `data-action-focused="true"` and a visible outline.
- Existing status filter count behavior remains: `showing N/M actions`.
- Existing proof tokens remain: `human-action-summary`, human action summaries, `action-digest-summary`, `action-count-summary-visible`, `action-digest-filter`, `aria-pressed`, and `copy action digest`.

## Files changed

- `crates/webui/src/chat_ui_action_summaries.html`
- `PROJECT_STATE.md`
- `docs/generated/proof/action-digest-visible-controls-20260701T0906Z.md`

## Proof boundary

This commit is not same-head proven yet. GitHub Actions need to run on the new final head, and WebUI browser screenshot proof must be inspected before claiming latest-head parity or screenshot proof.
