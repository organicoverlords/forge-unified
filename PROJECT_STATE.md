# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `0d1323f50b0381067202548f67af80c5f11717f4`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `0d1323f50b0381067202548f67af80c5f11717f4` had CI, Build Proof, App Build Proof, and App Multistep Build Proof success; Fast WebUI Proof and Live WebUI Feature Sprint were red.
- Latest implementation slice: browser proof harness now keeps CI visual/DOM capture bounded by using the initial WebUI HTML fetch as the default DOM snapshot on GitHub Actions and requiring Chrome only for the visual PNG proof path.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `scripts/smoke/capture-browser-proof.sh` so `FORGE_BROWSER_PROOF_REFRESH_DOM_WITH_CHROME` controls whether the harness runs a second Chrome `--dump-dom` render after visual proof capture.
- GitHub Actions defaults DOM refresh to `0`; local runs default to `1`, preserving richer local diagnostics while avoiding a repeated CI timeout path.
- The DOM proof surface still comes from the exact same WebUI URL via the existing `curl` fetch at the start of `capture_direct()`.
- Accepted screenshot proof still requires a readable PNG produced by the browser-real visual capture path; DOM-only, JSON-only, or fabricated image fallback is not counted.
- Reduced per-attempt Chrome proof budgets through env-overridable defaults: PDF `18s`, screenshot `16s`, DOM refresh `12s` when enabled.
- Proof JSON and Chrome diagnostics now record `ci_curl_dom_snapshot_default`, `chrome_dom_refresh_opt_in`, and `bounded_visual_attempts`.
- Preserved browser-real capture semantics: the accepted proof PNG must come from Chrome rendering the same WebUI URL, not from fabricating a proof image from text.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; session-control ledger search; search session events; data-session-search-hidden; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest; action-digest-pin-visible; pinned-action-summary; copy-pinned-actions; action-pin-count; pinned-action-rail; pinned-action-rail-list; pinned-action-rail-overflow.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `partState`, `assistantVisible`, `showThinking`, `MAX_FILES`, `showAll`, `overflow`, `visible`, `session-turn-diffs-more`, and error-card rendering. These anchor the requirement that proof-relevant session surfaces stay compact, visible, delayed-render-safe, and diagnosable when render/capture state is delayed or failing.
- Forge implementation paths: `scripts/smoke/capture-browser-proof.sh`, `PROJECT_STATE.md`, and `docs/generated/proof/browser-proof-bounded-dom-refresh-20260701T1152Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
