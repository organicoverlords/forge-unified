# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Accepted proof HEAD: `7a1951ec9b706bca9a3dea3d7204fff1e01f87cf`
- Same-head workflows: CI `28301576019`, Build Proof `28301575992`, Live WebUI Feature Sprint `28301576020` all green.
- Accepted proof artifact: Live WebUI Feature Sprint artifact `7928099674`, digest `sha256:ecee63563b695fbbc62e270dae86ca9347c202ca191a0eb0845da3144503d078`.

## Accepted live full benchmark proof

Forge now has accepted real browser proof for the full six-phase agentic benchmark prompt through the WebUI.

Proof requirements now satisfied:

- The full benchmark prompt is sent through `/api/conversations/:id/chat/stream`.
- The proof rejects local/scripted paths: no `provider: local`, no `local_shortcut`, no `benchmark-phase`.
- The run uses real `nvidia_nim` with model `deepseek-ai/deepseek-v4-flash`.
- The full benchmark stream contains real `tool-call` and `tool-result` events.
- The inspected proof contains 28 tool calls and 28 tool results.
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

## OpenCode source anchors retained

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, `providerExecuted`, and same-call ToolPart update semantics.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — ToolPart / ToolState / FilePart schema shape.
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts` — EventV2Bridge receipt behavior.
- `anomalyco/opencode:packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — tool catalog behavior anchors.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI tool prompt renders live ToolPart lifecycle cards with `providerExecuted` metadata.
- File-change and EventV2Bridge receipts are visible in chat.
- Normal file tools emit OpenCode-style file/watch/LSP receipts, formatter metadata, BOM metadata, and completed ToolPart attachments.
- Native watcher publishes `watcher.started` and live `watcher.updated` events.
- LSP diagnostic envelopes and report blocks are visible in the event rail, but live language-server collection remains incomplete.
- Conversation compaction emits OpenCode `session.next.compaction.started` and `session.next.compaction.ended` receipts.

## Current gaps / do not overclaim

- Forge-owned tools still mark `providerExecuted: false`; provider-side execution with true provider-executed tool calls remains incomplete.
- Live language-server process/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.
- OpenCode database-backed part IDs remain partial.

## Next targets

1. Continue toward true providerExecuted tool calls from the NIM/provider stream.
2. Continue live LSP diagnostics.
3. Continue full formatter registry/config/runtime parity.
4. Continue deeper watcher parity.
5. Continue NIM-backed compaction summaries.
