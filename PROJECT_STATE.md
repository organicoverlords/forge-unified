# Forge Unified — Current State

Updated: 2026-07-02

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this state note: `c33419e4f7d960e09fe59b6601f5f372c1844648`; state-file update head is the next commit after this note.
- PR state verified live: open, non-draft, mergeable.
- Source-of-truth selection: started from the provided branch URL; open PR #3 remains the meaningful current app-change PR for this branch. Open PRs #2 and #1 are older/superseded for this work.
- Latest same-head status inspected before this slice for `2538e5eae09dd875fc65c46659b24b137b666802`: Build Proof `28561652584`, Fast WebUI Proof `28561652576`, App Build Proof `28561652572`, and App Multistep Build Proof `28561652587` succeeded; Live WebUI Feature Sprint `28561652571` was still in progress; CI `28561652573` failed in Smoke Test.
- Failed CI job inspected: `84680354813`, failed only `Validate WebUI proof harness` after source/UI checks for session-control features passed.
- Failure class: state-file assertions were stale. The app/UI checks passed for session-control error card, latest error, copy latest error, backend session controls, ledgers, filters, overflow, diff/duration chips, and search hooks, but `PROJECT_STATE.md` did not record those markers.
- Latest implementation slice: added an assistant-part summary strip that surfaces visible assistant-part count, latest reasoning/section heading, working state, and copyable assistant summary.
- Do not claim the new post-state-update head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_assistant_part_summary.html`.
- Updated `crates/webui/src/chat_ui.rs` to load the assistant-part summary after the existing live turn ordinal and turn summary strip.
- The new UI renders compact chips for `assistant-visible-part-count`, `assistant-reasoning-heading`, and `assistant-working-state`, plus `assistant-copy-part-summary`.
- The strip records its source marker as `packages/session-ui/src/components/session-turn.tsx` and mirrors OpenCode's SessionTurn concepts: assistant content visibility, thinking state, assistant-derived visible-part count, and reasoning heading extraction.
- Recorded proof in `docs/generated/proof/assistant-part-summary-strip-20260702T0557EEST.md`.

## Preserved implementation and proof markers

- Preserved compact browser-side turn summary strip shaped after OpenCode's `turnSummaryCommit()` output (`▣ agent · model · duration`).
- Preserved proof markers for browser/static validation: `turn-summary-strip`, `opencode-turn-summary-commit-shape`, `turn-summary-agent-model-duration`, and `data-turn-summary-source`.
- Preserved the prior readable latest-error card from `crates/webui/src/chat_ui_error_unwrap.html`.
- Preserved the prior session error history rail from `crates/webui/src/chat_ui_error_history.html`.
- Preserved the prior Live WebUI turn-summary gate fix: required screenshots are `full-benchmark-webui.png`, `tool-lifecycle-webui.png`, and `webui.png`; `event-rail.png` remains optional diagnostic proof.
- Preserved prior central session-turn proof shell and benchmark evidence markers: `Full six-phase agentic benchmark prompt`, `Phase 1` through `Phase 6`, `## Founder report`, `## Technical report`, `.agent_test/repo_summary.md`, `.agent_test/action_plan.json`, `.agent_test/investigation.md`, `copy benchmark evidence`, `copy turn`, `retry`, assistant parts, thinking/working message part, and central session-turn proof hooks.
- Preserved existing browser-facing controls: session-part hooks, tool card hooks, changed-file/file receipt summaries, action digest summaries, pins, review checklist, reviewed-action visibility filtering, and session-control/search receipts.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.

## Session-control proof state required by smoke gate

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

## Source-backed contracts

- OpenCode source anchor used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially assistant-message derivation, assistant visible part counting, reasoning heading extraction, thinking/working state, and `data-slot="session-turn-assistant-content"` / `data-slot="session-turn-thinking"` surfaces.
- Existing OpenCode source anchors preserved: `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`.
- Forge implementation paths for this slice: `crates/webui/src/chat_ui_assistant_part_summary.html`, `crates/webui/src/chat_ui.rs`, `PROJECT_STATE.md`, `docs/generated/proof/assistant-part-summary-strip-20260702T0557EEST.md`.
- Browser proof gap remains explicit until a same-head workflow artifact from the new head contains valid readable PNGs from the browser-real capture path and the natural-language NVIDIA NIM WebUI run passes its proof checker.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
