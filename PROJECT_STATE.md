# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `b8438f015c1811e6ea91e3a574d555db5113602f`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `b8438f015c1811e6ea91e3a574d555db5113602f` had CI, Build Proof, App Build Proof, and App Multistep Build Proof success; Fast WebUI Proof and Live WebUI Feature Sprint were red at browser proof capture after the NVIDIA NIM WebUI stream.
- Latest implementation slice: WebUI action digest review now has an OpenCode-backed unreviewed-action overflow rail: first four unreviewed visible actions, `+N more unreviewed`, click-to-focus source card, and copy unreviewed actions.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_action_review.html` with browser-side review backlog behavior for every human-readable action summary card.
- The review overflow rail is interactive product behavior, not docs-only and not cosmetic-only: the user can see remaining unreviewed visible actions, jump to the first unreviewed action cards, copy the unreviewed backlog, then mark visible actions reviewed.
- Proof markers added for static/browser checks: action-review-overflow-rail; action-review-overflow-count; focus-unreviewed-action; copy-unreviewed-actions; opencode-showall-overflow-parity.
- Preserved browser proof gap explicitly: accepted screenshot proof still requires a readable PNG produced by the browser-real visual capture path; DOM-only, JSON-only, or fabricated image fallback is not counted.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; session-control ledger search; search session events; data-session-search-hidden; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest; action-digest-pin-visible; pinned-action-summary; copy-pinned-actions; action-pin-count; pinned-action-rail; pinned-action-rail-list; pinned-action-rail-overflow; action-review-checklist; action-reviewed-count; mark-visible-reviewed; copy-reviewed-actions; reviewed-action-summary; reviewed-action-checkbox; action-review-overflow-rail; action-review-overflow-count; focus-unreviewed-action; copy-unreviewed-actions; opencode-showall-overflow-parity.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `showAll`, `overflow`, `visible`, `session-turn-diffs-more`, visible session part filtering, and error-card proof surface behavior; `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, especially `writeClipboard`/message action button behavior; `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, especially visible counted summary behavior.
- Forge implementation paths: `crates/webui/src/chat_ui_action_review.html`, `PROJECT_STATE.md`, and `docs/generated/proof/action-review-overflow-rail-20260701T1347Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
