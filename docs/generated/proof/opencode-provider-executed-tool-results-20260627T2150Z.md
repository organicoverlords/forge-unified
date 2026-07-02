# OpenCode parity proof — provider-executed tool results

Date: 2026-06-27T21:50Z
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Source of truth

Target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

Selected branch remains `mvp/nim-freellmapi-router-20260626` because PR #3 is open, non-draft, and its head branch is the requested target branch.

## OpenCode source backing

Upstream paths used:

- `anomalyco/opencode/packages/opencode/src/session/processor.ts`
  - `ensureToolCall(...)` stores `metadata.providerExecuted` when the provider owns the tool call.
  - Existing tool calls are upgraded to `providerExecuted: true` when later provider metadata proves provider execution.
  - `tool-call` and `tool-result` processing keeps one same-call ToolPart lifecycle instead of creating unrelated result records.
- `anomalyco/opencode/packages/schema/src/v1/session.ts`
  - ToolPart / ToolState metadata shape is the compatibility target already tracked in `tool_parts.rs`.

## Implemented Forge slice

Forge now annotates every tool result produced from an LLM/provider-selected tool call inside the orchestrated model loop with:

- `providerExecuted: true`
- `provider_executed: true`
- `opencode_provider_executed_source: "packages/opencode/src/session/processor.ts:ensureToolCall/updateToolCall/completeToolCall"`
- preserved `opencode_tool_input` when available

This moves the orchestrated model-tool loop closer to OpenCode processor semantics: provider-selected calls now carry explicit provider-executed metadata through the ToolResult and completed/error ToolPart state metadata that the WebUI can render/prove.

## Files changed

- `crates/engine/src/orchestrator.rs`
- `docs/generated/proof/opencode-provider-executed-tool-results-20260627T2150Z.md`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`

## Validation status

Connector-side code editing completed through GitHub API. Same-head GitHub Actions must prove compile/browser behavior. At the time this proof was written, previous same-head workflows for the immediately preceding HEAD were still in progress, and no green same-head WebUI/NVIDIA NIM screenshot artifact exists for this new commit yet.

## Do not overclaim

This is not full OpenCode parity. It improves provider-executed metadata propagation for orchestrated provider-selected tool calls. Browser screenshot proof with NVIDIA NIM only is still required before live parity can be claimed for this HEAD.
