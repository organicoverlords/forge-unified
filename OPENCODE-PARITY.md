# OpenCode Parity Tracker

Updated: 2026-06-27

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Source-backed status

| OpenCode source | Forge status |
|---|---|
| `packages/schema/src/v1/session.ts` | ToolPart lifecycle receipt slice is implemented for pending, running, completed, and error states. TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart remain proofed. Completed ToolParts can carry FilePart attachments. |
| `packages/opencode/src/session/processor.ts` | Forge emits source-tagged WebUI SSE lifecycle receipts for `tool-input-start`, `tool-input-delta`, `tool-input-end`, `tool-call`, `tool-result`, and `tool-error`, following `ensureToolCall`, `updateToolCall`, `completeToolCall`, and `failToolCall`. Forge updates the existing assistant-message ToolPart row by `callID` when a result is recorded. The WebUI now renders live ToolPart lifecycle cards and explicit `providerExecuted` metadata while the natural prompt is running. Forge-owned tools mark `providerExecuted: false`; upstream/provider-executed true deltas are documented and preserved as metadata shape, but full provider-side execution remains incomplete. |
| `packages/core/src/file-mutation.ts` | Forge file_write/file_edit copy `writeTextPreservingBom`: preserve an existing/input UTF-8 BOM and emit at most one BOM. Tool results expose `bom`, `bom_preserved`, and `bom_strategy` metadata. |
| `packages/opencode/src/format/index.ts` | Forge has a contained `Format.file`-style hook for file_write/file_edit. It matches formatter commands by extension, runs rustfmt for `.rs` when available, contains missing/spawn/exit failures, records `formatter_status`, and re-syncs BOM after formatter mutation. Full OpenCode formatter catalog/config remains partial. |
| `packages/core/src/filesystem/watcher.ts` | Forge starts a native workspace watcher when the tool executor starts. It records backend detection, native subscription status, ignore/protected paths, subscribe timeout metadata, `watcher.started`, and native `watcher.updated` events mapped to add/change/unlink. This copies OpenCode's watcher service flow: backend detection, protected/ignored paths, live callback, contained errors, and EventV2 publishing. |
| `packages/opencode/src/tool/apply_patch.ts` | Patch parsing, approval gate, changed-file summaries, post-edit receipts, watcher update receipts, LSP touch/warmup receipts, LSP diagnostic report envelopes, and event-bus status metadata are implemented. Live LSP collection remains incomplete. |
| `packages/opencode/src/lsp/lsp.ts` | Forge records the copied LSP service contract in diagnostics metadata and makes the event rail expose the LSP status/touch/diagnostics flow as first-class visible diagnostic cards. Live process-backed clients remain incomplete. |
| `packages/opencode/src/lsp/diagnostic.ts` | Forge mirrors the `Diagnostic.report` result shape: severity labels/counts, one-based report formatting, `<diagnostics file="...">` blocks for errors, and `MAX_PER_FILE` capped at 20. The event rail renders severity chips, diagnostic totals, diagnostic file count, and report blocks without forcing raw JSON inspection. |
| `packages/opencode/src/tool/write.ts` | Used as a source for the post-write sequence: permission gate, BOM-preserving write, `Format.file(filepath)`, BOM sync, `FileSystem.Event.Edited`, `Watcher.Event.Updated`, LSP touch, diagnostics collection, and diagnostic reporting. Forge mirrors this for file tools with formatter metadata and smoke proof requirements. |
| `packages/opencode/src/tool/edit.ts` | Used as a source for locked edit, BOM-preserving text write, `Format.file(filepath)`, BOM sync, post-edit filesystem/watcher publishing, LSP touch, and `LSP.Diagnostic.report` behavior. Forge surfaces the copied event bridge and formatter status in API/UI proof; deeper file_edit replacement parity remains. |
| `packages/opencode/src/tool/read.ts` | Forge copies the safety contract from the recent OpenCode LSP warmup fix: optional LSP warmup defects are contained and surfaced as `lsp.warmup.contained` activity instead of failing the user-visible edit/proof path. |
| `packages/opencode/src/patch/index.ts` | Parser and derive/apply replacement behavior partially copied into Rust helpers. |
| `packages/core/src/session/compaction.ts` | Forge selects an old head versus recent tail, serializes old context, emits a structured Markdown `compaction_summary`, stores `compaction_recent`, keeps a recent tail, and publishes compaction started/ended receipts into the WebUI event rail. LLM-streamed summary generation remains incomplete. |
| `packages/opencode/src/event-v2-bridge.ts` | Forge event rail uses the same publish/bridge concept for visible filesystem, watcher, LSP warmup, LSP diagnostics, compaction lifecycle receipts, and now in-chat EventV2Bridge receipt rows from live SSE. `/api/events/status` summarizes EventV2Bridge-style activity by sequence range, event type, source, and touched files. |
| `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts` | Forge streams change-bus events through SSE and sends a `server.connected` payload containing the event bridge status summary. |

## Implemented behavior

- Tool lifecycle receipts include pending, input-start/input-delta/input-end, running, completed, and error stages.
- Tool lifecycle SSE payloads cite `packages/opencode/src/session/processor.ts` and `packages/schema/src/v1/session.ts`.
- Tool lifecycle SSE payloads now expose `providerExecuted`, `providerExecuted_delta`, and the copied OpenCode repeated identical tool-call guard threshold shape (`DOOM_LOOP_THRESHOLD = 3`).
- The WebUI renders live OpenCode ToolPart lifecycle cards while the run is streaming, instead of only showing final tool results after completion.
- The WebUI renders file-change and event-bus receipts as an in-chat EventV2Bridge rail.
- Tool results update the existing assistant-message `tool_parts` row for the matching `callID`, mirroring OpenCode SessionProcessor's `session.updatePart` completion/error path.
- The recorded tool-result message includes `mutable_tool_part_updates`, and the updated assistant message includes `opencode_mutable_tool_part_source`.
- File write/edit operations preserve an existing or input UTF-8 BOM and strip duplicate leading BOMs before writing at most one BOM.
- File write/edit operations run a contained formatter hook after writing. `.rs` files use `rustfmt` when available; missing/failed formatter runs are recorded and contained.
- Formatter metadata includes `formatter_status`, `opencode_formatter_source`, formatter command/name/extension matching, status, exit code when available, and BOM resync state.
- File tool results expose `bom`, `bom_preserved`, `bom_strategy`, `formatter_status`, and copied OpenCode source metadata through WebUI proof.
- Patch flow records post-edit receipts after confirmed edits.
- Session part cards are visible in WebUI proof.
- The event rail displays filesystem, watcher, LSP warmup containment, and LSP diagnostic event-envelope activity.
- Native watcher startup publishes `watcher.started` and records `watcher_native_binding`, `native_filewatcher_active`, backend, ignore/protected paths, and subscribe timeout metadata.
- Native filesystem create/update/delete callbacks publish OpenCode-shaped `watcher.updated` events from `opencode.native_filewatcher` with add/change/unlink event names.
- LSP diagnostics metadata includes `severity_counts`, `diagnostic_count`, `max_per_file`, `report_block`, `report_emitted`, `lsp_client_status`, and copied source paths for `lsp.ts` and `diagnostic.ts`.
- The activity rail has a dedicated OpenCode LSP diagnostics panel: diagnostic totals, diagnostic file count, severity chips, per-event severity chips, and visible `report_block` text.
- The event rail has an OpenCode EventV2Bridge-style status panel showing event count, sequence range, by-type counts, by-source counts, and latest touched files.
- `/api/events/recent` includes the status summary; `/api/events/status` returns it directly.
- Compaction produces an OpenCode-shaped structured summary and preserves recent tail context.
- Compaction emits visible start/end lifecycle events with OpenCode source metadata and event-bus receipts.
- Browser proof script now runs both a real NIM model WebUI prompt and a natural-language WebUI tool-lifecycle prompt, then captures the UI with visible prompt, live ToolPart cards, providerExecuted metadata, EventV2Bridge receipts, and human-readable summary.

## Not done / do not overclaim

- Full provider-executed tool execution remains incomplete; Forge-owned tools explicitly mark `providerExecuted: false` while preserving the copied metadata/delta shape.
- Database-backed OpenCode part IDs remain partial.
- Live language-server process/client diagnostics; current LSP slice copies diagnostic report shape, containment, and visible UI presentation only.
- Full OpenCode formatter catalog/config/runtime is partial; only the contained hook shape and rustfmt `.rs` path are implemented.
- LLM-streamed compaction summaries through NVIDIA NIM.
- Durable EventV2 aggregate replay/storage semantics beyond current JSONL replay.
- AgentPart, RetryPart, StepStartPart, StepFinishPart, and SubtaskPart only when backed by a real Forge behavior path.

## Current highest-priority parity gaps

1. Full provider-side/providerExecuted tool execution through the model/tool stream.
2. Live LSP diagnostics from a running language server process.
3. Full OpenCode formatter catalog/config/runtime beyond rustfmt.
4. OpenCode prompt/system behavior.
5. LLM-backed compaction summary generation.
6. Durable session/message/part persistence beyond current snapshots.
