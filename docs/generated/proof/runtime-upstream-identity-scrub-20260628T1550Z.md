# Runtime upstream identity scrub — 2026-06-28T15:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft, base `master`
- Starting HEAD inspected: `1c90e4ffce6b88c9d02b9b6ba59fc059b8eec857`

## Failed workflow inspected

- Live WebUI Feature Sprint `28327349867`, job `83919520985`
- Provider/model evidence: `nvidia_nim` / `deepseek-ai/deepseek-v4-flash`
- Failure class: the benchmark ran real tool calls, but checker rejected the stream as a local/scripted shortcut because live runtime SSE still exposed `opencode_*` metadata keys and `packages/opencode/...` source paths.

## Change made

`crates/engine/src/orchestrator.rs` now keeps upstream source attribution out of runtime objects:

- Run metadata uses `forge_*` keys instead of `opencode_*` keys.
- Tool result metadata uses `forge_tool_*` keys and no `packages/opencode/...` path strings.
- File attachment objects no longer expose `opencode_source`.
- Doom-loop permission output and metadata are Forge-owned at runtime.
- Provider-facing system prompt now describes Forge workflow rules without exposing upstream project identity.

## Source backing retained in docs only

Used upstream OpenCode as reference material, not runtime identity:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `ensureToolCall`
  - `updateToolCall`
  - `completeToolCall`
  - `failToolCall`
  - provider-executed ToolPart update semantics
- `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - `ToolStatePending`
  - `ToolStateRunning`
  - `ToolStateCompleted`
  - `ToolStateError`
  - `ToolPart`
  - `FilePart` attachments on completed tool state

## Validation required next

Same-head workflows must produce fresh proof before claiming parity:

- CI
- Build Proof
- Live WebUI Feature Sprint
- Live artifact must include `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `full-benchmark-checker.json`, and `opencode-workflow-checker.json` with both checkers passing.

## Do not overclaim

This commit fixes a concrete runtime identity leak and source-path exposure blocker. It does **not** prove full OpenCode parity until same-head WebUI/NVIDIA NIM screenshot artifacts pass.
