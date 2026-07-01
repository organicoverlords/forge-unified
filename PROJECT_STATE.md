# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `ad425044946860c45ee317814f885ffffbf458e3`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `ad425044946860c45ee317814f885ffffbf458e3` had CI, Build Proof, App Build Proof, and App Multistep Build Proof success; Fast WebUI Proof and Live WebUI Feature Sprint were red at browser proof capture after the NVIDIA NIM WebUI stream.
- Latest implementation slice: WebUI action digest now has an OpenCode-backed action review checklist: per-action reviewed checkboxes, reviewed count summary, mark visible reviewed, and copy reviewed actions.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_action_review.html` with browser-side action review controls for every human-readable action summary card.
- Updated `crates/webui/src/chat_ui.rs` to include the new action review enhancement in the bundled single-page chat UI.
- The review checklist is interactive product behavior, not docs-only and not cosmetic-only: the user can mark visible or individual actions as reviewed and copy the reviewed-action checklist for final-answer/proof auditing.
- Proof markers added for static/browser checks: action-review-checklist; action-reviewed-count; mark-visible-reviewed; copy-reviewed-actions; reviewed-action-summary; reviewed-action-checkbox; opencode-checkbox-parity; opencode-message-action-parity.
- Preserved browser proof gap explicitly: accepted screenshot proof still requires a readable PNG produced by the browser-real visual capture path; DOM-only, JSON-only, or fabricated image fallback is not counted.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; session-control ledger search; search session events; data-session-search-hidden; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest; action-digest-pin-visible; pinned-action-summary; copy-pinned-actions; action-pin-count; pinned-action-rail; pinned-action-rail-list; pinned-action-rail-overflow; action-review-checklist; action-reviewed-count; mark-visible-reviewed; copy-reviewed-actions; reviewed-action-summary; reviewed-action-checkbox.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, especially `writeClipboard`, `MessageActionButton`, checkbox imports, and action button behavior; `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, especially visible counted summaries; `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially visible session part filtering and compact proof surface behavior.
- Forge implementation paths: `crates/webui/src/chat_ui_action_review.html`, `crates/webui/src/chat_ui.rs`, `PROJECT_STATE.md`, and `docs/generated/proof/action-review-checklist-20260701T1258Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
