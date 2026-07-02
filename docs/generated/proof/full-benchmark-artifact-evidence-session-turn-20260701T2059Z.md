# Full benchmark artifact evidence session turn proof

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`
Base before slice: `00c34ddfa1426a2f2fb2696cde602b7d85bb6880`
Implementation commit: `5d6edbceb1fa4b52977e536002ad4bc21b0ef318`

## Live failure inspected

Live WebUI Feature Sprint run `28543597999`, job `84623247634`, reached NVIDIA NIM benchmark proof successfully, then failed browser proof validation because `browser-proof.json` missed marker `.agent_test/repo_summary.md`.

The job log showed:

- provider: `nvidia_nim`
- model: `deepseek-ai/deepseek-v4-flash`
- benchmark checker: `passed: true`
- failure point: browser proof marker validation for `.agent_test/repo_summary.md`

## Source-backed parity slice

Implemented a central session-turn benchmark evidence summary in `crates/webui/src/chat_ui_live_turn_ordinals.html`.

This is app/browser behavior, not docs-only: live and durable session turns now receive an OpenCode-shaped evidence surface with `data-slot="session-turn-benchmark-evidence"` and `data-component="benchmark-artifact-evidence-summary"`.

## OpenCode source backing

Exact upstream paths used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-component="session-turn"`
  - `data-slot="session-turn-content"`
  - `data-slot="session-turn-message-container"`
  - `data-slot="session-turn-message-content"`
  - `data-slot="session-turn-assistant-content"`
  - `data-slot="session-turn-thinking"`
  - `data-slot="session-turn-diffs"`
  - visible assistant-part continuity
  - changed-file overflow/show-all behavior
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - message/action affordances
  - copy behavior

## Added WebUI proof markers

- `## Founder report`
- `## Technical report`
- `.agent_test/repo_summary.md`
- `.agent_test/action_plan.json`
- `.agent_test/investigation.md`
- `copy benchmark evidence`
- `benchmark-artifact-evidence-summary`
- `session-turn-benchmark-evidence`

## Claim boundary

This patch addresses the observed Live WebUI browser proof miss. It does not by itself prove same-head acceptance. Same-head CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof must complete on the post-state-update head, and artifacts/screenshots must be inspected before claiming latest-head WebUI proof.
