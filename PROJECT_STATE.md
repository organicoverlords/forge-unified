# Forge Unified — Current State

Updated: 2026-07-02

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- PR state verified live before this slice: open, non-draft, mergeable.
- Source-of-truth selection: started from the provided branch URL; open PR #3 remains the meaningful current app-change PR for this branch. Open PRs #2 and #1 are older/superseded for this work.
- Latest same-head status inspected before this slice for `473f2986676feecfd0ddfebc6c0cd4b646756aae`: CI `28614040714`, Build Proof `28614040665`, Fast WebUI Proof `28614040656`, Live WebUI Feature Sprint `28614040690`, App Build Proof `28614040694`, and App Multistep Build Proof `28614040669` all succeeded.
- Live WebUI proof for the pre-slice head: run `28614040690`, job `84853287573`, artifact `8049090416`; the workflow completed the natural WebUI feature-build prompt, full benchmark evidence/quality check, and proof upload successfully.
- Latest implementation slice: added OpenCode-shaped WebUI session-control source-focus keyboard commands.
- Do not claim this latest post-state-update head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_session_control_source_focus_trail.html`.
- Added keyboard command map to the sticky selected source-focus trail: `[` previous source receipt, `]` next source receipt, `f` pause/resume focus follow, `c` copy focus context, and `r` return to selected receipt.
- Keyboard commands are ignored inside inputs, textareas, selects, contenteditable fields, and role=textbox surfaces so natural prompt entry is not hijacked.
- Keyboard commands reuse the same visible buttons through `data-source-focus-command`, keeping click and keyboard behavior aligned.
- Copying focus context still pauses live focus and locks to the current receipt, so incoming ledger mutations do not silently move the inspected/copied row.
- Added proof hooks for `session-control-source-focus-keyboard`, `session-control-source-focus-command-map`, and `opencode-session-turn-keyboard-interaction-shape`.
- Recorded proof in `docs/generated/proof/session-control-source-focus-keyboard-20260702T2250EEST.md`.

## Preserved implementation and proof markers

- Preserved compact browser-side turn summary strip shaped after OpenCode's `turnSummaryCommit()` output (`agent · model · duration`).
- Preserved assistant-part summary strip, assistant copy source strip, readable latest-error card, session error history rail, action summaries, pins, review checklist, and reviewed-action visibility filtering.
- Preserved session-control keyboard navigation markers: `session-control-keyboard-nav`, `session-control-keyboard-focus`, `session-control-copy-focused-event`, `session-control-next-error`, `session-control-focus-search`, `session-control-open-focused-event`, and `opencode-session-turn-keyboard-interaction`.
- Preserved grouped session-control ledger markers: `session-control-grouped-ledger`, `session-control-group-strip`, `session-control-group-row`, `session-control-group-toggle`, `session-control-group-detail`, `session-control-group-count`, `session-control-group-visible`, `session-control-group-overflow`, `session-control-group-show-all`, `session-control-group-show-less`, `session-control-group-status-summary`, `session-control-group-status-chip`, `session-control-group-copy-summary`, `session-control-group-path-meta`, `session-control-group-directory`, `session-control-group-filename`, `session-control-group-meta`, `session-control-group-latest`, `opencode-session-turn-diffs-group-shape`, `opencode-session-turn-diff-path-shape`, and `opencode-session-turn-diff-meta-shape`.
- Preserved source-map markers: `session-control-source-map`, `session-control-source-map-source`, `session-control-source-map-action`, `session-control-source-map-receipt`, `copy-session-control-source-map`, `session-control-source-map-filter`, `session-control-source-map-clear`, `source-map-filtered-event-count`, and `opencode-session-turn-diff-meta-shape`.
- Preserved selected source-map markers: `session-control-source-map-selected`, `session-control-source-map-select`, `session-control-source-map-selected-copy`, and `opencode-session-turn-sticky-accordion-header-shape`.
- Preserved selected detail markers: `session-control-source-map-selected-detail`, `session-control-source-map-selected-detail-toggle`, `opencode-session-turn-diff-view-shape`, `session-control-source-map-lazy-detail`, `session-control-source-map-selected-detail-pending`, and `opencode-session-turn-diff-view-lazy-shape`.
- Preserved source jump markers: `session-control-source-map-jump`, `session-control-source-map-jump-target`, and `opencode-session-turn-diff-path-shape`.
- Preserved source focus markers: `session-control-source-focus-trail`, `session-control-source-focus-return`, `session-control-source-focus-nav`, `session-control-source-focus-position`, `session-control-source-focus-lock`, `session-control-source-focus-follow-paused`, `session-control-source-focus-copy`, `session-control-source-focus-context`, `session-control-source-focus-keyboard`, `session-control-source-focus-command-map`, `opencode-session-turn-autoscroll-pause-shape`, `opencode-session-turn-assistant-copy-shape`, `opencode-session-turn-keyboard-interaction-shape`, `opencode-session-turn-diffs-toggle`, `opencode-session-turn-diffs-more`, `opencode-session-turn-diffs-group-shape`, and `opencode-session-turn-diff-meta-shape`.
- Preserved prior central session-turn proof shell and benchmark evidence markers: `Full six-phase agentic benchmark prompt`, `Phase 1` through `Phase 6`, `Founder report`, `Technical report`, `.agent_test/repo_summary.md`, `.agent_test/action_plan.json`, `.agent_test/investigation.md`, `copy benchmark evidence`, `copy turn`, `retry`, assistant parts, thinking/working message part, and central session-turn proof hooks.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.

## Source-backed contracts

- OpenCode source anchor used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `createAutoScroll`, `autoScroll.handleInteraction`, `toggleAll()` calling `autoScroll.pause()`, `showAll`, `overflow`, `visible`, `StickyAccordionHeader`, `Accordion.Trigger`, `data-slot="session-turn-diff-trigger"`, `data-slot="session-turn-diff-path"`, `data-slot="session-turn-diff-meta"`, `showAssistantCopyPartID`, and `assistantCopyPartID`.
- Existing OpenCode source anchors preserved: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, and `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`.
- Forge implementation paths for this slice: `crates/webui/src/chat_ui_session_control_source_focus_trail.html`, `PROJECT_STATE.md`, `docs/generated/proof/session-control-source-focus-keyboard-20260702T2250EEST.md`.
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
- session-control source-map selected detail
- session-control source-map lazy detail
- session-control source-map selected detail pending
- session-control source-map jump
- jump to receipt source
- session-control source-map jump target
- session-control source focus trail
- return to selected receipt
- session-control source focus navigation
- previous source receipt
- next source receipt
- session-control source focus position
- session-control source focus follow lock
- pause focus follow
- resume live focus
- copy focus context
- session-control source focus context
- session-control source focus keyboard
- session-control source focus command map
- keys: [ prev · ] next · f follow · c copy · r return
- opencode-session-turn-autoscroll-pause-shape
- opencode-session-turn-assistant-copy-shape
- opencode-session-turn-keyboard-interaction-shape
- opencode-session-turn-diffs-toggle
- opencode-session-turn-diffs-more
- opencode-session-turn-diffs-group-shape
- opencode-session-turn-diff-view-shape
- opencode-session-turn-diff-view-lazy-shape
- opencode-session-turn-diff-path-shape
- opencode-session-turn-diff-meta-shape
