# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Selected active work: newest open PR with meaningful app changes.
- Latest fully green baseline before this run: `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed`.
- Latest browser proof artifact before this run: Live WebUI Feature Sprint run `28299351117`, artifact `7927453969`.
- Current HEAD needs Actions/browser proof before a fresh green claim.

## Latest source-backed slice

Forge now exposes an OpenCode-backed provider-visible tool catalog and renders it in the WebUI so browser proof can verify the actual tool set advertised to NIM/provider execution.

Upstream OpenCode source paths used:

- `anomalyco/opencode:packages/opencode/src/tool/apply_patch.ts` — `apply_patch` parameter shape (`patchText`), permission metadata, apply/format/watch/LSP/diagnostic flow.
- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — source for provider/tool stream lifecycle, `providerExecuted`, task/tool call processing, and same-call ToolPart updates.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — source for ToolPart/FilePart schema naming used in proof metadata.
- `anomalyco/opencode:packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, and `webfetch.ts` — source path anchors for the provider-visible tool catalog entries.

Copied / improved behavior:

- `tool_definitions()` now exposes the full Forge tool executor surface instead of only the minimal repo/file/shell subset.
- `apply_patch`, `propose_patch`, `task`, `batch_parallel`, `web_fetch`, `web_search`, `browser_proof`, `vision_review`, `graph_build`, `graph_query`, `terminal_run`, and `switch_mode` are now advertised to providers with JSON schema and exact OpenCode source-path metadata.
- `/api/tools` returns the provider-visible catalog, tool count, required OpenCode-style tool names, and upstream source references.
- The WebUI renders an `OpenCode Tool Catalog` panel showing provider-visible tools including `apply_patch`.
- The live WebUI smoke now requires `/api/tools` to include at least 20 provider-visible tools, `apply_patch`, `task`, `batch_parallel`, `patchText`, and OpenCode source paths.
- Browser proof now requires the visible `OpenCode Tool Catalog` and `apply_patch` markers in both the tool-lifecycle proof and full benchmark proof.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI tool prompt renders live ToolPart lifecycle cards with `providerExecuted` metadata.
- File-change and EventV2Bridge receipts are visible in chat.
- Normal file tools emit OpenCode-style file/watch/LSP receipts, formatter metadata, BOM metadata, and completed ToolPart attachments.
- Native watcher publishes `watcher.started` and live `watcher.updated` events.
- LSP diagnostic envelopes and report blocks are visible in the event rail, but live language-server collection remains incomplete.
- Conversation compaction emits OpenCode `session.next.compaction.started` and `session.next.compaction.ended` receipts.

## Current gaps

- Current HEAD is not yet workflow/browser-proof green.
- The provider-visible catalog is advertised and UI-proofed, but full provider-side/providerExecuted=true tool execution is still incomplete for Forge-owned tools.
- Live language-server process/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.
- OpenCode database-backed part IDs remain partial.

## Next targets

1. Check same-head CI, Build Proof, and Live WebUI Feature Sprint for the provider tool catalog head.
2. If WebUI smoke fails, inspect `tool-catalog.json`, browser DOM, `tool-lifecycle-stream.sse`, and `server.log` before rerunning.
3. Continue toward true providerExecuted tool calls from the NIM/provider stream, live LSP diagnostics, full formatter registry, deeper watcher parity, or NIM-backed compaction.
