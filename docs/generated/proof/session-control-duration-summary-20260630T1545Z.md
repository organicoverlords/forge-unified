# Proof — WebUI session-control duration summary

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`
Baseline before slice: `4d795f1e2706f95732a666e23ade2682b63dcdbe`

## Selection basis

- Target URL branch remained `mvp/nim-freellmapi-router-20260626`.
- PR #3 remained open, non-draft, and mergeable.
- No newer open PR or pushed non-main branch with meaningful app changes superseded the target branch.

## Baseline proof inspected

Same-head workflows for `4d795f1e2706f95732a666e23ade2682b63dcdbe` were green before this slice:

- CI: `28453718241`
- Build Proof: `28453718000`
- Fast WebUI Proof: `28453717708`
- Live WebUI Feature Sprint: `28453718572`
- App Build Proof: `28453718515`
- App Multistep Build Proof: `28453717692`

Live WebUI artifact inspected by metadata:

- Artifact id: `7984676474`
- Artifact name: `live-webui-feature-sprint-proof`
- Head SHA: `4d795f1e2706f95732a666e23ade2682b63dcdbe`

## OpenCode source backing

Reference paths inspected from upstream OpenCode:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `SessionTurn` computes turn duration from the user message creation time and assistant completion times before passing `turnDurationMs` into `AssistantParts`.
  - Relevant behavior copied as a Forge-local browser-control concept: surface timing for a user-visible session operation instead of hiding operation timing in logs.
- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Session diffs and status areas remain browser-visible in compact rows; Forge mirrors that pattern for session-control event ledger rows.

## Feature built

Added Forge-local WebUI session-control duration receipts.

The browser control bundle now records UI-side timing for checkpoint/fork/revert/retry/copy control operations:

- `started_at`
- `completed_at`
- `duration_ms`

The visible ledger renders this as compact chips:

- `duration: <n>ms`
- `started: <timestamp>`
- `completed: <timestamp>`

The raw receipt stores this under `ui_timing`, keeping upstream OpenCode source paths out of runtime browser payloads.

## Files changed

- `crates/webui/src/chat_ui_session_controls.html`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-duration-summary-20260630T1545Z.md`

## Deterministic proof gate

Updated `scripts/smoke/check-session-controls-contract.py` to require:

- `session-control-duration-summary`
- `backend-session-control-duration-summary`
- `backend-session-control-duration-chip`
- `session-control-duration-ms`
- `session-control-started-at`
- `session-control-completed-at`
- `ui_timing`
- `duration_ms`
- `started_at`
- `completed_at`

Runtime source-path guard remains active: upstream OpenCode paths are allowed in docs/proof/state only, not in Forge browser/runtime session-control payloads.

## Claim boundary

This is a meaningful OpenCode-backed WebUI behavior slice.

Do not claim latest-head WebUI browser proof or parity for the new head until same-head CI / Build Proof / Fast WebUI / Live WebUI / App Build / App Multistep workflows complete and artifacts/screenshots are inspected.
