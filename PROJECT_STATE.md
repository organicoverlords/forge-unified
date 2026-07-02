# Forge Unified — Current State

Updated: 2026-07-02

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head after this note: `6cf942a0cfbe69a00d1b410770619f8293779e9c`; state-file update head is the next commit after this note.
- PR state verified live: open, non-draft, mergeable.
- Source-of-truth selection: started from the provided branch URL; only open PRs found were #3, #2, and #1, and #3 remains the meaningful current app-change PR for this branch.
- Baseline same-head workflow state inspected for `6769f96f4a3e945d4ab05bc7466fb95c07253206`: CI `28556582482`, Build Proof `28556582475`, Fast WebUI Proof `28556582470`, Live WebUI Feature Sprint `28556582473`, App Build Proof `28556582469`, and App Multistep Build Proof `28556582480` all succeeded.
- Baseline Fast WebUI artifact inspected: `8025862263` / `fast-webui-proof`.
- Baseline Live WebUI artifact inspected: `8026081210` / `live-webui-feature-sprint-proof`. The live sprint proof passed with NVIDIA NIM, natural feature-build, full benchmark checker, workflow checker, manifest checker, quality score, and PNG browser proof screenshots.
- Latest implementation slice: added a readable browser session-control error history rail. The UI now keeps recent session-control errors in a compact rail, shows the newest three by default, supports `show N older errors` / `show fewer errors`, preserves raw event data, and provides `copy error history`.
- Do not claim the new post-state-update head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_error_history.html`.
- Updated `crates/webui/src/chat_ui.rs` to load the new session error history rail after the readable session-control error card.
- The new UI listens for `forge:session-control` error receipts and renders a browser-side `session-error-history-rail` in the session header.
- The history rail uses OpenCode-shaped overflow behavior: newest three errors visible by default, explicit `show N older errors` expansion, and `show fewer errors` collapse.
- Each rendered history row includes a readable summary and preserves the raw event as `data-session-control-event`.
- Added `copy error history` so the user can copy readable summaries plus raw receipt/event JSON without hunting through the event ledger.
- Recorded proof in `docs/generated/proof/session-error-history-rail-20260702T0355EEST.md`.
- Preserved the prior readable latest-error card from `crates/webui/src/chat_ui_error_unwrap.html`.
- Preserved the prior Live WebUI turn-summary gate fix: required screenshots are `full-benchmark-webui.png`, `tool-lifecycle-webui.png`, and `webui.png`; `event-rail.png` remains optional diagnostic proof.
- Preserved prior central session-turn proof shell and benchmark evidence markers: `Full six-phase agentic benchmark prompt`, `Phase 1` through `Phase 6`, `## Founder report`, `## Technical report`, `.agent_test/repo_summary.md`, `.agent_test/action_plan.json`, `.agent_test/investigation.md`, `copy benchmark evidence`, `copy turn`, `retry`, assistant parts, thinking/working message part, and central session-turn proof hooks.
- Preserved existing browser-facing controls: session-part hooks, tool card hooks, changed-file/file receipt summaries, action digest summaries, pins, review checklist, reviewed-action visibility filtering, and session-control/search receipts.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.
- Proof markers added/preserved for static/browser checks: session-error-history-rail; session-error-history-overflow; copy-session-control-error-history; session-error-unwrap-card; opencode-error-unwrap-parity; opencode-error-card-parity; readable-session-control-error-card; latest-session-control-error-unwrapped; copy-session-control-error-unwrapped; packages/session-ui/src/components/session-turn.tsx; full-six-phase-agentic-benchmark-prompt; Full six-phase agentic benchmark prompt; benchmark-phase-labels; Phase 1; Phase 2; Phase 3; Phase 4; Phase 5; Phase 6; live-session-turn-ordinal; data-turn-ordinal; session-turn-central; session-turn-content; session-turn-message-container; session-turn-message-content; session-turn-assistant-content; session-turn-thinking; session-turn-diffs; message-part; assistant-parts; copy-retry-actions; benchmark-artifact-evidence-summary; session-turn-benchmark-evidence; founder-report-marker; technical-report-marker; repo-summary-artifact-marker; action-plan-artifact-marker; investigation-artifact-marker; copy benchmark evidence; natural-feature browser proof fallback; direct Chrome natural browser proof; Turn 1: Central session turn proof shell; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest; action-digest-pin-visible; pinned-action-summary; copy-pinned-actions; action-pin-count; pinned-action-rail; pinned-action-rail-list; pinned-action-rail-overflow; action-review-checklist; action-reviewed-count; mark-visible-reviewed; copy-reviewed-actions; reviewed-action-summary; reviewed-action-checkbox; action-review-overflow-rail; action-review-overflow-count; focus-unreviewed-action; copy-unreviewed-actions; hide-reviewed-actions; show-reviewed-actions; reviewed-action-visibility-filter; reviewed-action-hidden-count; opencode-showall-overflow-parity; opencode-visible-filter-parity.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; session-control ledger search; search session events; data-session-search-hidden.

## Source-backed contracts

- OpenCode source anchor used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `unwrap()`, visible error-card rendering, and `showAll`/`overflow`/`visible` diff-group behavior.
- Existing OpenCode source anchors preserved: `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`.
- Forge implementation paths for this slice: `crates/webui/src/chat_ui_error_history.html`, `crates/webui/src/chat_ui.rs`, `PROJECT_STATE.md`, `docs/generated/proof/session-error-history-rail-20260702T0355EEST.md`.
- Browser proof gap remains explicit until a same-head workflow artifact from the new head contains valid readable PNGs from the browser-real capture path and the natural-language NVIDIA NIM WebUI run passes its proof checker.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
