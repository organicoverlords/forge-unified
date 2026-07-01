# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `e53a53b3681a3248de1a4a9edd87621e6df9668d`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `e53a53b3681a3248de1a4a9edd87621e6df9668d` had Build Proof, App Build Proof, and App Multistep Build Proof success; CI, Fast WebUI Proof, and Live WebUI Feature Sprint were red.
- Latest implementation slice: WebUI action digest visible-action controls. The action digest now exposes `copy visible actions` plus `focus first visible action`, keeps focused action summaries visible with `data-action-focused`, and preserves filtered visible counts.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_action_summaries.html` with a focusable action summary card state, `copy visible actions`, and `focus first visible action` controls for the filtered action digest.
- Added source-visible proof tokens `action-digest-focus-visible` and `copy-visible-actions` while preserving existing `human-action-summary`, human action summaries, `action-digest-summary`, `action-count-summary-visible`, status filters, `aria-pressed`, visible counts, and `copy action digest` evidence.
- Preserved browser-real capture semantics: success still requires a readable PNG; DOM-only or JSON-only fallback is not counted as screenshot proof.
- Preserved prior contract phrases so CI state validation continues to cover: Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; search session events; data-session-search-hidden; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, especially visible count item filtering and fallback count-summary behavior; `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially rendered turn state, visible part filtering, copyable assistant turn behavior, and focused session proof surface.
- Forge implementation paths: `crates/webui/src/chat_ui_action_summaries.html`, `PROJECT_STATE.md`, and `docs/generated/proof/action-digest-visible-controls-20260701T0906Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
