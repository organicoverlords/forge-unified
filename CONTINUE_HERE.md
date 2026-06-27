# Continue Here — Forge Unified

Updated: 2026-06-27

## Start here

1. Read `PROJECT_STATE.md`.
2. Read `OPENCODE-PARITY.md`.
3. Read `FEATURE-AUDIT.md`.
4. Verify repo, branch, HEAD, PR state, and latest CI / Build Proof / Live WebUI Feature Sprint before editing.

## Current branch and PR

- Repo: `organicoverlords/forge-unified`
- PR branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, base `master`
- Accepted proof HEAD before this docs-only continuation update: `7a1951ec9b706bca9a3dea3d7204fff1e01f87cf`.
- Accepted workflows at that head: CI `28301576019`, Build Proof `28301575992`, Live WebUI Feature Sprint `28301576020`.
- Accepted artifact: `7928099674`, digest `sha256:ecee63563b695fbbc62e270dae86ca9347c202ca191a0eb0845da3144503d078`.
- If this file has a newer docs-only HEAD, check Actions before calling the new HEAD green; do not lose the accepted proof above.

## Latest accepted proof slice

Forge now has real live browser proof for the full six-phase benchmark prompt through the WebUI.

The accepted artifact proves:

- Full benchmark prompt submitted through `/api/conversations/:id/chat/stream`.
- Real `nvidia_nim` provider and model `deepseek-ai/deepseek-v4-flash`.
- No local/scripted acceptance path: no `provider: local`, no `local_shortcut`, no `benchmark-phase`.
- Real `tool-call` and `tool-result` events.
- 28 tool calls and 28 tool results in the inspected proof.
- Browser-visible full benchmark markers: `Full six-phase agentic benchmark prompt`, `Phase 1`, `Phase 2`, `Founder report`, and `Technical report`.
- Proof files include `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `tool-lifecycle-webui.png`, `webui.png`, and `event-rail.png`.

## Recent implementation fixes

- Provider-facing tool schema now exposes repo/shell/search/file tools needed by the full benchmark.
- Full benchmark fixture lives at `scripts/smoke/full-agentic-benchmark-prompt.txt`.
- Live WebUI smoke gates the full benchmark prompt and fails on local/scripted paths.
- File read/delete accept schema-shaped `{path: ...}` arguments.
- Failed tool executions are recorded back to the model as tool results.
- `/` and empty paths normalize to workspace root for repo-scoped file tools.
- When tool rounds are exhausted, the orchestrator uses a clean no-tools finalization pass from a compact evidence digest so the WebUI can show final Founder/Technical reports.

## Source-backed OpenCode anchors

- `packages/opencode/src/session/processor.ts` — lifecycle, ToolPart mutation, `providerExecuted`, and provider/tool stream processing.
- `packages/schema/src/v1/session.ts` — ToolPart, ToolState, FilePart schema shape.
- `packages/opencode/src/event-v2-bridge.ts` — visible EventV2Bridge receipt stream.
- `packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — provider-visible tool catalog behavior anchors.

## Previous proven slices

- `7a1951ec9b706bca9a3dea3d7204fff1e01f87cf` — real full six-phase benchmark prompt through WebUI; CI `28301576019`, Build Proof `28301575992`, Live WebUI Feature Sprint `28301576020`; artifact `7928099674`.
- `332b5bbf98c4faaa481fe8a63cd64bb2b1359f92` — live NIM model proof plus visible OpenCode ToolPart lifecycle proof; artifact `7927706533`.
- `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed` — live model-backed browser proof; artifact `7927453969`.
- `04d35a5085a89658b158b7ee23f40510d9a949cd` — older local deterministic six-phase screenshot path. Do not use this as live model acceptance evidence.
- `e562d783538b884b16558b8a62c4e495423f02b3` — formatter proof path repaired; artifact `7926326967`.
- `86fca8e036937f7531ddbf3d09df299119adcc81` — formatter hook metadata and contained formatter execution; artifact `7925827340`.

## Still incomplete / do not overclaim

- True providerExecuted tool calls from provider-side execution are still incomplete for Forge-owned tools.
- OpenCode database-backed part IDs are not fully copied.
- Live LSP server/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.

## Next source-backed targets

1. Check Actions for any docs-only HEAD after this continuation update.
2. Continue toward true providerExecuted tool calls from the NIM/provider stream.
3. Continue live LSP diagnostics.
4. Continue full formatter registry/config/runtime parity.
5. Continue deeper watcher parity or NIM-backed compaction.
