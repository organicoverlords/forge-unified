# WebUI action digest summary proof

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection basis

The provided target URL points to `organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
Live PR search found PR #3 as the newest relevant open PR in this repository. PR #3 is open, non-draft, mergeable, and targets `master`.

## Pre-slice live state

Latest inspected head before this slice: `1bbf430c304dfbd77d1eed2647345a7aad67158e`.

Same-head workflow state for that head:

- Build Proof `28484490778`: success.
- CI `28484490787`: failure in Smoke Test only.
- Fast WebUI Proof `28484490807`: failure.
- Live WebUI Feature Sprint `28484490817`: failure.
- App Build Proof `28484490809`: failure.
- App Multistep Build Proof `28484490785`: failure.

Inspected CI failed job:

- CI run: `28484490787`.
- Smoke Test job: `84427863548`.
- Failure was deterministic state/proof wording guard: `PROJECT_STATE.md` matched formatter activation overclaim patterns before runtime formatter config/dependency probes exist.

## OpenCode source backing

Used upstream source paths as behavior references only; no OpenCode runtime branding is exposed in Forge UI.

- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - Source-backed behavior: count summaries filter/hide zero-count items and expose active count state.
- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Source-backed behavior: session-turn scoped assistant/tool presentation, turn duration/error/card surface, copy affordances, changed-file/diff rollups, and overflow handling.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
- `anomalyco/opencode:packages/session-ui/src/components/basic-tool.tsx`
- `anomalyco/opencode:packages/web/src/components/share/part.tsx`

## Feature built

Built a real WebUI product slice: action digest summaries for long tool runs.

Implementation path:

- `crates/webui/src/chat_ui_action_summaries.html`

User-facing behavior:

- Keeps existing per-tool `human-action-summary` cards.
- Adds an `action-digest-summary` near the visible tool list/session scope.
- Shows non-zero completed, needs-attention, running, and pending counts.
- Lists the first visible tool actions with human label, target, and status.
- Adds `copy action digest` so the user can copy a compact proof receipt.

DOM/proof hooks added or retained:

- `human-action-summary`
- `action-digest-summary`
- `action-count-summary-visible`
- `action-digest-ok-count`
- `action-digest-error-count`
- `action-digest-running-count`
- `action-digest-pending-count`
- `copy-action-digest`

## Guard updates

Updated deterministic WebUI contract guard:

- `scripts/smoke/check-webui-proof-part-contract.py`

The guard now includes `crates/webui/src/chat_ui_action_summaries.html` in inspected UI paths and checks for the action digest tokens/copy affordance.

## State/proof repair

Updated:

- `PROJECT_STATE.md`

Repair:

- Removed wording that looked like runtime formatter activation parity.
- Kept the formatter source-evidence trail and explicit runtime gap.

## Claim boundary

This slice changes the real browser WebUI surface and deterministic guard coverage.

Not claimed yet:

- Same-head green status.
- Full OpenCode parity.
- Production readiness.
- Latest-head WebUI screenshot proof.

Acceptance still requires CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof to complete successfully on the final exact head, with artifacts/screenshots inspected.
