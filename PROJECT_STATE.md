# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `0fe448d6333bac7629855c8b4b234df80fe0f02d`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `0fe448d6333bac7629855c8b4b234df80fe0f02d` had Build Proof and App Multistep Build Proof success; App Build Proof success; CI, Fast WebUI Proof, and Live WebUI Feature Sprint were red.
- Latest implementation slice: searchable WebUI session-control event ledger. Session control receipts can now be searched by event JSON or visible summary text, nonmatching event rows are hidden with `data-session-search-hidden`, and the visible search count reports `search showing N/M`.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_session_control_search.html` as an executable browser UI slice bundled into `CHAT_HTML` via `crates/webui/src/chat_ui.rs`.
- Added `session-control ledger search`, `search session events`, `session-control-ledger-search-input`, `session-control-ledger-search-count`, and `data-session-search-hidden` proof tokens.
- Updated `scripts/smoke/check-session-controls-contract.py` so the deterministic smoke gate checks the new search slice plus the earlier backend-backed session controls, checkpoint, fork, revert latest turn, and retry source behavior.
- Preserved prior contract phrases so CI state validation continues to cover: Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially the session-turn diffs `showAll`, `overflow`, `visible`, and `session-turn-diffs-more` pattern, plus the same file's compact error-card/copyable turn surface.
- Forge implementation paths: `crates/webui/src/chat_ui_session_control_search.html`, `crates/webui/src/chat_ui.rs`, `scripts/smoke/check-session-controls-contract.py`, and `PROJECT_STATE.md`.
- Required session search tokens: `session-control-ledger-search`, `session-control-ledger-search-input`, `session-control-ledger-search-count`, `session-control-search-query`, `data-session-search-hidden`, `search session events`, and `MutationObserver`.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
- Browser proof gap remains explicit: repeated red runs show Chrome screenshot capture failures after NIM/WebUI work completed; same-head screenshot proof is still required before acceptance.
