# Proof — session-control group status summary

Date: 2026-07-02 10:47 EEST
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Live selection basis

- Source-of-truth URL branch: `mvp/nim-freellmapi-router-20260626`.
- Selected PR: #3, `mvp router slice`, open and non-draft.
- Previous exact head inspected before this slice: `ce8fc3ac7f6789aea6cf236b85d5a5f0f82e196f`.
- Previous same-head workflows were green:
  - CI `28571274876`
  - Build Proof `28571274849`
  - Fast WebUI Proof `28571274809`
  - Live WebUI Feature Sprint `28571274840`
  - App Build Proof `28571274833`
  - App Multistep Build Proof `28571274832`
- Live WebUI proof inspected for the previous exact head:
  - Run `28571274840`
  - Job `84709423129`
  - Artifact `8031309789` (`live-webui-feature-sprint-proof`)

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Source-backed behavior copied as a Forge-shaped implementation, without OpenCode branding:

- `DiffChanges` status/count summary behavior in the session-turn diffs header.
- `data-slot="session-turn-diffs-header"`
- `data-slot="session-turn-diffs-label"`
- `data-slot="session-turn-diffs-toggle"`
- `data-component="session-turn-diffs-content"`
- `showAll()` / `toggleAll()`
- `overflow()` / `visible()`

## Forge implementation

Updated:

- `crates/webui/src/chat_ui_session_control_groups.html`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-group-status-summary-20260702T1047EEST.md`

Behavior added:

- The grouped session-control ledger now shows status-count summary chips, equivalent in purpose to a compact changes summary in OpenCode's turn diff header.
- The summary is computed from real session-control receipt groups, grouped by status and action.
- Added `copy group summary`, which copies a compact multiline receipt summary for auditing and support handoff.
- Preserved show-all/show-fewer overflow behavior and per-group expandable raw receipt details.

Proof markers added/preserved:

- `session-control-group-status-summary`
- `session-control-group-status-chip`
- `session-control-group-copy-summary`
- `opencode-session-turn-diff-changes-shape`
- `session-control-group-show-all`
- `session-control-group-show-less`
- `opencode-session-turn-show-all-toggle`

## Claim boundary

This slice is source-backed and committed, but the post-slice head is not yet same-head browser/NIM proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact new head and the Live WebUI artifact/screenshots are inspected.
