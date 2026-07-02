# Proof — session-control source-map detail disclosure

Date: 2026-07-02 15:45 EEST
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection basis

- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected PR #3 because it is the open, non-draft, mergeable app-change PR for this branch.
- Same-head baseline before this slice: `ede8c6c68ab25ce30962ffb516089e3bdcee98f4`.
- Same-head workflows on that baseline were all green: CI `28587843560`, Build Proof `28587843584`, Fast WebUI Proof `28587843608`, Live WebUI Feature Sprint `28587843553`, App Build Proof `28587843592`, App Multistep Build Proof `28587843606`.
- Live WebUI proof artifact for the baseline: `8038163630` (`live-webui-feature-sprint-proof`), job `84763810602`.

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Exact source affordances copied/adapted:

- `Accordion.Trigger` for user-controlled disclosure.
- `StickyAccordionHeader` around the selected/expanded header shape.
- `data-slot="session-turn-diff-trigger"` for the clickable trigger shape.
- `data-slot="session-turn-diff-meta"` for compact receipt metadata.
- `data-slot="session-turn-diff-view"` for lazy detail rendering.
- The visible/expanded diff pattern where summary rows stay compact and details are shown only after explicit expansion.

## Forge implementation

Changed:

- `crates/webui/src/chat_ui_session_control_source_map.html`

Feature built:

- Adds `show receipt details` / `hide receipt details` to the selected source-map receipt panel.
- Uses `aria-expanded` for the disclosure control.
- Renders the raw selected session-control receipt only when expanded.
- Adds a bounded, scrollable receipt detail view so large receipts do not flood the session lane.
- Keeps `copy selected receipt`, now including detail text only when the disclosure is open.
- Preserves source/action/status filtering, selected receipt highlight, source-map copy, shown/hidden counts, and all previous session-control proof hooks.

New proof hooks:

- `session-control-source-map-selected-detail`
- `session-control-source-map-selected-detail-toggle`
- `opencode-session-turn-diff-view-shape`

## Proof boundary

This commit records source-backed implementation proof. It is not same-head browser/NIM proven until GitHub Actions run on the resulting head and the Live WebUI natural-language NVIDIA NIM artifact/screenshots are inspected.
