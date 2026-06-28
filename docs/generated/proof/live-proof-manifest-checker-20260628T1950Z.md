# Live WebUI proof manifest checker slice — 2026-06-28T19:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open and non-draft
- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Live state before this slice

Same-head workflows for `6de537bc881c66348301dfb18ff50914e110eff0` were green:

- CI: `28333513946`, success
- Build Proof: `28333513947`, success
- Live WebUI Feature Sprint: `28333513957`, success
- Live WebUI artifact: `7937929953`, `live-webui-feature-sprint-proof`, digest `sha256:2196d93ea90df5b65b14f11d6c345edfc738bc7be22e8eec34412af3f6f61da2`

## Source-backed parity slice

Added `scripts/smoke/check-live-webui-proof-manifest.py`.

The checker treats the artifact directory as the source of truth and fails if a green run is missing any required proof file:

- `full-benchmark-webui.png`
- `full-benchmark-browser-proof.json`
- `full-benchmark-stream.sse`
- `full-benchmark-conversation.json`
- `full-benchmark-checker.json`
- `opencode-workflow-checker.json`
- `live-proof-status.txt`

It also validates:

- full benchmark checker passed
- OpenCode-style workflow checker passed
- stream contains `run-finish`
- stream contains at least eight tool calls and eight tool results
- conversation provider is exactly `nvidia_nim`
- model is recorded
- status records screenshot and workflow checker paths
- runtime stream does not contain local shortcuts or upstream identity markers

## OpenCode source backing

Reference path retained in docs only:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`

Relevant semantics:

- `completeToolCall` stores completed tool output, metadata, title, timing, and attachments on the ToolPart.
- `failToolCall` stores explicit error state and timing before settling the tool call.
- Forge's checker mirrors that artifact-first behavior: final claims count only when there is durable tool/proof output, not because a workflow name is green.

## Proof status

This slice is code/proof hardening, not a new live artifact yet.

After this commit, same-head CI / Build Proof / Live WebUI workflows must run again before claiming proof for the new head.
