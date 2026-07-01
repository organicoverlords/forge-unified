# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `eb7131fa07c078f083b366d0b592f3ac910a5e25`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `eb7131fa07c078f083b366d0b592f3ac910a5e25` had Build Proof, App Build Proof, and App Multistep Build Proof success; CI, Fast WebUI Proof, and Live WebUI Feature Sprint were red.
- Latest implementation slice: browser proof screenshot path/read guards. The direct Chrome capture harness now creates screenshot/JSON/DOM parent directories before each capture attempt and uses a `png_size` helper that checks file existence before `wc -c`, avoiding shell redirection failures when Chrome segfaults or produces no PNG.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `scripts/smoke/capture-browser-proof.sh` with explicit path-parent creation for browser screenshot, DOM, JSON, and Chrome profile paths.
- Added `png_size` so the proof harness records `png_bytes=0` cleanly when Chrome produces no file instead of emitting `No such file or directory` from shell redirection.
- Added browser JSON metadata tokens `screenshot_path_parent_guard` and `png_size_redirection_guard` so artifacts identify this exact harness behavior.
- Preserved browser-real capture semantics: success still requires a readable PNG; DOM-only or JSON-only fallback is not counted as screenshot proof.
- Preserved prior contract phrases so CI state validation continues to cover: Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; search session events; data-session-search-hidden; human-action-summary; action-digest-summary; action-digest-filter; copy action digest.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially the session-turn rendered proof surface, `turnDurationMs`, error-card rendering, compact copyable assistant turn behavior, and session diff overflow (`showAll`, `overflow`, `visible`, `session-turn-diffs-more`).
- Forge implementation paths: `scripts/smoke/capture-browser-proof.sh`, `PROJECT_STATE.md`, and `docs/generated/proof/browser-proof-path-guards-20260701T0754Z.md`.
- Browser proof guard tokens: `screenshot_path_parent_guard`, `png_size_redirection_guard`, `path_parent_guard=true`, and `png_size_redirection_guard=true`.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
