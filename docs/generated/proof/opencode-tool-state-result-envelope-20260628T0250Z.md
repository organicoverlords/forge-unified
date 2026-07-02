# OpenCode tool state result envelope proof

Date: 2026-06-28
Branch: `mvp/nim-freellmapi-router-20260626`
Repo: `organicoverlords/forge-unified`

## Source of truth

Target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

Selection basis:

- PR #3 is open and non-draft for `mvp/nim-freellmapi-router-20260626`.
- No newer open PR superseded the target branch during this run.

## Failed workflow inspected

Current pre-slice HEAD: `6d5bf28390e9c573d0b6faf05e5093353952c604`.

Failed same-head workflows:

- Live WebUI Feature Sprint `28308137738`: failure.
- Build Proof `28308137749`: failure.
- CI `28308137748`: failure.

Primary inspected failure:

- Live WebUI job `83867976719` built `forge-app` successfully, then failed in `scripts/smoke/live-webui-feature-sprint.sh` with `line 96: syntax error near unexpected token '('`.
- The full benchmark checker then failed because `full-benchmark-conversation.json` and `full-benchmark-stream.sse` were missing.

## OpenCode source backing

Inspected upstream OpenCode paths:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `completeToolCall(...)` writes completed ToolPart state with `status: "completed"`, `input`, `output`, `metadata`, `title`, `time`, and optional `attachments`.
  - `failToolCall(...)` writes error ToolPart state with `status: "error"`, `input`, `error`, and `time`.
  - `ensureToolCall(...)` preserves `providerExecuted` metadata on existing ToolParts and newly created provider-executed tool parts.

Relevant copied behavior:

- Forge tool results now carry an OpenCode-style state envelope in metadata:
  - `opencode_tool_state_status`
  - `opencode_tool_state_title`
  - `opencode_tool_call_id`
  - `opencode_tool_state_time`
  - `opencode_tool_state_source`
  - `opencode_tool_output_shape`
  - `opencode_tool_error` for failed tools
- Doom-loop permission results also get the same state envelope, so interrupted loops are visible as structured error ToolPart states rather than only free text.

## Files changed

- `crates/engine/src/orchestrator.rs`
  - Added OpenCode-style tool state envelope metadata for completed/error provider-selected tool results.
  - Added source marker `packages/opencode/src/session/processor.ts:completeToolCall/failToolCall`.
  - Added titles such as `Read file`, `Wrote file`, `Ran shell`, `Ran task`, and `Failed <ToolKind>`.
- `scripts/smoke/live-webui-feature-sprint.sh`
  - Rewrote fragile model proof parsing around conversation creation/model extraction.
  - Added explicit proof markers for `opencode_tool_state_status` and `opencode_tool_state_title` in stream and conversation artifacts.
  - Kept NVIDIA NIM-only rejection gates for `provider: local`, `local_shortcut`, and scripted benchmark events.

## Proof expectation

The same-head Live WebUI Feature Sprint should now prove:

- Real NVIDIA NIM model route.
- Natural-language WebUI prompt flow.
- Tool lifecycle stream includes provider metadata and OpenCode-style state envelope markers.
- Full six-phase benchmark artifacts are produced only if the app completes the prompt and checker passes.

## Do not overclaim

This slice does not prove parity by itself. Same-head CI / Build Proof / Live WebUI Feature Sprint must complete and upload browser screenshot artifacts before this head can be called proven.
