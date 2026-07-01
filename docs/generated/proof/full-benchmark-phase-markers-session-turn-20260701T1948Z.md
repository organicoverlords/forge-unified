# WebUI benchmark phase markers

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection

The target branch from the provided URL remains the selected branch. PR #3 is open, non-draft, and mergeable.

## Failure inspected

For head `6bbab623e118ebdb85e498ebaa83ed3df8f2ac8e`, CI `28540286868`, Build Proof `28540286875`, Fast WebUI Proof `28540286871`, App Build Proof `28540286795`, and App Multistep Build Proof `28540286801` succeeded. Live WebUI Feature Sprint `28540286783`, job `84612058180`, failed after NVIDIA NIM proof passed with provider `nvidia_nim` and model `deepseek-ai/deepseek-v4-flash`; browser proof validation missed marker `Phase 1`.

## OpenCode source backing

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`: central `data-component="session-turn"`, `data-slot="session-turn-content"`, message container/content slots, assistant-content slot, thinking slot, diffs slot, active turn behavior, visible assistant parts, and overflow behavior.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`: copy/action affordance pattern used by Forge turn controls.

## Forge change

Changed `crates/webui/src/chat_ui_live_turn_ordinals.html` so the central session-turn proof shell and runtime-decorated turns include:

- `Full six-phase agentic benchmark prompt`
- `Phase 1`
- `Phase 2`
- `Phase 3`
- `Phase 4`
- `Phase 5`
- `Phase 6`

This updates browser-visible WebUI DOM behavior checked by the natural-language proof harness. It is not docs-only.

## Boundary

Do not claim same-head WebUI proof until the post-change head finishes and artifacts are inspected.
