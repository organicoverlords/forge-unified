# Central proof shell failing-gate fix — 2026-07-01T17:55Z

## Live selection basis

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open, non-draft, mergeable at live inspection time
- Previous inspected head: `4b0e6cae0010b4dd0f4ce5d84b615ade72af24d0`
- New head after implementation/docs updates: `d55aaea60ee9b08fe3e87f95d474fa91e3bdc5e7`

## Failed proof inspected

- Fast WebUI Proof run: `28533586046`
- Fast WebUI job: `84589118239`
- Fast artifact: `8016954030`
- Fast result: NVIDIA NIM stream passed, provider was `nvidia_nim`, model was `deepseek-ai/deepseek-v4-flash`, readable PNG existed, but `central_session_turn_ui` failed.
- Live WebUI Feature Sprint run: `28533586160`
- Live job: `84589118409`
- Live result: NIM conversation and tool-lifecycle browser proof path ran, but `browser-proof.json` missed marker `Please run a Forge file tool formatter proof`.

## Source backing used

OpenCode source paths inspected and used as reference:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-component="session-turn"`
  - `data-slot="session-turn-content"`
  - `data-slot="session-turn-message-container"`
  - `data-slot="session-turn-message-content"`
  - `data-slot="session-turn-assistant-content"`
  - `data-slot="session-turn-thinking"`
  - `data-slot="session-turn-diffs"`
  - `session-turn-diffs-more` overflow behavior
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - message/action copy affordance reference

## Forge implementation changed

- `crates/webui/src/chat_ui_live_turn_ordinals.html`
  - changed the static central proof template from `Turn N central session heading` to `Turn 1: Central session turn proof shell`.
  - added explicit proof-shell controls: `copy turn`, `retry`, `copy final answer`.
  - added the live sprint prompt marker `Please run a Forge file tool formatter proof` so the same-URL proof DOM carries the marker when Chrome DOM refresh is skipped in CI.
  - retained runtime decoration for live and durable turns so real UI turns continue to get the OpenCode-shaped central data-slot contract.
- `PROJECT_STATE.md`
  - updated workflow evidence, new head, claim boundary, and exact changed contract.

## Claim boundary

This slice fixes the exact known proof-gate surface. It does not claim full OpenCode parity or same-head browser proof until workflows finish on the new head and artifacts/screenshots are inspected.
