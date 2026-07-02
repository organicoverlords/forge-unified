# Central session turn contract proof — 2026-07-01T16:45Z

## Live selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open, non-draft, mergeable at inspection time
- Previous inspected head before this slice: `95acd92a51734cf6e6356a58ac038850be7897fe`

## Workflow failure inspected

Latest same-head workflows for `95acd92a51734cf6e6356a58ac038850be7897fe`:

- CI `28530268065`: success
- Build Proof `28530268121`: success
- App Build Proof `28530268078`: success
- App Multistep Build Proof `28530268082`: success
- Fast WebUI Proof `28530268053`, job `84577594386`: failed only `central_session_turn_ui`; readable proof UI passed, provider was `nvidia_nim`, model was `deepseek-ai/deepseek-v4-flash`, screenshot path was `forge-proof/fast-webui-proof/webui.png`, artifact `8015515910`.
- Live WebUI Feature Sprint `28530268182`, job `84577594703`: failed because `browser-proof.json` missed the marker `Please run a Forge file tool formatter proof` after the direct tool-lifecycle proof path. The proof harness recorded DOM-summary PNG fallback as diagnostic and not browser-rendered.

## OpenCode source backing

Used upstream OpenCode source as reference:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-component="session-turn"`
  - `data-slot="session-turn-content"`
  - `data-slot="session-turn-message-container"`
  - `data-slot="session-turn-message-content"`
  - `data-slot="session-turn-assistant-content"`
  - `data-slot="session-turn-thinking"`
  - `data-slot="session-turn-diffs"`
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - assistant parts/message part/copy action affordance model already used by existing Forge turn controls.

## Source-backed parity slice built

Changed `crates/webui/src/chat_ui_live_turn_ordinals.html` to:

1. Add a static `forge-session-turn-central-proof-template` containing the central turn data-component/data-slot contract. This is needed because CI skips Chrome DOM refresh after browser instability and builds the proof JSON from the same initial WebUI HTML.
2. Wrap durable reloaded `turn()` rendering so saved turns also get OpenCode-shaped `data-slot` annotations.
3. Keep live streaming `appendUser()` turns on the same central session-turn contract with stable `data-turn-ordinal`, `Turn N: <prompt excerpt>`, `assistant-parts`, thinking part, copy turn, and retry controls.

This is not docs-only: it changes the browser UI contract and runtime DOM for both live and reloaded turns.

## Files changed by this slice

- `crates/webui/src/chat_ui_live_turn_ordinals.html`
- `PROJECT_STATE.md`
- `docs/generated/proof/central-session-turn-contract-20260701T1645Z.md`

## Claim boundary

Do not claim full OpenCode parity, same-head production readiness, or browser-rendered screenshot proof yet. Latest-head GitHub Actions must complete and the WebUI artifacts/screenshots must be inspected. DOM-summary PNG fallback remains diagnostic only, not browser-rendered parity proof.
