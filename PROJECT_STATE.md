# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `5e941c37c825e88e1377e3c89298839e51c1b7a4`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `5e941c37c825e88e1377e3c89298839e51c1b7a4` had Build Proof, App Build Proof, and App Multistep Build Proof success; CI, Fast WebUI Proof, and Live WebUI Feature Sprint were red.
- Latest implementation slice: WebUI action digest pinned-action controls. The action digest now exposes per-action `pin action` / `unpin action`, a live pinned action count, and `copy pinned actions` using pinned cards first and visible cards as fallback.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_action_pins.html` and bundled it from `crates/webui/src/chat_ui.rs` so human action summaries can be pinned without editing the larger action digest implementation.
- Added source-visible proof tokens `action-digest-pin-visible`, `pinned-action-summary`, `copy-pinned-actions`, `action-pin-count`, and `message-action-button-parity` while preserving existing `human-action-summary`, human action summaries, `action-digest-summary`, `action-count-summary-visible`, status filters, `aria-pressed`, visible counts, `copy action digest`, `copy visible actions`, and focus-first-visible evidence.
- Preserved browser-real capture semantics: success still requires a readable PNG; DOM-only or JSON-only fallback is not counted as screenshot proof.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; search session events; data-session-search-hidden; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, especially `writeClipboard` and `MessageActionButton` copy/action-button behavior; `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, especially visible count item filtering and fallback count-summary behavior.
- Forge implementation paths: `crates/webui/src/chat_ui_action_pins.html`, `crates/webui/src/chat_ui.rs`, `PROJECT_STATE.md`, and `docs/generated/proof/action-digest-pinned-actions-20260701T0950Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
