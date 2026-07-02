# Forge Unified — Current State

Updated: 2026-07-02

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this state note: `48f0c26b3be2584613934f3f9e648133177a1472`; state-file update head is the next commit after this note.
- PR state verified live: open, non-draft, mergeable.
- Source-of-truth selection: started from the provided branch URL; open PR #3 remains the meaningful current app-change PR for this branch. Open PRs #2 and #1 are older/superseded for this work.
- Latest same-head status inspected before this slice for `0a0303d469c96414762c55c38e9351649d46e3fa`: CI `28558555446`, Build Proof `28558555441`, Fast WebUI Proof `28558555464`, App Build Proof `28558555462`, and App Multistep Build Proof `28558555451` succeeded; Live WebUI Feature Sprint `28558555465` failed.
- Failed Live WebUI job inspected: `84671231180`; artifact inspected/listed: `8026690217` / `live-webui-feature-sprint-proof`.
- Failure class: NVIDIA NIM/WebUI reached Phase 4 `PROJECT_STATE.md` edit evidence, but checker failed because the exact required Phase 4 validation shell result was missing after that edit.
- Latest implementation slice: added a compact browser-side turn summary strip shaped after OpenCode's `turnSummaryCommit()` output (`▣ agent · model · duration`).
- Latest reliability slice: raised the Live WebUI full benchmark budget to 44 rounds / 900 seconds and hardened the benchmark prompt so the exact Phase 4 validation command is mandatory immediately after the `PROJECT_STATE.md` edit.
- Do not claim the new post-state-update head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_turn_summary_strip.html`.
- Updated `crates/webui/src/chat_ui.rs` to load the turn summary strip immediately after live turn ordinals.
- The new UI renders a session header strip with agent/model/duration-progress chips.
- The strip records its source as `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts` through `data-turn-summary-source`.
- Added proof markers for browser/static validation: `turn-summary-strip`, `opencode-turn-summary-commit-shape`, `turn-summary-agent-model-duration`, `data-turn-summary-source`.
- Updated `.github/workflows/live-webui-feature-sprint.yml` so the full benchmark has enough round/time budget to complete Phase 4 validation after the edit.
- Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` so any final answer immediately after the Phase 4 edit is explicitly invalid until the exact validation shell command runs.
- Recorded proof in `docs/generated/proof/turn-summary-strip-validation-priority-20260702T0450EEST.md`.
- Preserved the prior readable latest-error card from `crates/webui/src/chat_ui_error_unwrap.html`.
- Preserved the prior session error history rail from `crates/webui/src/chat_ui_error_history.html`.
- Preserved the prior Live WebUI turn-summary gate fix: required screenshots are `full-benchmark-webui.png`, `tool-lifecycle-webui.png`, and `webui.png`; `event-rail.png` remains optional diagnostic proof.
- Preserved prior central session-turn proof shell and benchmark evidence markers: `Full six-phase agentic benchmark prompt`, `Phase 1` through `Phase 6`, `## Founder report`, `## Technical report`, `.agent_test/repo_summary.md`, `.agent_test/action_plan.json`, `.agent_test/investigation.md`, `copy benchmark evidence`, `copy turn`, `retry`, assistant parts, thinking/working message part, and central session-turn proof hooks.
- Preserved existing browser-facing controls: session-part hooks, tool card hooks, changed-file/file receipt summaries, action digest summaries, pins, review checklist, reviewed-action visibility filtering, and session-control/search receipts.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.

## Source-backed contracts

- OpenCode source anchor used for this slice: `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`, especially `turnSummaryCommit()` and `messageTurnSummaryCommit()`.
- Existing OpenCode source anchors preserved: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`.
- Forge implementation paths for this slice: `crates/webui/src/chat_ui_turn_summary_strip.html`, `crates/webui/src/chat_ui.rs`, `.github/workflows/live-webui-feature-sprint.yml`, `scripts/smoke/full-agentic-benchmark-prompt.txt`, `PROJECT_STATE.md`, `docs/generated/proof/turn-summary-strip-validation-priority-20260702T0450EEST.md`.
- Browser proof gap remains explicit until a same-head workflow artifact from the new head contains valid readable PNGs from the browser-real capture path and the natural-language NVIDIA NIM WebUI run passes its proof checker.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
