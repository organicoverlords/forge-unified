# Forge Unified — Current State

Updated: 2026-07-02

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this state note: `d2cd879b87f90919a9eb953c8404d03cb9fd44aa`; state-file update head is the next commit after this note.
- PR state verified live: open, non-draft, mergeable.
- Source-of-truth selection: started from the provided branch URL; open PR #3 remains the meaningful current app-change PR for this branch. Open PRs #2 and #1 are older/superseded for this work.
- Latest same-head status inspected before this slice for `74c8d18e93fdbe9d147ada948241754efc65a498`: Build Proof `28563994299`, App Multistep Build Proof `28563994337`, Fast WebUI Proof `28563994321`, and App Build Proof `28563994303` succeeded; CI `28563994308` and Live WebUI Feature Sprint `28563994300` failed.
- Failed CI job inspected: Smoke Test job `84687418579`; `Validate WebUI proof harness` failed because this state file no longer preserved required session-control markers.
- Failed Live WebUI job inspected: `84687418580`; NVIDIA NIM ran with provider `nvidia_nim` and model `deepseek-ai/deepseek-v4-flash`, browser proof passed, but final benchmark quality failed on final answer/report content.
- Latest implementation slice: added keyboard navigation for the WebUI session-control event ledger.
- Do not claim this latest post-state-update head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_session_control_keyboard.html`.
- Updated `crates/webui/src/chat_ui.rs` to load the keyboard-navigation enhancer after session-control search.
- The WebUI now exposes `j/k` and arrow-key focus movement, `e` for next error, `Enter` to open the focused event, `c` to copy the focused receipt, and `/` to focus event search.
- The new UI renders proof hooks for `session-control-keyboard-nav`, `session-control-keyboard-focus`, `session-control-copy-focused-event`, `session-control-next-error`, `session-control-focus-search`, `session-control-open-focused-event`, and `opencode-session-turn-keyboard-interaction`.
- Recorded proof in `docs/generated/proof/session-control-keyboard-nav-20260702T0758EEST.md`.

## Preserved implementation and proof markers

- Preserved compact browser-side turn summary strip shaped after OpenCode's `turnSummaryCommit()` output (`agent · model · duration`).
- Preserved proof markers for browser/static validation: `turn-summary-strip`, `opencode-turn-summary-commit-shape`, `turn-summary-agent-model-duration`, and `data-turn-summary-source`.
- Preserved the prior assistant-part summary strip with visible assistant part count, latest heading, working state, and copyable assistant summary.
- Preserved the assistant copy source strip markers: `assistant-copy-source-state`, `assistant-copy-source-working-guard`, `assistant-copy-source-part-id`, and `copy latest assistant text`.
- Preserved the prior readable latest-error card from `crates/webui/src/chat_ui_error_unwrap.html`.
- Preserved the prior session error history rail from `crates/webui/src/chat_ui_error_history.html`.
- Preserved the prior Live WebUI turn-summary gate fix: required screenshots are `full-benchmark-webui.png`, `tool-lifecycle-webui.png`, and `webui.png`; `event-rail.png` remains optional diagnostic proof.
- Preserved prior central session-turn proof shell and benchmark evidence markers: `Full six-phase agentic benchmark prompt`, `Phase 1` through `Phase 6`, `Founder report`, `Technical report`, `.agent_test/repo_summary.md`, `.agent_test/action_plan.json`, `.agent_test/investigation.md`, `copy benchmark evidence`, `copy turn`, `retry`, assistant parts, thinking/working message part, and central session-turn proof hooks.
- Preserved existing browser-facing controls: session-part hooks, tool card hooks, changed-file/file receipt summaries, action digest summaries, pins, review checklist, reviewed-action visibility filtering, and session-control/search receipts.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.

## Source-backed contracts

- OpenCode source anchor used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `autoScroll.handleInteraction`, `showAll`, `toggleAll`, `visible`, overflow handling, and `data-slot="session-turn-content"` / `data-slot="session-turn-message-container"` interaction surfaces.
- Existing OpenCode source anchors preserved: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, and `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`.
- Forge implementation paths for this slice: `crates/webui/src/chat_ui_session_control_keyboard.html`, `crates/webui/src/chat_ui.rs`, `PROJECT_STATE.md`, `docs/generated/proof/session-control-keyboard-nav-20260702T0758EEST.md`.
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
- crates/webui/src/conversation_controls.rs
- crates/webui/src/chat_ui_session_controls.html
- crates/webui/src/chat_ui_session_control_search.html
