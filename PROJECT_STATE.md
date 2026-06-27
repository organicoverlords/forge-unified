# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest fully green baseline before this slice: `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed`.
- Latest browser proof artifact before this slice: Live WebUI Feature Sprint run `28299351117`, artifact `7927453969`.
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
- `04d35a5085a89658b158b7ee23f40510d9a949cd` — six-phase natural WebUI repo benchmark path; CI `28297659041`, Build Proof `28297659050`, and Live WebUI Feature Sprint `28297659029` were green with proof artifact `7926961624`.
- `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed` — live model-backed browser proof on current head; CI `28299351121`, Build Proof `28299351118`, and Live WebUI Feature Sprint `28299351117` were green with proof artifact `7927453969`.
- This slice — visible OpenCode ToolPart lifecycle rail: live WebUI cards for pending/input/running/completed tool states, `providerExecuted` metadata shape, file-change receipts, EventV2Bridge rows, and screenshot proof requirements.

## OpenCode source references for latest slice

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — source for `ensureToolCall`, `updateToolCall`, `completeToolCall`, `failToolCall`, same-row ToolPart updates by call ID, `providerExecuted` preservation, and `DOOM_LOOP_THRESHOLD = 3` guard shape.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — source for ToolPart, ToolStatePending, ToolStateRunning, ToolStateCompleted, ToolStateError, and ToolStateCompleted attachments.
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts` — source for publishing UI-visible event receipts.

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
- The WebUI now renders those lifecycle states as live ToolPart cards during the stream, with visible `providerExecuted` metadata and OpenCode source metadata behind details.
- Conversation storage mutates the previous assistant message's matching ToolPart row from running to completed/error and records `opencode_mutable_tool_part_source`.
- The change bus replays the latest persisted `.forge/change-events.jsonl` events on startup and continues sequence numbers after replay.
- The tool executor starts a native filesystem watcher for the workspace and keeps its subscription alive while the app is running.
- Native watcher status exposes backend, active binding, ignore/protected paths, subscribe timeout, and copied OpenCode watcher source path through `/api/events/status`.
- Native create/update/delete callbacks publish `watcher.updated` events from `opencode.native_filewatcher`, mapped to add/change/unlink.
- `/api/events/status`, `/api/events/recent`, and the activity rail expose OpenCode-style bridge status including durability metadata.
- Conversation compaction stores a structured summary and recent tail, then publishes exact OpenCode `session.next.compaction.started` / `session.next.compaction.ended` event receipts.
- Existing session part cards remain: TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, PatchPart.
- Live WebUI smoke now requires a real NIM model proof and then a natural WebUI tool-lifecycle proof screenshot with visible prompt, live ToolPart cards, `providerExecuted`, EventV2Bridge receipts, and human-readable summary.

## Current gaps

- Current HEAD is not yet workflow/browser-proof green.
- Full provider-side/providerExecuted tool execution is not implemented; Forge-owned tools explicitly mark `providerExecuted: false` while preserving the OpenCode metadata/delta shape.
- Native watcher exists, but deeper OpenCode parity remains: dynamic config-driven watcher ignore entries, separate VCS-directory watch behavior, and exact scoped finalizer semantics are still partial.
- LSP diagnostic report shape and UI are copied, but diagnostics are not yet collected from a live language server process.
- Formatter hook shape is implemented, but full OpenCode formatter catalog/config/runtime remains partial.
- Compaction summary creation is deterministic in Forge; full streamed NIM-backed compaction summary remains incomplete.

## Next targets

1. Check latest Actions for this branch HEAD and fix exact failures.
2. Inspect browser proof artifacts and screenshot DOM for visible ToolPart lifecycle cards and real NIM model proof.
3. Continue toward full provider-side tool execution, live LSP server/client diagnostics, full formatter catalog/config, deeper watcher parity, or NIM-backed compaction from OpenCode sources.
