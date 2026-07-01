# Live turn ordinal session surface proof

Date: 2026-07-01

## Selection basis

- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- Selected PR: #3 `mvp router slice`.
- PR state before this slice: open, non-draft, mergeable.
- Previous inspected head: `a617a4e3122d9e6253119b8b7da946a1c670b992`.

## Workflow state inspected before this slice

For head `a617a4e3122d9e6253119b8b7da946a1c670b992`:

- CI `28526586260`: success.
- Build Proof `28526586280`: success.
- App Build Proof `28526586232`: success.
- App Multistep Build Proof `28526586406`: success.
- Fast WebUI Proof `28526586395`: failed.
- Live WebUI Feature Sprint `28526586204`: failed.

Fast WebUI job `84564544941` reached NVIDIA NIM WebUI streaming and produced a readable PNG, provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, but failed `central_session_turn_ui` because the proof text did not contain the expected central turn header token.

Fast artifact: `8013934520`.

## Source-backed app slice

Implemented WebUI live-turn ordinal surfacing so the central session turn has the same visible ordinal shape while streaming as it has after durable conversation reload:

- `Turn N: <prompt excerpt>` title on the active live turn.
- `data-turn-ordinal` and `live-session-turn-ordinal` proof marker.
- Existing `copy turn`, `retry`, assistant-parts, thinking/working, and message-part behavior preserved.

This is not docs-only and not cosmetic-only: the browser UI now exposes a stable live session-turn identity to the user while the natural-language WebUI prompt is actively running.

## OpenCode source backing used

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx` — central session turn structure, visible session part continuity, delayed/visible turn surfaces.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx` — message/action button behavior and copy affordances.

## Forge files changed

- `crates/webui/src/chat_ui_live_turn_ordinals.html`
- `crates/webui/src/chat_ui.rs`
- `PROJECT_STATE.md`
- `docs/generated/proof/live-turn-ordinal-session-surface-20260701T1552Z.md`

## Claim boundary

No same-head parity claim is made yet. Latest-head NVIDIA NIM WebUI proof must still complete and artifact screenshots must be inspected on the exact final head.
