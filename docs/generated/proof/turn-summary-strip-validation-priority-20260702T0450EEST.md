# Turn summary strip + validation priority proof note

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`

## Live state checked

- PR #3 remains the selected app-change PR.
- Live PR metadata showed the branch head at `0a0303d469c96414762c55c38e9351649d46e3fa` before this slice.
- Same-head workflows for `0a0303d469c96414762c55c38e9351649d46e3fa` had five successes and one Live WebUI Feature Sprint failure.
- Failed run: Live WebUI Feature Sprint `28558555465`, job `84671231180`, artifact `8026690217`.
- Failure class: NVIDIA NIM/WebUI reached Phase 4 edit evidence, but the full benchmark checker reported missing required Phase 4 validation after the `PROJECT_STATE.md` edit.

## OpenCode source backing used exactly

- `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`
  - `turnSummaryCommit()` emits the compact `▣ agent · model · duration` final/session summary shape.
  - `messageTurnSummaryCommit()` derives assistant turn summaries only when the assistant turn has completed timing metadata.

## Forge slice built

- Added `crates/webui/src/chat_ui_turn_summary_strip.html`.
- Mounted it from `crates/webui/src/chat_ui.rs` immediately after live turn ordinals.
- The browser UI now exposes a compact OpenCode-shaped strip with:
  - `▣ Forge` agent chip.
  - model chip derived from visible model/provider DOM when available.
  - duration/progress chip derived from running/completed turn/tool state.
  - proof markers: `turn-summary-strip`, `opencode-turn-summary-commit-shape`, `turn-summary-agent-model-duration`, `data-turn-summary-source`.

## Reliability change

- Updated `.github/workflows/live-webui-feature-sprint.yml` to raise the full benchmark budget from 36 rounds / 840s to 44 rounds / 900s.
- Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` to make the exact Phase 4 validation command the next mandatory action after the `PROJECT_STATE.md` edit, before any prose or final report.

## Claim boundary

This slice is source-backed and committed, but it is not same-head browser/NIM proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the final commit and their artifacts are inspected.
