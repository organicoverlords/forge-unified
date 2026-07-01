# Natural WebUI direct browser proof fallback — 2026-07-01T21:48Z

## Selection basis

- Source-of-truth branch: `mvp/nim-freellmapi-router-20260626` from `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- PR: #3 into `master`, open/non-draft/mergeable at live inspection.
- Previous same-head checked: `d1b37820c1d7ab904452ed1610d2d6eb7f7ee4b1`.
- Same-head workflow state for that commit:
  - CI `28546832506`: success.
  - Build Proof `28546832535`: success.
  - Fast WebUI Proof `28546832497`: success.
  - App Build Proof `28546832555`: success.
  - App Multistep Build Proof `28546832510`: success.
  - Live WebUI Feature Sprint `28546832493`: failure.

## Failure inspected

- Live WebUI Feature Sprint job: `84634155142`.
- Artifact: `8022412516` / `live-webui-feature-sprint-proof`.
- The full benchmark pass inside the job used provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, passed with `failed_checks: []`, and recorded 30 tool-call events.
- The natural feature-build prompt path used provider `nvidia_nim`, normal WebUI path `true`, stream exit code `0`, 19 tool-call events, and 19 tool-result events, but failed `browser_proof_success` and `screenshot_png_present` because `forge-proof/live-webui-feature-sprint/natural-feature-work/webui.png` was not produced.

## Source-backed OpenCode parity slice

OpenCode source anchors used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-component="session-turn"` and `data-slot="session-turn-content"` central turn shell.
  - `session-turn-message-container`, `session-turn-message-content`, `session-turn-assistant-content`, and `session-turn-thinking` live/assistant continuity.
  - `session-turn-diffs` and `session-turn-diffs-more` bounded overflow/show-more behavior.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - Message/action copy affordances and user-visible action surfaces.

Forge paths changed:

- `scripts/smoke/natural-feature-work.sh`

Behavior added:

- The natural-language feature-build proof now keeps the normal WebUI streaming path unchanged, then tries the backend `/api/browser-proof` endpoint first.
- If the endpoint does not produce a readable PNG, the script falls back to the same direct browser proof harness used by the full WebUI proof path: `scripts/smoke/capture-browser-proof.sh`.
- The fallback targets the same WebUI conversation URL with `&proof=natural-feature`, writes `browser-proof.json` and `webui.png` into the natural proof directory, and records screenshot capture metadata in `summary.json` / `summary.md`.
- This is not docs-only or cosmetic-only: it changes the runtime proof gate for natural-language WebUI feature prompts and directly targets the failing Live WebUI job.

## Proof command expectation

The branch update should trigger PR workflows for the new head. Passing acceptance requires same-head inspection of:

- CI.
- Build Proof.
- Fast WebUI Proof.
- Live WebUI Feature Sprint.
- App Build Proof.
- App Multistep Build Proof.

Do not claim browser-rendered visual parity if the metadata reports `dom_summary_png_fallback: true`; that is diagnostic proof only. Full browser-rendered proof requires a valid readable PNG from the browser/PDF capture path and passing Live WebUI proof manifest.
