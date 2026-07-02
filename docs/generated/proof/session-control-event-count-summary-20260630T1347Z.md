# Session-control event count summary proof — 2026-06-30T13:47Z

Repo: `organicoverlords/forge-unified`

Branch: `mvp/nim-freellmapi-router-20260626`

Slice: WebUI session-control event count summary and status filters.

## Selection basis

- Target URL branch remains the selected branch.
- PR #3 is open, non-draft, and mergeable.
- Prior same-head proof for `6238426a2b5ee6fc055999f31a193899dfed1ca6` was inspected. All proof workflows passed except CI Smoke Test, which failed on stale `PROJECT_STATE.md` wording for `checkpoint, fork, revert latest turn, and retry source`.

## Failure inspected

- CI run `28445679343`, job `84294314080`, failed in `Validate WebUI proof harness`.
- Failed check: `state:checkpoint, fork, revert latest turn, and retry source`.
- The underlying backend route/UI checks for checkpoint, fork, revert, retry, receipts, event ledger, disclosure, and source-path scrub all passed.

## OpenCode source backing

- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - `AnimatedCountList` filters visible count items to count values greater than zero.
  - Exposes a count-summary component with empty fallback and active count slots.
- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Session turns derive pending/current status from stored messages and expose user actions around turn/session state.
- Retained source anchor: `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - Show/hide detail panes and visible tool/session details remain the browser behavior reference.

## Forge implementation

- `crates/webui/src/chat_ui_session_controls.html`
  - Added `.backend-session-control-summary`.
  - Added `.backend-session-control-count`.
  - Added status filters with `aria-pressed`: `all`, `ok`, `error`.
  - Added `renderSummary()` and ledger filtering so the user can see counts and filter receipt history without opening raw JSON.
- `scripts/smoke/check-session-controls-contract.py`
  - Requires `session-control-count-summary`, `backend-session-control-summary`, `backend-session-control-count`, `session-control-filter`, `session-control-filter-all`, `session-control-filter-ok`, `session-control-filter-error`, and `aria-pressed`.
- `PROJECT_STATE.md`
  - Records the exact failure and the required session-control summary phrase.

## Claim boundary

This is a meaningful OpenCode-backed WebUI behavior increment. It is not complete parity or production readiness.

Latest head must pass CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof on the same commit before same-head browser proof can be claimed.
