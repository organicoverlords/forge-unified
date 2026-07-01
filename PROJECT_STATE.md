# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `dc5161acaa22bac5854ddf8c68739b078dfbb4d1`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `dc5161acaa22bac5854ddf8c68739b078dfbb4d1` had CI, Build Proof, App Build Proof, and App Multistep Build Proof success; Fast WebUI Proof and Live WebUI Feature Sprint were red.
- Latest implementation slice: browser proof harness now disables direct Chrome `--screenshot` fallback by default on GitHub Actions after PDF-first capture fails, avoiding the repeatedly observed `--screenshot` segmentation-fault path.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `scripts/smoke/capture-browser-proof.sh` so `FORGE_BROWSER_PROOF_STRATEGY` controls capture order; on GitHub Actions the default strategy remains `pdf-first`, while local/default non-CI behavior remains `screenshot-first`.
- Added `allow_screenshot_after_pdf_fail()`: GitHub Actions defaults to `0`, local runs default to `1`, and `FORGE_BROWSER_PROOF_ALLOW_SCREENSHOT_AFTER_PDF_FAIL` remains an explicit override.
- The CI `pdf-first` path renders the same WebUI URL with Chrome `--print-to-pdf`, converts the rendered page to the required PNG, and no longer falls through to the known-bad direct screenshot path unless explicitly requested.
- Proof JSON and Chrome diagnostics now record `ci_screenshot_fallback_disabled_after_pdf_fail` and `pdf-only-failed`, so failures are fast and diagnosable instead of timing out in a repeated screenshot segfault.
- Success still requires a readable PNG and the existing marker checks for provider/model/tool UI content; DOM-only or JSON-only fallback is not counted as screenshot proof.
- Preserved browser-real capture semantics: the accepted proof PNG must come from Chrome rendering the same WebUI URL, not from fabricating a proof image from text.
- Preserved prior contract phrases so CI state validation continues to cover: backend-backed session controls; checkpoint, fork, revert latest turn, and retry source; Forge-local session control receipts; session control event ledger; copy session control event; session-control event disclosure; session-control count summary; status filters; session-control diff summary; before/after/removed message chips; session-control duration summary; start/completion/duration chips; session-control ledger overflow; show older/show less overflow toggle; visible ledger count; hidden older receipt row; session-control ledger export; copy all events; session-control error card; copy latest error; latest session-control error; stable session-control receipt identity; receipt_id; sequence; forge.webui.session_controls; `crates/webui/src/conversation_controls.rs`; `crates/webui/src/chat_ui_session_controls.html`; `crates/webui/src/chat_ui_session_control_search.html`; session-control-ledger-search; session-control ledger search; search session events; data-session-search-hidden; human-action-summary; human action summaries; action-digest-summary; action-digest-filter; copy action digest; action-digest-pin-visible; pinned-action-summary; copy-pinned-actions; action-pin-count; pinned-action-rail; pinned-action-rail-list; pinned-action-rail-overflow.

## Source-backed contracts

- OpenCode source anchors used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `partState`, `assistantVisible`, `showThinking`, `MAX_FILES`, `showAll`, `overflow`, `visible`, `session-turn-diffs-more`, and error-card rendering. These anchor the requirement that proof-relevant session surfaces stay compact, visible, delayed-render-safe, and diagnosable when render/capture state is delayed or failing.
- Forge implementation paths: `scripts/smoke/capture-browser-proof.sh`, `PROJECT_STATE.md`, and `docs/generated/proof/browser-proof-ci-screenshot-fallback-disabled-20260701T1047Z.md`.
- Browser proof gap remains explicit until a same-head workflow artifact contains a valid readable PNG from the browser-real capture path.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
