# Session-control group path metadata proof — 2026-07-02T11:48 EEST

## Selection basis

- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- Selected PR: #3, `mvp router slice`.
- Selection reason: PR #3 remains the newest meaningful open app-change PR for the branch. Open PRs #2 and #1 are older/superseded for this Forge WebUI/NIM/router work.

## Live state inspected before this slice

- Previous PR head: `967fdaa6406ade8d72d5dcb3db87c75f485dce44`.
- PR state: open, non-draft, mergeable.
- Same-head workflows for `967fdaa6406ade8d72d5dcb3db87c75f485dce44`:
  - CI `28574254035`: success.
  - Build Proof `28574254100`: success.
  - Fast WebUI Proof `28574254023`: success.
  - App Build Proof `28574254060`: success.
  - App Multistep Build Proof `28574254050`: success.
  - Live WebUI Feature Sprint `28574254128`: failure.
- Failed Live WebUI job inspected: `84718935696`.
- Live proof artifact present: `8032549033` (`live-webui-feature-sprint-proof`).
- Failure details from logs: NVIDIA NIM proof used provider `nvidia_nim` and model `deepseek-ai/deepseek-v4-flash`; OpenCode workflow checker passed, but the full benchmark checker failed `phase2_confidence_labels_in_answer` and `final_tests_run_names_exact_phase4_validation`.

## OpenCode source backing used

Upstream source paths and behaviors used as reference only:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-slot="session-turn-diff-path"`
  - `data-slot="session-turn-diff-directory"`
  - `data-slot="session-turn-diff-filename"`
  - `data-slot="session-turn-diff-meta"`
  - `data-slot="session-turn-diff-changes"`
  - grouped diff content shape: `data-component="session-turn-diffs-group"` and `data-component="session-turn-diffs-content"`
  - show-all/overflow pattern: `showAll`, `toggleAll`, `overflow`, `visible`, `session-turn-diffs-more`

Forge remains independent; no OpenCode branding or source-path text is exposed in the WebUI.

## Feature built

Implemented **session-control grouped ledger path metadata** in `crates/webui/src/chat_ui_session_control_groups.html`.

Concrete behavior:

- Group key now includes `status`, `action`, and path/target metadata when available.
- Each grouped ledger row renders a two-line main block:
  - action/status/event count title;
  - OpenCode-shaped path metadata split into directory and filename spans.
- Added latest receipt meta chip beside the group toggle.
- Copy summary now includes each visible group path.
- Existing status summary chips, copy summary, show-all/show-fewer overflow, and expandable raw details are preserved.

## Proof hooks added

- `session-control-group-path-meta`
- `session-control-group-directory`
- `session-control-group-filename`
- `session-control-group-meta`
- `session-control-group-latest`
- `opencode-session-turn-diff-path-shape`
- `opencode-session-turn-diff-meta-shape`

## Files changed

- `crates/webui/src/chat_ui_session_control_groups.html`
- `docs/generated/proof/session-control-group-path-meta-20260702T1148EEST.md`
- `PROJECT_STATE.md`

## Claim boundary

This slice is committed, but the latest post-state-update head is not same-head browser/NIM proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and the Live WebUI screenshots/artifacts are inspected.
