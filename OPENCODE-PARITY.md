# OpenCode Parity Tracker

Updated: 2026-06-27

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Source-backed status

| OpenCode source | Forge status |
|---|---|
| `packages/schema/src/v1/session.ts` | ToolPart lifecycle receipt slice is implemented for pending, running, completed, and error states. TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart remain proofed. Completed ToolParts can carry FilePart attachments. |
| `packages/opencode/src/session/processor.ts` | Forge now emits source-tagged WebUI SSE lifecycle receipts for `tool-input-start`, `tool-input-delta`, `tool-input-end`, `tool-call`, `tool-result`, and `tool-error`, following `ensureToolCall`, `updateToolCall`, `completeToolCall`, and `failToolCall`. Forge also updates the existing assistant-message ToolPart row by `callID` when a result is recorded, with `mutable_tool_part_updates` receipts. Exact database-backed part IDs and provider-executed deltas remain partial. |
| `packages/core/src/file-mutation.ts` | Forge file_write/file_edit now copy `writeTextPreservingBom`: preserve an existing/input UTF-8 BOM and emit at most one BOM. Tool results expose `bom`, `bom_preserved`, and `bom_strategy` metadata. |
| `packages/opencode/src/tool/apply_patch.ts` | Patch parsing, approval gate, changed-file summaries, post-edit receipts, watcher update receipts, LSP touch/warmup receipts, LSP diagnostic report envelopes, and event-bus status metadata are implemented. Live LSP collection and formatter hooks remain incomplete. |
| `packages/opencode/src/lsp/lsp.ts` | Forge now records the copied LSP service contract in diagnostics metadata: status/touchFile/diagnostics are explicit, clients are reported as contained when no live LSP backend is attached, and missing clients do not fail an edit path. |
| `packages/opencode/src/lsp/diagnostic.ts` | Forge now mirrors the `Diagnostic.report` result shape: severity labels/counts, one-based report formatting, `<diagnostics file="...">` blocks for errors, and `MAX_PER_FILE` capped at 20. |
| `packages/opencode/src/tool/write.ts` | Used as a source for the post-write sequence: permission gate, BOM-preserving write, optional format, `FileSystem.Event.Edited`, `Watcher.Event.Updated`, LSP touch, diagnostics collection, and diagnostic reporting. Forge mirrors this as event status metadata and proof requirements; formatter hooks remain partial. |
| `packages/opencode/src/tool/edit.ts` | Used as a source for locked edit, BOM-preserving text write, post-edit filesystem/watcher publishing, LSP touch, and `LSP.Diagnostic.report` behavior. Forge surfaces the copied event bridge status in API/UI proof; deeper file_edit parity remains. |
| `packages/opencode/src/tool/read.ts` | Forge copies the safety contract from the recent OpenCode LSP warmup fix: optional LSP warmup defects are contained and surfaced as `lsp.warmup.contained` activity instead of being allowed to fail the user-visible edit/proof path. |
| `packages/opencode/src/patch/index.ts` | Parser and derive/apply replacement behavior partially copied into Rust helpers. |
| `packages/core/src/session/compaction.ts` | Forge selects an old head versus recent tail, serializes old context, emits a structured Markdown `compaction_summary`, stores `compaction_recent`, keeps a recent tail, and publishes compaction started/ended receipts into the WebUI event rail. LLM-streamed summary generation remains incomplete. |
| `packages/opencode/src/event-v2-bridge.ts` | Forge event rail uses the same publish/bridge concept for visible filesystem, watcher, LSP warmup, LSP diagnostics, and compaction lifecycle receipts. New `/api/events/status` summarizes EventV2Bridge-style activity by sequence range, event type, source, and touched files. |
| `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts` | Forge streams change-bus events through SSE and sends a `server.connected` payload containing the event bridge status summary. |

## Implemented behavior

- Tool lifecycle receipts include pending, input-start/input-delta/input-end, running, completed, and error stages.
- Tool lifecycle SSE payloads cite `packages/opencode/src/session/processor.ts` and `packages/schema/src/v1/session.ts`.
- Tool results now update the existing assistant-message `tool_parts` row for the matching `callID`, mirroring OpenCode SessionProcessor's `session.updatePart` completion/error path.
- The recorded tool-result message includes `mutable_tool_part_updates`, and the updated assistant message includes `opencode_mutable_tool_part_source`.
- File write/edit operations preserve an existing or input UTF-8 BOM and strip duplicate leading BOMs before writing at most one BOM.
- File tool results expose `bom`, `bom_preserved`, `bom_strategy`, and `packages/core/src/file-mutation.ts` metadata through WebUI proof.
- Patch flow records post-edit receipts after confirmed edits.
- Session part cards are visible in WebUI proof.
- The event rail displays filesystem, watcher, LSP warmup containment, and LSP diagnostic event-envelope activity.
- LSP diagnostics metadata now includes `severity_counts`, `diagnostic_count`, `max_per_file`, `report_block`, `report_emitted`, `lsp_client_status`, and copied source paths for `lsp.ts` and `diagnostic.ts`.
- The event rail has an OpenCode EventV2Bridge-style status panel showing event count, sequence range, by-type counts, by-source counts, and latest touched files.
- `/api/events/recent` includes the status summary; `/api/events/status` returns it directly.
- Compaction produces an OpenCode-shaped structured summary and preserves recent tail context.
- Compaction emits visible start/end lifecycle events with OpenCode source metadata and event-bus receipts.
- Browser proof script waits on health, index, event page, event API, and event status readiness before sending the natural WebUI prompt, then requires screenshots for both the chat UI and event rail.

## Not done / do not overclaim

- Database-backed OpenCode part IDs and providerExecuted delta semantics are still partial.
- Live language-server process/client diagnostics; current slice copies diagnostic report shape and containment only.
- Formatter hooks after write/edit are still partial.
- LLM-streamed compaction summaries through NVIDIA NIM.
- Durable EventV2 aggregate replay/storage semantics.
- AgentPart, RetryPart, StepStartPart, StepFinishPart, and SubtaskPart only when backed by a real Forge behavior path.

## Current highest-priority parity gaps

1. Formatter hooks after write/edit.
2. Live LSP diagnostics from a running language server process.
3. OpenCode prompt/system behavior.
4. LLM-backed compaction summary generation.
5. Durable session/message/part persistence beyond current snapshots.
