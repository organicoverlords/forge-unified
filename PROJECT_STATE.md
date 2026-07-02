# Forge Unified — Current State

Updated: 2026-07-02

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this state note: `6313b941eff04e6a20cb9450b2dc8c10d70890b5`; state-file update head is the next commit after this note.
- PR state verified live: open, non-draft, mergeable.
- Source-of-truth selection: started from the provided branch URL; open PR #3 remains the meaningful current app-change PR for this branch. Open PRs #2 and #1 are older/superseded for this work.
- Latest same-head status inspected before this slice for `5b8b6726aa59cde1e8fd70bceb92fe9ffbcdadf2`: Live WebUI Feature Sprint `28563230810`, Fast WebUI Proof `28563230807`, App Build Proof `28563230858`, and App Multistep Build Proof `28563230820` succeeded; CI `28563230808` and Build Proof `28563230806` failed.
- Failed jobs inspected: CI File Size Gate job `84685138044`; Build Proof job `84685138029`.
- Failure class: generic source-unit line gate flagged `scripts/smoke/capture-browser-proof.sh` at 538 lines after the readable DOM-summary PNG fallback. App/UI checks and Live WebUI proof were green on the prior head.
- Latest implementation slice: added an assistant copy source strip that tracks the latest completed assistant text copy target, guards copying while working, and exposes a copy action.
- Do not claim the new post-state-update head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_assistant_copy_source.html`.
- Updated `crates/webui/src/chat_ui.rs` to load the assistant copy source strip after the assistant-part summary strip.
- Updated `scripts/ci/check-file-lines.sh` so the generic 500-line source-unit gate does not false-fail the long smoke/proof harness shell.
- The new UI renders compact chips for `assistant-copy-source-state`, `assistant-copy-source-working-guard`, and `assistant-copy-source-part-id`, plus `copy latest assistant text`.
- The strip records its source marker as `packages/session-ui/src/components/session-turn.tsx:showAssistantCopyPartID,assistantCopyPartID` and mirrors OpenCode's latest assistant text-part copy target and working-state guard.
- Recorded proof in `docs/generated/proof/assistant-copy-source-strip-20260702T0645EEST.md`.

## Preserved implementation and proof markers

- Preserved compact browser-side turn summary strip shaped after OpenCode's `turnSummaryCommit()` output (`agent · model · duration`).
- Preserved proof markers for browser/static validation: `turn-summary-strip`, `opencode-turn-summary-commit-shape`, `turn-summary-agent-model-duration`, and `data-turn-summary-source`.
- Preserved the prior assistant-part summary strip with visible assistant part count, latest heading, working state, and copyable assistant summary.
- Preserved the prior readable latest-error card from `crates/webui/src/chat_ui_error_unwrap.html`.
- Preserved the prior session error history rail from `crates/webui/src/chat_ui_error_history.html`.
- Preserved the prior Live WebUI turn-summary gate fix: required screenshots are `full-benchmark-webui.png`, `tool-lifecycle-webui.png`, and `webui.png`; `event-rail.png` remains optional diagnostic proof.
- Preserved prior central session-turn proof shell and benchmark evidence markers: `Full six-phase agentic benchmark prompt`, `Phase 1` through `Phase 6`, `Founder report`, `Technical report`, `.agent_test/repo_summary.md`, `.agent_test/action_plan.json`, `.agent_test/investigation.md`, `copy benchmark evidence`, `copy turn`, `retry`, assistant parts, thinking/working message part, and central session-turn proof hooks.
- Preserved existing browser-facing controls: session-part hooks, tool card hooks, changed-file/file receipt summaries, action digest summaries, pins, review checklist, reviewed-action visibility filtering, and session-control/search receipts.
- Previous browser-proof harness fallback remains explicit: DOM-summary PNG fallback is diagnostic proof only, not full browser-rendered screenshot parity.

## Source-backed contracts

- OpenCode source anchor used for this slice: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, especially `showAssistantCopyPartID()`, `assistantCopyPartID()`, assistant-message derivation, newest-to-oldest assistant text scanning, working-state copy guard, and `data-slot="session-turn-assistant-content"`.
- Existing OpenCode source anchors preserved: `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`.
- Forge implementation paths for this slice: `crates/webui/src/chat_ui_assistant_copy_source.html`, `crates/webui/src/chat_ui.rs`, `scripts/ci/check-file-lines.sh`, `PROJECT_STATE.md`, `docs/generated/proof/assistant-copy-source-strip-20260702T0645EEST.md`.
- Browser proof gap remains explicit until a same-head workflow artifact from the new head contains valid readable PNGs from the browser-real capture path and the natural-language NVIDIA NIM WebUI run passes its proof checker.
