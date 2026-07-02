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
- Latest accepted fully green proof HEAD before the current UI slice: `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`.
- Accepted workflows at that head: CI `28396533513`, Build Proof `28396533470`, Live WebUI Feature Sprint `28396533488`.
- Accepted artifact: `7962177971`, digest `sha256:3075ddbafc7baed84ac93480c19b5dac730657210375745e4d3529febea46e14`.
- Do not call any later head accepted until CI, Build Proof, and Live WebUI Feature Sprint are all green on that same head and screenshots are inspected.

## Current instruction from user

User rejected the last screenshots as bad UX:

- tool cards showed raw tool names and were unintuitive;
- the full benchmark final screenshot showed mostly the benchmark prompt, not proof/final answer;
- the process was messy and visually poor.

## Current UI/proof target

Readable WebUI proof-card repair:

- `crates/webui/src/chat_ui.rs` maps raw tool names to readable labels such as `Read file`, `Write file`, `Run command`, `Run tools in parallel`, and `Delegate subtask`.
- Raw tool IDs remain in collapsed technical metadata and data attributes for checkers.
- Tool cards show status badges, concise result text, and file chips.
- `proof=final` renders a top `Run proof summary` card with provider/model/tool count/actions/files/final answer.
- `scripts/smoke/capture-browser-proof.sh` now fails if readable proof markers are missing.
- Proof doc: `docs/generated/proof/readable-webui-proof-cards-20260629T1955Z.md`.

## Latest accepted proof slice

Forge has same-head green proof for the full six-phase benchmark prompt and natural feature-build prompt through the WebUI at `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`.

The accepted artifact proves:

- Full benchmark prompt submitted through `/api/conversations/:id/chat/stream`.
- Real `nvidia_nim` provider and model `deepseek-ai/deepseek-v4-flash`.
- No local/scripted acceptance path: no `provider: local`, no `local_shortcut`, no `benchmark-phase`.
- 36 `tool-call` and 30 `tool-result` events for the full benchmark.
- 18 `tool-call` and 18 `tool-result` events for the natural feature-build prompt.
- Browser and artifact proof files include `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `tool-lifecycle-webui.png`, `webui.png`, `event-rail.png`, and `natural-feature-work/webui.png`.
- `full-benchmark-checker.json` passed with no failed checks.
- `opencode-workflow-checker.json` passed with no failed checks.
- `quality-score.json` passed at `95.24`.

## Recent implementation fixes

- Provider-facing tool schema exposes repo/shell/search/file tools needed by the full benchmark.
- Full benchmark fixture lives at `scripts/smoke/full-agentic-benchmark-prompt.txt`.
- Live WebUI smoke gates the full benchmark prompt and fails on local/scripted paths.
- File read/delete accept schema-shaped `{path: ...}` arguments.
- Failed tool executions are recorded back to the model as tool results.
- `/` and empty paths normalize to workspace root for repo-scoped file tools.
- When tool rounds are exhausted, the orchestrator uses a clean no-tools finalization pass from a compact evidence digest so the WebUI can show final Founder/Technical reports.
- Stale exact `file_edit` failures now return first-class error metadata with current-file preview and recovery guidance.
- Phase 4 benchmark proof now requires a successful dedicated repository edit result outside `.agent_test` before final reporting.
- Natural WebUI feature-build prompt gate with dedicated proof artifacts under `natural-feature-work/`.
- Readable WebUI proof-card and final-proof summary repair after user screenshot rejection.

## Source-backed OpenCode anchors

- `packages/opencode/src/session/processor.ts` — lifecycle, ToolPart mutation, failed/completed ToolPart state, `providerExecuted`, and provider/tool stream processing.
- `packages/schema/src/v1/session.ts` — ToolPart, ToolState, FilePart schema shape.
- `packages/opencode/src/event-v2-bridge.ts` — visible EventV2Bridge receipt stream.
- `packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — provider-visible tool catalog behavior anchors.
- `packages/opencode/src/format/index.ts` and `packages/opencode/src/format/formatter.ts` — formatter registry, extension matching, command probing, and contained formatter failures.
- `packages/opencode/src/cli/cmd/run/turn-summary.ts` — final turn summary shape with agent/model/duration metadata.
- `packages/core/src/session/runner/max-steps.ts` — max-step no-tools finalization summary requirements.
- `packages/opencode/src/session/prompt.ts` — session prompt path for file-reference resolution and delegated prompt operations.

## Previous proven slices

- `10a0f1d67c99bae0242faf4cff210fe61f5c62c0` — accepted same-head CI, Build Proof, Live WebUI full benchmark proof, and natural feature-build proof; artifact `7962177971`.
- `9410a9d66806ec8f5fd14c0096f4410946864e35` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7961800514`.
- `5b5e97c42a0c6b2daff1b23cfadf0360b8b7dc97` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7960159157`.
- `a49747de437d0f8e54ee5011861329f0423fef3b` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7942306628`.
- `c12789a7b7c59ba7bfe0ba22118892396356fc7c` — accepted same-head stale edit recovery proof; artifact `7941120525`.
- `74a5b5aa8836075fd187c2da404f25ac14c83229` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7939934286`.
- `7a1951ec9b706bca9a3dea3d7204fff1e01f87cf` — earlier real full six-phase benchmark prompt through WebUI; artifact `7928099674`.
- `332b5bbf98c4faaa481fe8a63d50042ec00523d4c198e1ed` — live NIM model proof plus visible ToolPart lifecycle proof; artifact `7927706533`.
- `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed` — live model-backed browser proof; artifact `7927453969`.
- `04d35a5085a89658b158b7ee23f40510d9a949cd` — older local deterministic six-phase screenshot path. Do not use this as live model acceptance evidence.

## Still incomplete / do not overclaim

- True providerExecuted tool calls from provider-side execution are still incomplete for Forge-owned tools.
- OpenCode database-backed part IDs are not fully copied.
- Live LSP server/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.

## Next source-backed targets

1. Check same-head Actions for the readable proof-card head and fix any real failures.
2. Download and inspect the new screenshot artifact; do not accept if the UI still looks bad.
3. Continue formatter registry/config/runtime parity using upstream `packages/opencode/src/format/index.ts` and `packages/opencode/src/format/formatter.ts` semantics already inspected.
4. Continue live LSP diagnostics.
5. Continue deeper watcher parity or NIM-backed compaction.
