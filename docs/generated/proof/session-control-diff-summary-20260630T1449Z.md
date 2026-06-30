# Proof — WebUI session-control diff summary

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`
Baseline before slice: `10733f606bfd8492bc88c9f1a40932df87fb9c75`

## Selection basis

- Target URL branch remained `mvp/nim-freellmapi-router-20260626`.
- PR #3 remained open, non-draft, and mergeable.
- No newer open PR or pushed non-main branch with meaningful app changes superseded the target branch.

## Baseline proof inspected

Same-head workflows for `10733f606bfd8492bc88c9f1a40932df87fb9c75` were green before this slice:

- CI: `28449513638`
- Build Proof: `28449513772`
- Fast WebUI Proof: `28449513610`
- Live WebUI Feature Sprint: `28449513526`
- App Build Proof: `28449513650`
- App Multistep Build Proof: `28449513613`

Live WebUI artifact inspected by metadata:

- Artifact id: `7983007545`
- Artifact name: `live-webui-feature-sprint-proof`
- Head SHA: `10733f606bfd8492bc88c9f1a40932df87fb9c75`

## OpenCode source backing

Reference paths inspected from upstream OpenCode:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `SessionTurn` computes changed file diffs, limits visible items, supports `showAll`, and renders changed-file summary/diff rows.
  - Relevant behavior copied as Forge-local UI concept: surface a compact per-turn mutation summary instead of hiding it inside raw JSON.
- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - Count summary exposes visible count state and hides inactive count entries.
  - This remains the count/filter backing for the session-control ledger.
- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - Detail panes stay browser-visible and copyable through explicit controls.

## Feature built

Added a Forge-local WebUI session-control diff summary for backend session-control receipts.

The browser control bundle now renders revert receipt payload fields as compact chips:

- `before: <message_count>`
- `after: <message_count>`
- `removed: <message_count>`

This makes the result of `revert latest turn` visible in screenshots without requiring the user to expand/copy raw JSON.

## Files changed

- `crates/webui/src/chat_ui_session_controls.html`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-diff-summary-20260630T1449Z.md`

## Deterministic proof gate

Updated `scripts/smoke/check-session-controls-contract.py` to require:

- `session-control-diff-summary`
- `backend-session-control-diff-summary`
- `backend-session-control-diff-chip`
- `session-control-diff-before`
- `session-control-diff-after`
- `session-control-diff-removed`
- `removed_messages`

Runtime source-path guard remains active: upstream OpenCode paths are allowed in docs/proof/state only, not in Forge browser/runtime session-control payloads.

## Claim boundary

This is a meaningful OpenCode-backed WebUI behavior slice.

Do not claim latest-head WebUI browser proof or parity for the new head until same-head CI / Build Proof / Fast WebUI / Live WebUI / App Build / App Multistep workflows complete and artifacts/screenshots are inspected.
