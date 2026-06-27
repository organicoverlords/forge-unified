# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest fully green baseline before this slice: `e562d783538b884b16558b8a62c4e495423f02b3`.
- Latest browser proof artifact before this slice: Live WebUI Feature Sprint run `28295482726`, artifact `7926326967`.
- Current HEAD needs Actions/browser proof before a fresh green claim.

## Latest source-backed slices

- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — OpenCode ToolStateCompleted attachment parity.
- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity.
- `c3b826d7136298c7bb7d62ba30e11fd12cfeff70` — OpenCode watcher status and local mutable ToolPart proof path.
- `2680e673645ced1a799b3a5053885b11996301e0` — OpenCode LSP diagnostic report shape.
- `d2ecc6a4e9ca89a05fb7d8551b9a1b1c938bf114` — OpenCode FileMutation BOM preservation.
- `e562d783538b884b16558b8a62c4e495423f02b3` — formatter hook proof repaired and fully green.
- This slice — visible OpenCode LSP diagnostics rail: the event page now renders diagnostic totals, diagnostic file count, severity chips, and `Diagnostic.report` blocks instead of burying the LSP envelope in raw JSON.

## OpenCode source references for latest slice

- `anomalyco/opencode:packages/opencode/src/lsp/diagnostic.ts` — source for `MAX_PER_FILE`, severity label mapping, one-based `ERROR [line:col]` formatting, and `<diagnostics file="...">` report blocks.
- `anomalyco/opencode:packages/opencode/src/lsp/lsp.ts` — source for the LSP contract: `status`, `touchFile`, `diagnostics`, and connected/error client status shape.
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts` — source for streaming visible workspace activity through an EventV2 bridge.
- `anomalyco/opencode:packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts` — source for HTTP/SSE event delivery to UI consumers.

## Current behavior

- WebUI uses the newer dark Codex/OpenCode-like theme.
- Natural proof note prompt creates a pending edit approval before writing.
- Approval route applies the patch and records FilePart/PatchPart only after approval.
- Approved `apply_patch` results persist source-shaped post-edit receipts for filesystem edits, watcher updates, LSP touch targets, warmup containment, and diagnostics report metadata.
- LSP diagnostics receipts include OpenCode-shaped `severity_counts`, `diagnostic_count`, `max_per_file`, `report_block`, `report_emitted`, and `lsp_client_status` fields.
- The event rail visibly summarizes LSP diagnostics with totals, diagnostic file count, severity chips, per-event severity chips, and visible `report_block` output.
- Normal file tools (`file_write`, `file_edit`, `file_delete`) emit OpenCode-style file/watch/LSP receipts and attach FilePart entries to their completed ToolPart state.
- `file_write` and `file_edit` preserve existing/input UTF-8 BOMs and strip duplicate leading BOMs before writing at most one BOM.
- `file_write` and `file_edit` run a contained post-write formatter hook. `.rs` files use rustfmt when present; unavailable, spawn-failed, or nonzero formatter exits are recorded instead of failing the edit path.
- Formatter metadata is visible through ToolResult metadata as `formatter_status` and `opencode_formatter_source`.
- The WebUI SSE stream carries OpenCode SessionProcessor lifecycle metadata for pending input, input deltas, running tool calls, completed results, and error results.
- Conversation storage mutates the previous assistant message's matching ToolPart row from running to completed/error and records `opencode_mutable_tool_part_source`.
- The change bus replays the latest persisted `.forge/change-events.jsonl` events on startup and continues sequence numbers after replay.
- `/api/events/status`, `/api/events/recent`, and the activity rail expose OpenCode-style bridge status including durability metadata.
- Conversation compaction stores a structured summary and recent tail, then publishes exact OpenCode `session.next.compaction.started` / `session.next.compaction.ended` event receipts.
- Existing session part cards remain: TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, PatchPart.
- Live WebUI smoke now requires the visible OpenCode LSP diagnostics panel markers in the event-rail browser proof.

## Current gaps

- Current HEAD is not yet workflow/browser-proof green.
- Live filesystem watcher integration is still receipt-backed rather than OS-watch backed.
- LSP diagnostic report shape and UI are copied, but diagnostics are not yet collected from a live language server process.
- Formatter hook shape is implemented, but full OpenCode formatter catalog/config/runtime remains partial.
- Mutable ToolPart row parity is implemented for Forge's conversation snapshots, but OpenCode database-backed part IDs and `providerExecuted` delta updates are still partial.
- Compaction summary creation is deterministic in Forge; full streamed NIM-backed compaction summary remains incomplete.

## Next targets

1. Check latest Actions for this branch HEAD and fix exact failures.
2. Inspect browser proof artifacts and screenshot DOM for the visible LSP diagnostics panel.
3. Continue toward real LSP service/client process, full formatter catalog/config, OS-backed watcher/file edited events, or NIM-backed compaction from OpenCode sources.
