# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest fully green baseline before this slice: `c3b826d7136298c7bb7d62ba30e11fd12cfeff70`.
- Latest browser proof artifact before this slice: GitHub Actions artifact `7925108696` from Live WebUI Feature Sprint run `28291373990`.
- Current HEAD needs Actions/browser proof before a fresh green claim.

## Latest source-backed slices

- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — OpenCode ToolStateCompleted attachment parity.
- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity. WebUI SSE emits source-tagged lifecycle receipts for pending, input, running, completed, and error ToolPart transitions.
- `c3b826d7136298c7bb7d62ba30e11fd12cfeff70` — OpenCode watcher status and local mutable ToolPart proof path; CI, Build Proof, and Live WebUI Feature Sprint green.
- This slice — OpenCode LSP diagnostic report shape. Forge now records severity counts, max-per-file cap, report blocks, one-based error formatting, client containment status, and upstream `lsp.ts` / `diagnostic.ts` source metadata in post-edit diagnostics receipts.

## OpenCode source references for latest slice

- `anomalyco/opencode:packages/opencode/src/tool/apply_patch.ts` — after edits, OpenCode touches LSP documents, calls `lsp.diagnostics()`, and appends `LSP.Diagnostic.report(...)` output for each edited file with errors.
- `anomalyco/opencode:packages/opencode/src/lsp/lsp.ts` — defines `status`, `touchFile`, and `diagnostics` behavior for LSP clients.
- `anomalyco/opencode:packages/opencode/src/lsp/diagnostic.ts` — `Diagnostic.report` caps errors at 20 per file and formats `ERROR [line:col] message` inside a `<diagnostics file="...">` block.
- `anomalyco/opencode:packages/opencode/src/tool/read.ts` — optional LSP warmup defects are contained and must not fail successful file operations.

## Current behavior

- WebUI uses the newer dark Codex/OpenCode-like theme.
- Natural proof note prompt creates a pending edit approval before writing.
- Approval route applies the patch and records FilePart/PatchPart only after approval.
- Approved `apply_patch` results persist source-shaped post-edit receipts for filesystem edits, watcher updates, LSP touch targets, warmup containment, and diagnostics report metadata.
- LSP diagnostics receipts now include OpenCode-shaped `severity_counts`, `diagnostic_count`, `max_per_file`, `report_block`, `report_emitted`, and `lsp_client_status` fields.
- Normal file tools (`file_write`, `file_edit`, `file_delete`) emit OpenCode-style file/watch/LSP receipts and attach FilePart entries to their completed ToolPart state.
- The WebUI SSE stream carries OpenCode SessionProcessor lifecycle metadata for pending input, input deltas, running tool calls, completed results, and error results.
- Conversation storage now mutates the previous assistant message's matching ToolPart row from running to completed/error and records `opencode_mutable_tool_part_source`.
- The change bus replays the latest persisted `.forge/change-events.jsonl` events on startup and continues sequence numbers after replay.
- `/api/events/status`, `/api/events/recent`, and the activity rail expose OpenCode-style bridge status including durability metadata.
- Conversation compaction stores a structured summary and recent tail, then publishes exact OpenCode `session.next.compaction.started` / `session.next.compaction.ended` event receipts.
- Existing session part cards remain: TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, PatchPart.

## Current gaps

- Current HEAD is not yet workflow/browser-proof green.
- Live filesystem watcher integration is still receipt-backed rather than OS-watch backed.
- LSP diagnostic report shape is copied, but diagnostics are not yet collected from a live language server process.
- BOM and formatter parity are still incomplete.
- Mutable ToolPart row parity is now implemented for Forge's conversation snapshots, but OpenCode database-backed part IDs and `providerExecuted` delta updates are still partial.
- Compaction summary creation is deterministic in Forge; full streamed NIM-backed compaction summary remains incomplete.

## Next targets

1. Check latest Actions for this branch HEAD and fix exact failures.
2. Inspect browser proof artifacts and screenshot DOM for exact LSP diagnostic report fields.
3. Continue toward a real LSP service/client process, OS-backed watcher/file edited events, or formatter/BOM parity from OpenCode sources.
