# Forge Unified — Current State

Updated: 2026-07-02

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this state note: `e0913ea790a0bccb36281f74361c53a8700ec2e7`; state-file update head is the next commit after this note.
- PR state verified live: open, non-draft, mergeable.
- Source-of-truth selection: started from the provided branch URL; open PR #3 remains the meaningful current app-change PR for this branch. Open PRs #2 and #1 are older/superseded for this work.
- Latest same-head status inspected before this slice for `84abb5dc86326606f41832ee55cb1b637e36ccf9`: CI `28584430327`, Build Proof `28584430428`, Fast WebUI Proof `28584430333`, Live WebUI Feature Sprint `28584430492`, App Build Proof `28584430355`, and App Multistep Build Proof `28584430370` all succeeded.
- Live WebUI success boundary for the pre-slice head: NVIDIA NIM natural WebUI proof passed on run `28584430492`; job `84752362047`; artifact `8036844887` (`live-webui-feature-sprint-proof`).
- Latest implementation slice: added OpenCode-shaped WebUI session-control source-map selected receipt panel.
- Do not claim this latest post-state-update head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_session_control_source_map.html`.
- Added `select receipt` controls to source-map rows with `aria-pressed` state.
- Added a visible selected receipt panel above the ledger with source/action/kind/status/receipt metadata.
- Added `copy selected receipt` for compact handoff/audit receipt copying.
- Preserved source/action/status filter chips, clear filter, shown/hidden counts, and per-row `copy source map`.
- Added proof hooks for `session-control-source-map-selected`, `session-control-source-map-select`, `session-control-source-map-selected-copy`, and `opencode-session-turn-sticky-accordion-header-shape`.
- Recorded proof in `docs/generated/proof/session-control-source-map-selected-receipt-20260702T1455EEST.md`.

## Preserved implementation and proof markers

- Preserved compact browser-side turn summary strip shaped after OpenCode's `turnSummaryCommit()` output (`agent · model · duration`).
- Preserved proof markers for browser/static validation: `turn-summary-strip`, `opencode-turn-summary-commit-shape`, `turn-summary-agent-model-duration`, and `data-turn-summary-source`.
- Preserved the prior assistant-part summary strip with visible assistant part count, latest heading, working state, and copyable assistant summary.
- Preserved the assistant copy source strip markers: `assistant-copy-source-state`, `assistant-copy-source-working-guard`, `assistant-copy-source-part-id`, and `copy latest assistant text`.
- Preserved the prior readable latest-error card from `crates/webui/src/chat_ui_error_unwrap.html`.
- Preserved the prior session error history rail from `crates/webui/src/chat_ui_error_history.html`.
- Preserved the prior session-control keyboard navigation markers: `session-control-keyboard-nav`, `session-control-keyboard-focus`, `session-control-copy-focused-event`, `session-control-next-error`, `session-control-focus-search`, `session-control-open-focused-event`, and `opencode-session-turn-keyboard-interaction`.
- Preserved the grouped session-control ledger markers: `session-control-grouped-ledger`, `session-control-group-strip`, `session-control-group-row`, `session-control-group-toggle`, `session-control-group-detail`, `session-control-group-count`, `session-control-group-visible`, `session-control-group-overflow`, `session-control-group-show-all`, `session-control-group-show-less`, `session-control-group-status-summary`, `session-control-group-status-chip`, `session-control-group-copy-summary`, `session-control-group-path-meta`, `session-control-group-directory`, `session-control-group-filename`, `session-control-group-meta`, `session-control-group-latest`, `opencode-session-turn-diffs-group-shape`, `opencode-session-turn-diff-path-shape`, and `opencode-session-turn-diff-meta-shape`.
- Preserved source-map markers: `session-control-source-map`, `session-control-source-map-source`, `session-control-source-map-action`, `session-control-source-map-receipt`, `copy-session-control-source-map`, `session-control-source-map-filter`, `session-control-source-map-clear`, `source-map-filtered-event-count`, and `opencode-session-turn-diff-meta-shape`.
- Preserved selected source-map markers: `session-control-source-map-selected`, `session-control-source-map-select`, `session-control-source-map-selected-copy`, and `opencode-session-turn-sticky-accordion-header-shape`.
- Preserved the prior Live WebUI turn-summary gate fix: required screenshots are `full-benchmark-webui.png`, `tool-lifecycle-webui.png`, and `webui.png`; `event-rail.png` remains optional diagnostic proof.
- Preserved prior central session-turn proof shell and benchmark evidence markers: `Full six-phase agentic benchmark prompt`, `Phase 1` through `Phase 6`, `Founder report`, `Technical report`, `.agent_test/repo_summary.md`, `.agent_test/action_plan.json`, `.agent_test/investigation.md`, `copy benchmark evidence`, `copy turn`, `retry`, assistant parts, thinking/working message part, and central session-turn proof hooks.
- Preserved existing browser-facing controls: session-part hooks, tool card hooks, changed-file/file receipt summaries, action digest summaries, pins, review checklist, reviewed-action visibility filtering, and session-control/search receipts.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.

## Source-backed contracts

- OpenCode source anchor used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `StickyAccordionHeader`, `Accordion.Trigger`, `data-slot="session-turn-diff-trigger"`, `data-slot="session-turn-diff-meta"`, `data-slot="session-turn-diff-path"`, `DiffChanges`, `getDirectory()`, and `getFilename()`.
- Existing OpenCode source anchors preserved: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, and `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`.
- Forge implementation paths for this slice: `crates/webui/src/chat_ui_session_control_source_map.html`, `PROJECT_STATE.md`, `docs/generated/proof/session-control-source-map-selected-receipt-20260702T1455EEST.md`.
- Browser proof gap remains explicit until a same-head workflow artifact from the new head contains valid readable PNGs from the browser-real capture path and the natural-language NVIDIA NIM WebUI run passes its proof checker.

## State markers kept for CI proof harness

- backend-backed session controls
- checkpoint, fork, revert latest turn, and retry source
- Forge-local session control receipts
- session control event ledger
- copy session control event
- session-control event disclosure
- session-control count summary
- status filters
- session-control diff summary
- before/after/removed message chips
- session-control duration summary
- start/completion/duration chips
- session-control ledger overflow
- show older/show less overflow toggle
- visible ledger count
- hidden older receipt row
- session-control ledger export
- copy all events
- session-control error card
- copy latest error
- latest session-control error
- stable session-control receipt identity
- receipt_id
- sequence
- forge.webui.session_controls
- session-control ledger search
- search session events
- data-session-search-hidden
- session-control keyboard navigation
- crates/webui/src/conversation_controls.rs
- crates/webui/src/chat_ui_session_controls.html
- crates/webui/src/chat_ui_session_control_search.html
- session-control source map
- copy session-control source map
- source/action/receipt metadata chips
- session-control source-map filter
- clear source-map filter
- source-map filtered event count
- session-control source-map selected receipt
- select receipt
- copy selected receipt
