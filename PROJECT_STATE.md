# Forge Unified — Current State

Updated: 2026-06-28

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous accepted proof HEAD: `25c7a993b0b7be230f9ad26cc123a153ef95e505`
- Previous same-head workflows: CI `28302865160`, Build Proof `28302865166`, Live WebUI Feature Sprint `28302865162` all green for that older accepted proof head.
- Previous accepted proof artifact: Live WebUI Feature Sprint artifact `7928488316`, digest `sha256:0bb285fe270c03f58dc228090c56eb97fb18e7e96ba34dfffa2268419b7f2e1b`.
- Latest failed inspected HEAD before this update: `6d5bf28390e9c573d0b6faf05e5093353952c604`; same-head CI `28308137748`, Build Proof `28308137749`, and Live WebUI Feature Sprint `28308137738` failed.
- Latest failure diagnosis: Live WebUI job `83867976719` compiled `forge-app`, then `scripts/smoke/live-webui-feature-sprint.sh` failed with `line 96: syntax error near unexpected token '('`, so no full benchmark conversation/stream artifacts were produced.
- Latest repair: the live WebUI proof harness now uses explicit helper functions for conversation creation/model extraction and adds proof markers for OpenCode-style tool state envelopes.
- Latest parity slice: `crates/engine/src/orchestrator.rs` now annotates provider-selected tool results with OpenCode `completeToolCall` / `failToolCall` style status/title/time/output/error metadata.
- Latest proof doc: `docs/generated/proof/opencode-tool-state-result-envelope-20260628T0250Z.md`.

## Accepted live full benchmark proof

Forge has accepted real browser proof for the full six-phase agentic benchmark prompt through the WebUI on the previous accepted proof head.

Proof requirements satisfied by the older accepted artifact:

- The full benchmark prompt is sent through `/api/conversations/:id/chat/stream` and the WebUI proof helper.
- The proof rejects local/scripted paths: no `provider: local`, no truthy `local_shortcut`, no `benchmark-phase`.
- The run uses real `nvidia_nim` with model `deepseek-ai/deepseek-v4-flash`.
- The full benchmark stream contains real `tool-call` and `tool-result` events.
- The inspected proof contains 35 tool calls and 35 tool results.
- Browser proof includes `Full six-phase agentic benchmark prompt`, `Phase 1`, `Phase 2`, `Founder report`, and `Technical report`.
- Artifact includes `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `tool-lifecycle-webui.png`, `webui.png`, and `event-rail.png`.

## Latest implementation changes

- Exposed repo/shell/search/file tools to the provider-facing tool schema.
- Added `scripts/smoke/full-agentic-benchmark-prompt.txt` as the benchmark fixture.
- Added a full benchmark prompt gate to `scripts/smoke/live-webui-feature-sprint.sh`.
- Accepted schema-shaped file path arguments like `{path: ...}` for read/delete tools.
- Returned failed tool executions to the model as tool results instead of dropping them.
- Normalized `/` and empty file paths to workspace root for repo-scoped file tools.
- Added clean no-tools finalization from a compact evidence digest when the model exhausts tool rounds.
- Added OpenCode-style provider-executed metadata propagation for provider-selected tool results in the orchestrated model loop.
- Added deterministic OpenCode-style `id`, `sessionID`, and `messageID` base fields to generated TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart payloads.
- Added an OpenCode-style repeated-tool doom-loop guard in `crates/engine/src/orchestrator.rs` using threshold `3`, with visible interruption text and run metadata.
- Added a structured OpenCode-style doom-loop permission envelope: `permission: doom_loop`, `patterns`, `always`, `ruleset`, `input`, `recent_tool_signatures`, and upstream source metadata.
- Added OpenCode `completeToolCall` / `failToolCall` style result metadata: `opencode_tool_state_status`, `opencode_tool_state_title`, `opencode_tool_call_id`, `opencode_tool_state_time`, `opencode_tool_state_source`, `opencode_tool_output_shape`, and `opencode_tool_error`.
- Repaired the live proof harness startup path so workflow artifacts include the exact launched command and useful server logs when readiness fails.
- Repaired the live proof harness shell marker/predicate parsing after failed parser runs and hardened conversation/model extraction.

## OpenCode source anchors retained

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, `providerExecuted`, same-call ToolPart update semantics, `completeToolCall`, `failToolCall`, `DOOM_LOOP_THRESHOLD`, recent ToolPart repeated-call comparison, `permission.ask({ permission: "doom_loop", ... })`, and interrupted tool cleanup metadata.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — `partBase`, ToolPart / ToolState / FilePart schema shape.
- `anomalyco/opencode:packages/schema/src/session-id.ts` — `SessionID` prefix semantics.
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts` — EventV2Bridge receipt behavior.
- `anomalyco/opencode:packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — tool catalog behavior anchors.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI tool prompt renders live ToolPart lifecycle cards with provider metadata.
- File-change and EventV2Bridge receipts are visible in chat.
- Normal file tools emit OpenCode-style file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, and now ToolPart-like result state envelopes.
- Repeated identical tool-call batches are interrupted after three rounds to avoid infinite loops while preserving an explicit OpenCode source marker and permission-envelope metadata.
- Native watcher publishes `watcher.started` and live `watcher.updated` events.
- LSP diagnostic envelopes and report blocks are visible in the event rail, but live language-server collection remains incomplete.
- Conversation compaction emits OpenCode `session.next.compaction.started` and `session.next.compaction.ended` receipts.

## Current gaps / do not overclaim

- Latest HEAD does not yet have same-head green workflow/browser-proof artifact in this chat.
- The doom-loop guard now has a permission-envelope record, but it does not yet implement interactive allow/deny recovery.
- Full provider-side OpenCode processor semantics need more proof beyond metadata propagation.
- Live language-server process/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.
- OpenCode database-backed persistence remains partial; current part IDs are deterministic schema-shaped compatibility IDs, not database-backed rows.

## Next targets

1. Prove the latest HEAD with same-head CI, Build Proof, and Live WebUI Feature Sprint.
2. If startup and parser fixes are green, continue interactive OpenCode doom-loop allow/deny recovery UI.
3. Prove provider-selected tool results visibly carry provider metadata, ToolPart-like result state envelopes, and schema-compatible part IDs in WebUI screenshots/DOM.
4. Continue live LSP diagnostics.
5. Continue full formatter registry/config/runtime parity.
6. Continue deeper watcher parity.
7. Continue NIM-backed compaction summaries.
