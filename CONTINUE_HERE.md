# Continue Here — Forge Unified

Updated: 2026-06-29

## Start here

1. Read `PROJECT_STATE.md`.
2. Read `OPENCODE-PARITY.md`.
3. Read `FEATURE-AUDIT.md`.
4. Verify repo, branch, HEAD, PR state, and latest CI / Build Proof / Live WebUI Feature Sprint before editing.
5. Before any OpenCode-parity planning or patching, inspect the matching upstream OpenCode source code first. Record the exact upstream path used in `OPENCODE-PARITY.md` or the generated proof note for the change.
6. Do not invent Forge-only behavior when the user asked to copy OpenCode behavior.

## Current branch and PR

- Repo: `organicoverlords/forge-unified`
- PR branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, base `master`
- Latest accepted fully green proof HEAD: `74a5b5aa8836075fd187c2da404f25ac14c83229`.
- Accepted workflows at that head: CI `28340198388`, Build Proof `28340198385`, Live WebUI Feature Sprint `28340198381`.
- Accepted artifact: `7939934286`, digest `sha256:5a1fbb5801736071e85a8ea15ba7828adea5f1568c05c391684cfe3ab9d0cab4`.
- Later screenshot-capture work moved HEAD to `ed308b67a10a63392e00c693a99bdc08e66e8d05`; CI and Build Proof passed there, but strict Live WebUI proof failed because the model repeated a stale exact `file_edit` replacement instead of repairing with current-file/apply-patch semantics.
- Do not call any later head accepted until CI, Build Proof, and Live WebUI Feature Sprint are all green on that same head.

## Current instruction from user

Stop patching benchmark prompts as the main fix. Fix Forge code to behave like OpenCode. Before touching code for any OpenCode-parity behavior, inspect the upstream OpenCode source first.

Immediate OpenCode source-backed target:

- Forge stale `file_edit` failure handling should copy OpenCode-style tool failure lifecycle behavior.
- Upstream source already inspected: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, especially `failToolCall`, `completeToolCall`, and ToolPart state transitions.
- Relevant semantics: failed tool calls are first-class tool parts with `state.status = "error"`, original input, explicit error text, and call settlement; the agent should get enough state to recover with a different operation instead of repeating the same failed edit.

## Latest accepted proof slice

Forge has real live browser proof for the full six-phase benchmark prompt through the WebUI at `74a5b5aa8836075fd187c2da404f25ac14c83229`.

The accepted artifact proves:

- Full benchmark prompt submitted through `/api/conversations/:id/chat/stream`.
- Real `nvidia_nim` provider and model `deepseek-ai/deepseek-v4-flash`.
- No local/scripted acceptance path: no `provider: local`, no `local_shortcut`, no `benchmark-phase`.
- Real `tool-call` and `tool-result` events.
- Browser and artifact proof files include `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `tool-lifecycle-webui.png`, `webui.png`, and `event-rail.png`.
- `full-benchmark-checker.json` passed with no failed checks.
- `opencode-workflow-checker.json` passed with no failed checks.

## Recent implementation fixes

- Provider-facing tool schema exposes repo/shell/search/file tools needed by the full benchmark.
- Full benchmark fixture lives at `scripts/smoke/full-agentic-benchmark-prompt.txt`.
- Live WebUI smoke gates the full benchmark prompt and fails on local/scripted paths.
- File read/delete accept schema-shaped `{path: ...}` arguments.
- Failed tool executions are recorded back to the model as tool results.
- `/` and empty paths normalize to workspace root for repo-scoped file tools.
- When tool rounds are exhausted, the orchestrator uses a clean no-tools finalization pass from a compact evidence digest so the WebUI can show final Founder/Technical reports.

## Source-backed OpenCode anchors

- `packages/opencode/src/session/processor.ts` — lifecycle, ToolPart mutation, failed/completed ToolPart state, `providerExecuted`, and provider/tool stream processing.
- `packages/schema/src/v1/session.ts` — ToolPart, ToolState, FilePart schema shape.
- `packages/opencode/src/event-v2-bridge.ts` — visible EventV2Bridge receipt stream.
- `packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — provider-visible tool catalog behavior anchors.

## Previous proven slices

- `74a5b5aa8836075fd187c2da404f25ac14c83229` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7939934286`.
- `7a1951ec9b706bca9a3dea3d7204fff1e01f87cf` — earlier real full six-phase benchmark prompt through WebUI; artifact `7928099674`.
- `332b5bbf98c4faaa481fe8a63cd64bb2b1359f92` — live NIM model proof plus visible OpenCode ToolPart lifecycle proof; artifact `7927706533`.
- `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed` — live model-backed browser proof; artifact `7927453969`.
- `04d35a5085a89658b158b7ee23f40510d9a949cd` — older local deterministic six-phase screenshot path. Do not use this as live model acceptance evidence.

## Still incomplete / do not overclaim

- Stale exact `file_edit` replacement recovery is not OpenCode-grade yet.
- True providerExecuted tool calls from provider-side execution are still incomplete for Forge-owned tools.
- OpenCode database-backed part IDs are not fully copied.
- Live LSP server/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.

## Next source-backed targets

1. Patch Forge stale edit failure behavior using upstream `packages/opencode/src/session/processor.ts` semantics already inspected.
2. Check same-head Actions after the patch.
3. Continue live LSP diagnostics.
4. Continue full formatter registry/config/runtime parity.
5. Continue deeper watcher parity or NIM-backed compaction.
