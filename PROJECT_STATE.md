# Forge Unified — Current State

Updated: 2026-06-28

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current repair HEAD: pending workflow proof after runtime ToolPart independence cleanup.
- Previous accepted proof HEAD: `25c7a993b0b7be230f9ad26cc123a153ef95e505`
- Previous same-head workflows: CI `28302865160`, Build Proof `28302865166`, Live WebUI Feature Sprint `28302865162` all green for that older accepted proof head.
- Previous accepted proof artifact: Live WebUI Feature Sprint artifact `7928488316`, digest `sha256:0bb285fe270c03f58dc228090c56eb97fb18e7e96ba34dfffa2268419b7f2e1b`.
- Latest failed inspected HEAD before this update: `3af6b810747839229c3f783abc330d02d16fa7a6`; same-head CI `28321804112`, Build Proof `28321804108`, and Live WebUI Feature Sprint `28321804109` failed.
- Latest failure diagnosis: Live WebUI job `83904916420` compiled `forge-app`, started the WebUI, completed the smaller NIM/tool lifecycle stages, then timed out during the full six-phase benchmark stream with `curl: (28) Operation timed out after 1190002 milliseconds with 0 bytes received`; no full benchmark conversation/stream artifact was produced.
- Latest repair: `crates/webui/src/events.rs` now keeps OpenCode source paths and `opencode_*` identifiers out of runtime ToolPart lifecycle/result payloads while preserving Forge-owned ToolPart lifecycle/state semantics.
- Latest proof doc: `docs/generated/proof/runtime-toolpart-independence-cleanup-20260628T1250Z.md`.
- Latest parity slice retained: Forge follows OpenCode behavior references for provider-executed ToolPart lifecycle/state semantics, but source paths remain in developer docs/proof notes rather than provider-visible Forge runtime outputs.

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
- Added provider-executed metadata propagation for provider-selected tool results in the orchestrated model loop.
- Added deterministic `id`, `sessionID`, and `messageID` base fields to generated TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart payloads.
- Added a repeated-tool doom-loop guard in `crates/engine/src/orchestrator.rs` using threshold `3`, with visible interruption text and run metadata.
- Added a structured doom-loop permission envelope: `permission: doom_loop`, `patterns`, `always`, `ruleset`, `input`, and `recent_tool_signatures`.
- Added ToolPart-style result metadata for completed/failed tool states: status, title, call id, timing, output shape, and error fields.
- Added normalized attachment metadata for successful file/patch tool results.
- Repaired the live proof harness startup path so workflow artifacts include the exact launched command and useful server logs when readiness fails.
- Rewrote the live proof harness after repeated unmatched-quote failures so it uses self linting, quote-safe marker loops, Python JSON creation, and plain status output.
- Added provider-visible independence guards to the live proof harness for `/api/tools`.
- Replaced the fragile live proof harness tail with a shorter quote-safe script after the same-head run failed at EOF quote parsing on line 393.
- Removed OpenCode source paths and `opencode_*` metadata keys from WebUI runtime ToolPart lifecycle/result payloads in `crates/webui/src/events.rs`.

## OpenCode source anchors retained in developer docs only

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
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
- Repeated identical tool-call batches are interrupted after three rounds to avoid infinite loops.
- Native watcher publishes `watcher.started` and live `watcher.updated` events.
- LSP diagnostic envelopes and report blocks are visible in the event rail, but live language-server collection remains incomplete.
- Conversation compaction emits compaction started/ended receipts.

## Current gaps / do not overclaim

- Latest HEAD does not yet have same-head green workflow/browser-proof artifact in this chat.
- The full six-phase live benchmark stream currently stalls long enough that the proof harness times out without final benchmark artifacts; next useful source slice is incremental SSE progress while the model/tool loop is running.
- The attachment envelope is schema/metadata parity only; it does not implement image resizing or database-backed FilePart persistence.
- The doom-loop guard now has a permission-envelope record, but it does not yet implement interactive allow/deny recovery.
- Full provider-side processor semantics need more proof beyond metadata propagation.
- Live language-server process/client diagnostics are not implemented yet.
- Full formatter catalog/config/runtime remains partial.