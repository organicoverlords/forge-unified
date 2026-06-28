# OpenCode Parity Tracker

Updated: 2026-06-28

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Source-backed status

| OpenCode source | Forge status |
|---|---|
| `packages/opencode/src/tool/apply_patch.ts` | Forge implements patch parsing, approval metadata, changed-file summaries, post-edit receipts, watcher update receipts, LSP touch/warmup receipts, diagnostic report envelopes, and now advertises provider-visible `apply_patch` with `patchText` schema and source metadata. |
| `packages/opencode/src/session/processor.ts` | Forge emits source-tagged WebUI SSE lifecycle receipts for `tool-input-start`, `tool-input-delta`, `tool-input-end`, `tool-call`, `tool-result`, and `tool-error`; renders live ToolPart lifecycle cards; tracks `providerExecuted` metadata; cites this source for provider-visible task/batch/tool stream catalog entries; interrupts repeated identical tool request batches after the OpenCode `DOOM_LOOP_THRESHOLD` of 3; and now records an OpenCode-style `permission: doom_loop` envelope with `patterns`, `always`, `ruleset`, `input`, and source metadata before blocking the loop. |
| `packages/schema/src/v1/session.ts` | ToolPart lifecycle receipt slice is implemented for pending, running, completed, and error states. TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart now include deterministic schema-compatible `id`, `sessionID`, and `messageID` fields. |
| `packages/schema/src/session-id.ts` | Generated Forge session parts now use `ses_`-prefixed deterministic session IDs to match OpenCode SessionID prefix rules. |
| `packages/opencode/src/tool/write.ts` / `edit.ts` / `read.ts` | Forge file tools are exposed to providers with source-path metadata and keep copied BOM, formatter, watcher, LSP, and attachment proof behavior. |
| `packages/opencode/src/tool/bash.ts` | Forge advertises bounded `shell_command` and `terminal_run` tool schemas for repo inspection and validation. |
| `packages/opencode/src/tool/glob.ts` / `grep.ts` / `ls.ts` | Forge advertises provider-visible file discovery tools: `file_glob`, `file_search`, and `file_list`. |
| `packages/opencode/src/tool/webfetch.ts` | Forge advertises provider-visible `web_fetch` and `web_search` entries when network tools are available. |
| `packages/core/src/file-mutation.ts` | Forge file_write/file_edit copy `writeTextPreservingBom`: preserve an existing/input UTF-8 BOM and emit at most one BOM. |
| `packages/opencode/src/format/index.ts` | Forge has a contained `Format.file`-style hook for file_write/file_edit. `.rs` files use rustfmt when available; full formatter catalog/config remains partial. |
| `packages/core/src/filesystem/watcher.ts` | Forge starts a native workspace watcher, records backend/native subscription status, and publishes native `watcher.updated` events mapped to add/change/unlink. |
| `packages/opencode/src/lsp/lsp.ts` / `diagnostic.ts` | Forge mirrors diagnostic report envelopes, severity labels/counts, report blocks, and visible event-rail presentation. Live process-backed LSP clients remain incomplete. |
| `packages/core/src/session/compaction.ts` | Forge selects old context versus recent tail, serializes old context, stores a structured Markdown summary, and publishes compaction started/ended receipts. |
| `packages/opencode/src/event-v2-bridge.ts` | Forge event rail uses the same bridge concept for filesystem, watcher, LSP, compaction lifecycle receipts, and in-chat SSE receipt rows. |

## Implemented behavior

- `tool_definitions()` now exposes the full Forge executor surface to providers: repo/file discovery, read/write/edit/delete, patch/propose patch, shell/terminal, task, batch_parallel, web, browser proof, vision review, graph, and mode switching.
- Each provider-visible tool schema includes `provider_visible: true` and an `opencode_source` path.
- `/api/tools` returns `opencode_provider_tool_catalog`, the catalog count, names, required tool names, upstream source paths, and the full tool schema list.
- The WebUI renders an `OpenCode Tool Catalog` panel showing provider-visible tools, including `apply_patch`.
- Live smoke proof requires `/api/tools` to include at least 20 tools, `apply_patch`, `task`, `batch_parallel`, `patchText`, and OpenCode source paths.
- Browser proof requires visible `OpenCode Tool Catalog` and `apply_patch` markers in the natural WebUI tool-lifecycle and full benchmark screenshots/DOM.
- Tool lifecycle receipts include pending, input-start/input-delta/input-end, running, completed, and error stages.
- WebUI streams live ToolPart lifecycle cards and an in-chat EventV2Bridge receipt rail.
- Tool results update the existing assistant-message `tool_parts` row for the matching `callID`.
- Generated OpenCode-style parts now include deterministic `prt_`, `ses_`, and `msg_` prefixed base fields with source metadata.
- The orchestrator now detects three consecutive identical tool request batches and records a structured OpenCode-style doom-loop permission envelope before safely interrupting the repeated loop.
- File write/edit operations preserve UTF-8 BOMs and run contained formatter hooks.
- Native watcher, event bus, LSP diagnostic envelopes, and compaction receipts remain visible and proofed.

## Not done / do not overclaim

- Current HEAD is not yet same-head workflow/browser-proof green.
- The doom-loop slice now records the `permission.ask`-style envelope, but does not yet implement an interactive allow/deny recovery UI.
- Full provider-executed tool execution remains incomplete; Forge-owned tools explicitly mark `providerExecuted: false` while preserving the copied metadata/delta shape.
- Database-backed OpenCode part persistence remains partial; Forge currently emits deterministic schema-shaped IDs, not OpenCode database rows.
- Live language-server process/client diagnostics remain incomplete.
- Full OpenCode formatter catalog/config/runtime remains partial.
- LLM-streamed compaction summaries through NVIDIA NIM remain incomplete.
- Durable EventV2 aggregate replay/storage semantics beyond current JSONL replay remain incomplete.

## Current highest-priority parity gaps

1. True providerExecuted tool calls from the NIM/provider stream using the advertised catalog.
2. Interactive OpenCode doom-loop allow/deny recovery UI after the permission envelope is emitted.
3. Live LSP diagnostics from a running language server process.
4. Full OpenCode formatter catalog/config/runtime beyond rustfmt.
5. OpenCode prompt/system behavior.
6. LLM-backed compaction summary generation.
7. Durable session/message/part persistence beyond current snapshots.
