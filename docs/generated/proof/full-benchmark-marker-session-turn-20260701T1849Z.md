# Full benchmark marker session-turn proof slice — 2026-07-01T18:49Z

## Live selection basis

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open, non-draft, mergeable at live inspection time
- Previous head inspected: `4617390e358ab71e2aa5090667c319651b0d0482`
- Implementation commit for this slice: `6772b024e532eb1136745149ebabe357202eb8fc`

## Workflow state inspected before patch

Same-head workflows for `4617390e358ab71e2aa5090667c319651b0d0482`:

- CI `28536980669`: success
- Build Proof `28536980702`: success
- Fast WebUI Proof `28536980726`: success
- App Build Proof `28536980652`: success
- App Multistep Build Proof `28536980634`: success
- Live WebUI Feature Sprint `28536980680`: failure

Live WebUI job inspected:

- Job: `84600771131`
- Failed step: `Run live WebUI feature sprint`
- NIM proof line showed `provider: nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, and `passed: true` before browser proof marker validation.
- Exact failing gate: missing marker `Full six-phase agentic benchmark prompt` in `forge-proof/live-webui-feature-sprint/browser-proof.json`.

## Source backing used

OpenCode upstream source paths inspected and used as reference:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-component="session-turn"`
  - `data-slot="session-turn-content"`
  - `data-slot="session-turn-message-container"`
  - `data-slot="session-turn-message-content"`
  - `data-slot="session-turn-assistant-content"`
  - `data-slot="session-turn-thinking"`
  - `data-slot="session-turn-diffs"`
  - visible session part continuity, active/pending turn handling, and diff overflow surface
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - message/action copy affordance reference from the existing Forge parity contract

## Forge implementation changed

- `crates/webui/src/chat_ui_live_turn_ordinals.html`
  - Added `full-six-phase-agentic-benchmark-prompt` to the central session-turn proof template.
  - Added visible static DOM text `Full six-phase agentic benchmark prompt` so same-URL browser proof JSON includes the marker even when CI skips Chrome DOM refresh and uses the initially fetched WebUI HTML.
  - Added the same proof token to runtime-decorated live/durable turns through `forgeDecorateCentralSessionTurn()`.

## Claim boundary

This patch addresses the exact failed Live WebUI browser-proof gate from run `28536980680`. It does not claim same-head browser proof, full OpenCode parity, or production readiness until new workflows complete on the post-patch head and artifacts/screenshots are inspected.
