# Session-control ledger export proof

Timestamp: 2026-06-30T18:48Z
Branch: `mvp/nim-freellmapi-router-20260626`
Repo: `organicoverlords/forge-unified`

## Selection basis

- Target URL branch remains the selected work branch.
- PR #3 remains the active OpenCode-parity PR into `master`.
- No newer open PR or recently pushed non-main branch was found to supersede this branch for app/WebUI parity work during this run.

## Inspected live workflow state before this slice

Selected head before patch: `3f30efa81a1e96d28b52eb6f33d4ec0201718be8`.

- Build Proof `28467847332`: success.
- Fast WebUI Proof `28467847413`: success.
- App Build Proof `28467847407`: success.
- App Multistep Build Proof `28467847383`: success.
- CI `28467847385`: in progress overall, but Smoke Test job `84372208195` had already failed.
- Live WebUI Feature Sprint `28467847318`: still in progress at inspection time.

CI failure inspected:

- Job: `84372208195` / Smoke Test.
- Failed step: `Validate WebUI proof harness`.
- Failed deterministic check: `state:checkpoint, fork, revert latest turn, and retry source`.
- Interpretation: proof-state wording drift, not a backend route or UI token failure. The same job showed the route/UI checks around session-control event ledger, overflow, duration, diff, and hidden older row passing before the state phrase failure.

## OpenCode source backing

Upstream source read during this slice:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `showAssistantCopyPartID` chooses a copyable assistant text part after the turn is no longer working.
  - `AssistantParts` receives `showAssistantCopyPartID` and `turnDurationMs` for visible turn-level evidence.
  - `MAX_FILES`, `showAll`, `overflow`, `visible`, `toggleAll`, and `session-turn-diffs-more` implement compact/expanded evidence with a visible overflow affordance.

Forge adaptation:

- The WebUI session-control ledger already had per-event copy and compact overflow.
- This slice adds a turn-level/session-level `copy all events` affordance that exports the current filtered ledger as structured JSON, preserving counts and visible/filtered metadata.
- OpenCode source paths remain in docs/proof only, not runtime browser payloads.

## Files changed

- `crates/webui/src/chat_ui_session_controls.html`
  - Adds proof tokens `session-control-ledger-export` and `copy-session-control-ledger`.
  - Adds `ledgerExport()` JSON payload with filter, total, visible, filtered, ok/error counts, events, and timestamp.
  - Adds `copy all events` button in session controls and per-turn receipt toolbars.
- `scripts/smoke/check-session-controls-contract.py`
  - Requires `session-control-ledger-export`, `copy-session-control-ledger`, `copy all events`, and `forge.session_control_ledger`.
  - Keeps the forbidden runtime-source guard.
- `PROJECT_STATE.md`
  - Fixes exact state phrase required by CI: `checkpoint, fork, revert latest turn, and retry source`.
  - Records the ledger export behavior and source backing.
- `docs/generated/proof/session-control-ledger-export-20260630T1848Z.md`
  - This proof note.

## Claim boundary

This is a meaningful OpenCode-backed WebUI behavior increment plus deterministic CI-state repair. It is not full OpenCode parity and not production readiness.

Do not claim latest-head WebUI screenshot proof until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the final head and the artifacts/screenshots are inspected.
