# Proof — session-control source-map selected receipt panel

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection basis

- Source-of-truth URL branch: `mvp/nim-freellmapi-router-20260626`.
- PR #3 remains open, non-draft, and mergeable before this slice.
- Older open PRs #1 and #2 are superseded by PR #3 for the app-change work.
- Exact pre-slice head `84abb5dc86326606f41832ee55cb1b637e36ccf9` had all six required workflows passing.

## Pre-slice live proof inspected

- CI: `28584430327` success.
- Build Proof: `28584430428` success.
- Fast WebUI Proof: `28584430333` success.
- Live WebUI Feature Sprint: `28584430492` success.
- App Build Proof: `28584430355` success.
- App Multistep Build Proof: `28584430370` success.
- Live WebUI job: `84752362047` success.
- Live WebUI artifact: `8036844887` (`live-webui-feature-sprint-proof`) for head `84abb5dc86326606f41832ee55cb1b637e36ccf9`.

## OpenCode source backing used exactly

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-slot="session-turn-diffs-header"`
  - `data-slot="session-turn-diff-trigger"`
  - `data-slot="session-turn-diff-path"`
  - `data-slot="session-turn-diff-meta"`
  - `StickyAccordionHeader`
  - `Accordion.Trigger`
  - `DiffChanges`
  - `getDirectory()` / `getFilename()` path/meta separation

## Forge implementation

Changed source path:

- `crates/webui/src/chat_ui_session_control_source_map.html`

Implemented behavior:

- Added per-row `select receipt` control for session-control source-map rows.
- Added selected receipt panel above the ledger with source/action/kind/status/receipt metadata.
- Added `copy selected receipt` for a compact audit handoff.
- Preserved filter chips, clear filter, shown/hidden counts, and existing per-row `copy source map`.
- Added browser proof hooks:
  - `session-control-source-map-selected`
  - `session-control-source-map-select`
  - `session-control-source-map-selected-copy`
  - `opencode-session-turn-sticky-accordion-header-shape`

## Claim boundary

This source slice is committed, but the new post-slice head is not same-head browser/NIM proven until GitHub Actions complete on the exact head and the Live WebUI proof artifact/screenshots are inspected.
