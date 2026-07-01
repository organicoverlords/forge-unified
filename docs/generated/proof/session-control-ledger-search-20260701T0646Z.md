# Session Control Ledger Search Proof

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

- PR #3 remains the active open non-draft PR for the target branch.
- Latest inspected head before this slice: `0fe448d6333bac7629855c8b4b234df80fe0f02d`.
- Workflows on that head showed Build Proof, App Build Proof, and App Multistep Build Proof success, while CI, Fast WebUI Proof, and Live WebUI Feature Sprint were red.
- The CI Smoke Test failure was deterministic state/proof-token drift in `PROJECT_STATE.md`, not Rust build/test failure.

## Feature slice

Implemented searchable WebUI session-control event ledger:

- New browser UI partial: `crates/webui/src/chat_ui_session_control_search.html`.
- Bundled into `CHAT_HTML` from `crates/webui/src/chat_ui.rs`.
- Adds a search box labelled `search session events`.
- Filters rendered `.backend-session-control-event-row` nodes by visible summary text and `data-session-control-event` JSON.
- Hides nonmatching rows using `data-session-search-hidden="true"`.
- Shows a visible `search showing N/M` count.

## OpenCode source backing

Exact upstream reference paths inspected:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `showAll`, `overflow`, `visible`, and `session-turn-diffs-more` pattern.
  - Session turn error-card/copyable surface pattern.

## Deterministic contract updates

Updated `scripts/smoke/check-session-controls-contract.py` to require:

- `session-control-ledger-search`
- `session-control-ledger-search-input`
- `session-control-ledger-search-count`
- `session-control-search-query`
- `data-session-search-hidden`
- `search session events`
- `MutationObserver`
- `include_str!("chat_ui_session_control_search.html")`

## Claim boundary

This proof records a source-backed implementation slice and deterministic contract update. It does not claim same-head WebUI browser screenshot proof. Same-head acceptance still requires GitHub Actions artifacts/screenshots on the exact final head.
