# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current active-work HEAD after this slice: `d24d8e7183216aa8a50627b1bc280251d9171ee4`.
- Latest fully green baseline before this slice: `363599581e0101ef28e4a05546f9fa0d9c0a19b5`.
- Latest browser proof artifact before this slice: GitHub Actions artifact `7923762704` from Live WebUI Feature Sprint run `28286904374`.
- Current HEAD needs Actions/browser proof before a fresh green claim.

## Latest source-backed slices

- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay. The event rail status reports `durable`, `durable_log_path`, and `durable_replay_count`, and the engine appends emitted file/watch/LSP receipts to `.forge/change-events.jsonl` under the selected workspace.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity. Forge compaction now emits `session.next.compaction.started` and `session.next.compaction.ended` receipts with OpenCode-shaped `sessionID`, `messageID`, `reason`, `text`, and `recent` payload fields.

## OpenCode source references for latest slice

- `anomalyco/opencode:packages/core/src/session/compaction.ts` — head/recent split, structured summary template, and lifecycle publication around compaction.
- `anomalyco/opencode:packages/schema/src/session-event.ts` — canonical `SessionEvent.Compaction.Started` / `SessionEvent.Compaction.Ended` event types and durable inventory membership.
- Existing durable event bridge references remain recorded in status: `packages/opencode/src/event-v2-bridge.ts`, `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts`, `packages/opencode/src/tool/write.ts`, `packages/opencode/src/tool/edit.ts`, and `packages/opencode/src/tool/apply_patch.ts`.

## Current behavior

- WebUI uses the newer dark Codex/OpenCode-like theme.
- Natural proof note prompt creates a pending edit approval before writing.
- Approval route applies the patch and records FilePart/PatchPart only after approval.
- Approved `apply_patch` results persist source-shaped post-edit receipts for filesystem edits, watcher updates, LSP touch targets, and diagnostics touch metadata.
- The change bus replays the latest persisted `.forge/change-events.jsonl` events on startup and continues sequence numbers after replay.
- `/api/events/status`, `/api/events/recent`, and the activity rail expose OpenCode-style bridge status including durability metadata.
- Conversation compaction stores a structured summary and recent tail, then publishes exact OpenCode `session.next.compaction.started` / `session.next.compaction.ended` event receipts.
- Repo inspection still runs real `repo_info` and `file_list` tools with compact visible output and raw metadata preserved.
- Existing session part cards remain: TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, PatchPart.

## Current gaps

- Current HEAD is not yet workflow/browser-proof green.
- Live filesystem watcher integration is still receipt-backed rather than OS-watch backed.
- LSP touch receipts are not yet live diagnostics from a language server process.
- BOM and formatter parity are still incomplete.
- Full ToolPart lifecycle parity is still incomplete.
- Compaction summary creation is deterministic in Forge; full streamed NIM-backed compaction summary remains incomplete.

## Next targets

1. Check latest Actions for `d24d8e7183216aa8a50627b1bc280251d9171ee4` and fix exact failures.
2. Inspect browser proof artifacts and screenshot DOM for exact OpenCode compaction event-type receipts.
3. Continue toward OS-backed watcher/file edited events or full ToolPart lifecycle parity from OpenCode sources.
