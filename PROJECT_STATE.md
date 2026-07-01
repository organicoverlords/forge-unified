# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `a617a4e3122d9e6253119b8b7da946a1c670b992`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `a617a4e3122d9e6253119b8b7da946a1c670b992` had CI, Build Proof, App Build Proof, and App Multistep Build Proof success; Fast WebUI Proof and Live WebUI Feature Sprint were red. Fast WebUI Proof reached NVIDIA NIM WebUI streaming and readable PNG capture, then failed only `central_session_turn_ui`; Live WebUI Feature Sprint remained red on browser/full benchmark proof gating.
- Latest implementation slice: WebUI live central session turns now have OpenCode-backed ordinal headers during streaming: `Turn N: <prompt excerpt>`, `data-turn-ordinal`, and `live-session-turn-ordinal`.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_live_turn_ordinals.html` and wired it from `crates/webui/src/chat_ui.rs`.
- The live-turn ordinal surface is interactive product behavior, not docs-only and not cosmetic-only: while a natural-language WebUI prompt is streaming, the user sees a stable central session turn identity that matches durable reloaded turn cards.
- Existing browser-facing controls remain intact: `copy turn`, `retry`, assistant parts, thinking/working message part, session-part hooks, tool card hooks, changed-file/file receipt summaries, action digest summaries, pins, review checklist, and reviewed-action visibility filtering.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.
- Proof markers added/preserved for static/browser checks: live-session-turn-ordinal; data-turn-ordinal; session-turn-central; message-part; assistant-parts; copy-retry-actions; Turn N central session heading; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest; action-digest-pin-visible; pinned-action-summary; copy-pinned-actions; action-pin-count; pinned-action-rail; pinned-action-rail-list; pinned-action-rail-overflow; action-review-checklist; action-reviewed-count; mark-visible-reviewed; copy-reviewed-actions; reviewed-action-summary; reviewed-action-checkbox; action-review-overflow-rail; action-review-overflow-count; focus-unreviewed-action; copy-unreviewed-actions; hide-reviewed-actions; show-reviewed-actions; reviewed-action-visibility-filter; reviewed-action-hidden-count; opencode-showall-overflow-parity; opencode-visible-filter-parity.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; session-control ledger search; search session events; data-session-search-hidden.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially central session turn structure, visible session part continuity, delayed/visible turn surfaces, and turn overflow behavior; `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, especially message/action button behavior and copy affordances.
- Forge implementation paths: `crates/webui/src/chat_ui_live_turn_ordinals.html`, `crates/webui/src/chat_ui.rs`, `PROJECT_STATE.md`, and `docs/generated/proof/live-turn-ordinal-session-surface-20260701T1552Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path and the natural-language NVIDIA NIM WebUI run passes its proof checker.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
