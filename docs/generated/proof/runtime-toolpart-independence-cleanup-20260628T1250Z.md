# Runtime ToolPart independence cleanup — 2026-06-28T12:50Z

## Selection

- Repository: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open / non-draft
- Source of truth: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Failure inspected

Current-head Live WebUI Feature Sprint run `28321804109`, job `83904916420`, compiled `forge-app`, started the WebUI, completed the smaller NIM/tool lifecycle proof stages, then timed out during the full six-phase benchmark stream after about 20 minutes with no full benchmark conversation/stream artifact produced.

Important lines from the inspected log:

- `Finished dev profile ... forge-app`
- `start webui`
- `create NIM conversation`
- `model chat stream`
- `create tool lifecycle conversation`
- `browser proof tool lifecycle`
- `create full benchmark conversation`
- `curl: (28) Operation timed out after 1190002 milliseconds with 0 bytes received`
- `missing full benchmark conversation or stream artifact`

So the previous proof harness quoting failure is fixed, but current-head same-head full benchmark proof remains blocked by the long live NIM benchmark stream not producing artifacts before timeout.

## OpenCode source backing

OpenCode remains a developer reference only. Exact upstream paths used for behavior comparison:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - ToolPart lifecycle/update semantics: ensure/update/complete/fail, providerExecuted handling, and repeated identical tool-call guard shape.
- `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - ToolPart / ToolState / FilePart envelope shape.

## Feature slice built

Changed `crates/webui/src/events.rs` so runtime ToolPart lifecycle/result payloads use Forge-owned names only:

- Removed runtime constants that exposed `packages/opencode/src/session/processor.ts` and `packages/schema/src/v1/session.ts`.
- Replaced `opencode_provider_executed_source`, `opencode_tool_input`, `opencode_session_processor`, and `opencode_lifecycle_stage` metadata keys with Forge-owned keys.
- Replaced lifecycle metadata fields `opencode_source` and `schema_source` with Forge-owned `source` and `schema` labels.
- Reworded WebUI-visible summaries so Forge does not identify itself as OpenCode or expose OpenCode source paths in provider-visible/runtime output.
- Kept behavior-compatible ToolPart lifecycle receipts: pending, input start/delta/end, running tool call, completed/error result, `providerExecuted`, `ToolStateCompleted.attachments`, file-change receipts, and doom-loop threshold metadata.

## Proof status

- Source change committed at `b9cbe2c669c061455b70293659249f0e25cd3558`.
- Same-head GitHub workflow proof is pending after subsequent state/doc commits.
- No current-head parity claim is made until CI / Build Proof / Live WebUI Feature Sprint return green and a same-head WebUI/NVIDIA NIM screenshot artifact exists.

## Remaining blocker

The full six-phase benchmark stream currently stalls long enough that the proof harness times out without writing `full-benchmark-stream.sse` / `full-benchmark-conversation.json`. The next useful slice is to make the live chat stream emit incremental SSE progress while the model/tool loop is running, instead of buffering all events until `state.agent.chat(...).await` returns.