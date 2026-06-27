# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest fully green baseline: `d052a279d7a5c37b275043ad0e52fb966a0be4eb`.
- Latest browser proof artifact: GitHub Actions artifact `7924965603` from Live WebUI Feature Sprint run `28290889903`.
- Next code target: exact OpenCode mutable ToolPart row updates from `packages/opencode/src/session/processor.ts`.

## Latest source-backed slices

- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — OpenCode ToolStateCompleted attachment parity.
- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity. WebUI SSE emits source-tagged lifecycle receipts for pending, input, running, completed, and error ToolPart transitions.

## OpenCode source references for latest proven slice

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — `ensureToolCall`, `updateToolCall`, `completeToolCall`, `failToolCall`, `toolResultOutput`, and stream event handling for tool-input/tool-call/tool-result/tool-error lifecycle.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `ToolPart`, and completed attachments.

## Current behavior

- WebUI uses the newer dark Codex/OpenCode-like theme.
- Natural proof note prompt creates a pending edit approval before writing.
- Approval route applies the patch and records FilePart/PatchPart only after approval.
- Approved `apply_patch` results persist source-shaped post-edit receipts for filesystem edits, watcher updates, LSP touch targets, and diagnostics touch metadata.
- Normal file tools (`file_write`, `file_edit`, `file_delete`) emit OpenCode-style file/watch/LSP receipts and attach FilePart entries to their completed ToolPart state.
- The WebUI SSE stream carries OpenCode SessionProcessor lifecycle metadata for pending input, input deltas, running tool calls, completed results, and error results.
- The change bus replays the latest persisted `.forge/change-events.jsonl` events on startup and continues sequence numbers after replay.
- `/api/events/status`, `/api/events/recent`, and the activity rail expose OpenCode-style bridge status including durability metadata.
- Conversation compaction stores a structured summary and recent tail, then publishes exact OpenCode `session.next.compaction.started` / `session.next.compaction.ended` event receipts.
- Existing session part cards remain: TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, PatchPart.

## Current gaps

- Live filesystem watcher integration is still receipt-backed rather than OS-watch backed.
- LSP touch receipts are not yet live diagnostics from a language server process.
- BOM and formatter parity are still incomplete.
- ToolPart lifecycle parity is improved for SSE lifecycle receipts and completed attachments, but exact mutable session row updates from OpenCode remain incomplete.
- Compaction summary creation is deterministic in Forge; full streamed NIM-backed compaction summary remains incomplete.

## Next targets

1. Implement exact mutable ToolPart processor parity: update the existing assistant ToolPart row by `callID` when `completeToolCall` / `failToolCall` happens, instead of only appending a separate tool-result message.
2. Keep browser proof requiring natural WebUI prompts and screenshots.
3. Continue toward OS-backed watcher/file edited events or live LSP diagnostics from OpenCode sources.
