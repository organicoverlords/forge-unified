# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `9ae7a8f70abaaba13195ee0fca7a5a67b2ce20f1`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `9ae7a8f70abaaba13195ee0fca7a5a67b2ce20f1` had Build Proof, App Build Proof, and App Multistep Build Proof success; CI, Fast WebUI Proof, and Live WebUI Feature Sprint were red.
- Latest implementation slice: WebUI action digest pinned-action rail. Pinned action summaries now render into a compact pinned rail above the full action list, expose a clickable jump back to the source action card, retain `copy pinned actions`, and show pinned overflow after the first four pinned items.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_action_pins.html` so pinned human action summaries are no longer only a card-local flag; they are summarized in a visible pinned action rail with `pinned-action-rail`, `pinned-action-rail-list`, and `pinned-action-rail-overflow` proof tokens.
- The pinned rail keeps the OpenCode-style compact turn surface: only the first four pinned cards are surfaced in the rail while remaining pinned items stay available in the original action list, matching the visible/overflow pattern used for session diffs.
- Fixed the state-trail phrase that CI expected for the previous slice: `session-control ledger search` is now present alongside `session-control-ledger-search`, `search session events`, and `data-session-search-hidden`.
- Preserved browser-real capture semantics: success still requires a readable PNG; DOM-only or JSON-only fallback is not counted as screenshot proof.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; session-control ledger search; search session events; data-session-search-hidden; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest; action-digest-pin-visible; pinned-action-summary; copy-pinned-actions; action-pin-count; pinned-action-rail; pinned-action-rail-list; pinned-action-rail-overflow.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, especially `writeClipboard` and `MessageActionButton` copy/action-button behavior; `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `MAX_FILES`, `showAll`, `overflow`, and `visible`; `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`, especially compact visible count summary behavior.
- Forge implementation paths: `crates/webui/src/chat_ui_action_pins.html`, `PROJECT_STATE.md`, and `docs/generated/proof/action-digest-pinned-rail-20260701T1046Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
