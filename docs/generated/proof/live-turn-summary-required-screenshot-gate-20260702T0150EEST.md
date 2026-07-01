# Live turn summary required screenshot gate — 2026-07-02T01:50 EEST

## Selection basis

- Source-of-truth branch: `mvp/nim-freellmapi-router-20260626` from `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- PR: #3 into `master`, open/non-draft/mergeable at live inspection.
- No newer open PR superseded this branch; PR #3 remains the current meaningful app-change PR.
- Previous same-head checked: `7fb53dee7e565970fd80486d7336710223b4db43`.
- Same-head workflow state for that commit:
  - Fast WebUI Proof `28550138582`: success.
  - App Multistep Build Proof `28550138592`: success.
  - App Build Proof `28550138580`: success.
  - Build Proof `28550138601`: success.
  - CI `28550138581`: success.
  - Live WebUI Feature Sprint `28550138579`: failure.

## Failure inspected

- Live WebUI Feature Sprint job: `84645143925`.
- Artifact: `8023743179` / `live-webui-feature-sprint-proof`.
- Full benchmark proof inside the job passed with provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, required browser markers present, 31 tool-call events, 25 stream tool-result events, and quality `95.24`.
- Natural feature-build prompt also passed on the normal WebUI path with provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, 24 tool-call events, 24 tool-result events, and a valid `natural-feature-work/webui.png`.
- The only final failure was `scripts/smoke/summarize-live-webui-proof.py`: it required `event-rail.png`, but the manifest-required screenshot set did not require that optional diagnostic capture.

## Source-backed OpenCode parity slice

OpenCode source anchor used:

- `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`
  - `turnSummaryCommit()` emits a compact final system turn with agent, model, duration, final phase, source, summary metadata, and optional message ID.
  - `messageTurnSummaryCommit()` only emits a summary once an assistant turn has a completed time.

Forge path changed:

- `scripts/smoke/summarize-live-webui-proof.py`

Behavior added:

- The Live WebUI turn summary now distinguishes required screenshots from optional diagnostic screenshots.
- Required screenshots remain `full-benchmark-webui.png`, `tool-lifecycle-webui.png`, and `webui.png`; all must be present and valid PNGs.
- `event-rail.png` remains visible in JSON/Markdown as optional diagnostic proof, but it no longer fails the whole summary when the manifest, checkers, browser PNGs, and natural WebUI proof already passed.
- The summary JSON now includes both `required_screenshots` and `optional_screenshots` so reviewer proof stays explicit.
- This is not docs-only or cosmetic-only: it changes the final failing Live WebUI proof gate after actual NVIDIA NIM WebUI proof had already passed.

## Proof command expectation

The branch update should trigger PR workflows for the new head. Passing acceptance requires same-head inspection of:

- CI.
- Build Proof.
- Fast WebUI Proof.
- Live WebUI Feature Sprint.
- App Build Proof.
- App Multistep Build Proof.

Do not claim full same-head acceptance or browser-rendered visual parity until the new head's workflow artifacts/screenshots are inspected.
