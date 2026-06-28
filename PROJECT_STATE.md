# Forge Unified — Current State

Updated: 2026-06-28

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current repair HEAD: pending workflow proof after `5b518136581646a8a8e54fb4a863f30a67006e11`
- Previous accepted proof HEAD: `25c7a993b0b7be230f9ad26cc123a153ef95e505`
- Previous same-head workflows: CI `28302865160`, Build Proof `28302865166`, Live WebUI Feature Sprint `28302865162` all green for that older accepted proof head.
- Previous accepted proof artifact: Live WebUI Feature Sprint artifact `7928488316`, digest `sha256:0bb285fe270c03f58dc228090c56eb97fb18e7e96ba34dfffa2268419b7f2e1b`.
- Latest failed inspected HEAD before this update: `a7d081a666da2359b86a040c4856569dc6822687`; same-head Live WebUI Feature Sprint `28315640506`, Build Proof `28315640508`, and CI `28315640515` failed.
- Latest failure diagnosis: Live WebUI job `83888237891` compiled `forge-app`, then `scripts/smoke/live-webui-feature-sprint.sh` failed with `line 272: unexpected EOF while looking for matching '"'`, so no current-head WebUI/NIM browser screenshot or checker artifacts were produced.
- Latest repair: `scripts/smoke/live-webui-feature-sprint.sh` now replaces the fragile final Python/printf status tail with a quote-safe `write_status()` shell function and plain `echo` success lines. Conversation creation now uses Python-generated JSON with `curl --data-binary` instead of hand-built shell JSON.
- Latest proof doc: `docs/generated/proof/live-webui-proof-shell-tail-rewrite-20260628T0850Z.md`.
- Latest parity slice retained: `crates/engine/src/orchestrator.rs` annotates provider-selected successful file/patch tool results with OpenCode `toolResultOutput` / `completeToolCall` style normalized attachment metadata.

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
- Added OpenCode `toolResultOutput` / `completeToolCall` style normalized attachment metadata for successful file/patch tool results: `attachments`, `opencode_normalized_attachments`, and `opencode_tool_attachments_source`.
- Repaired the live proof harness startup path so workflow artifacts include the exact launched command and useful server logs when readiness fails.
- Repaired the live proof harness shell marker/predicate parsing after failed parser runs and hardened conversation/model extraction.
- Repaired the live proof harness again after a line-127 parser failure by replacing fragile inline JQ predicates with Python assertions and safer marker checks.
- Repaired the live proof harness final proof-status writer after line-239, line-249, line-270, and line-272 unmatched-quote failures by replacing embedded newline string literals with a quote-safe `write_status()` function and plain `echo` lines.

## OpenCode source anchors retained

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, `providerExecuted`, same-call ToolPart update semantics, `completeToolCall`, `failToolCall`, `toolResultOutput`, normalized `attachments`, `DOOM_LOOP_THRESHOLD`, recent ToolPart repeated-call comparison, `permission.ask({ permission: "doom_loop", ... })`, and interrupted tool cleanup metadata.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — `partBase`, ToolPart / ToolState / FilePart schema shape.
- `anomalyco/opencode:packages/schema/src/session-id.ts` — `SessionID` prefix semantics.
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts` — EventV2Bridge receipt behavior.
- `anomalyco/opencode:packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — tool catalog behavior anchors.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI tool prompt renders live ToolPart lifecycle cards with provider metadata.
- File-change and EventV2Bridge receipts are visible in chat.
- Normal file tools emit OpenCode-style file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
- Repeated identical tool-call batches are interrupted after three rounds to avoid infinite loops while preserving an explicit OpenCode source marker and permission-envelope metadata.
- Native watcher publishes `watcher.started` and live `watcher.updated` events.
- LSP diagnostic envelopes and report blocks are visible in the event rail, but live language-server collection remains incomplete.
- Conversation compaction emits OpenCode `session.next.compaction.started` and `session.next.compaction.ended` receipts.

## Current gaps / do not overclaim

- Latest HEAD does not yet have same-head green workflow/browser-proof artifact in this chat.
- The new attachment envelope is schema/metadata parity only; it does not implement OpenCode image resizing or database-backed FilePart persistence.
- The doom-loop guard now has a permission-envelope record, but it does not yet implement interactive allow/deny recovery.
- Full provider-side OpenCode processor semantics need more proof beyond metadata propagation.
- Live language-server process/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.
