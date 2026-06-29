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
- Latest accepted fully green proof HEAD before the current slice: `9410a9d66806ec8f5fd14c0096f4410946864e35`.
- Accepted workflows at that head: CI `28395411258`, Build Proof `28395411263`, Live WebUI Feature Sprint `28395411267`.
- Accepted artifact: `7961800514`, digest `sha256:41d8c4196a53c2c4516782f2bb590478210d018bed8d697de1e7c5e1aa1ab734`.
- Do not call any later head accepted until CI, Build Proof, and Live WebUI Feature Sprint are all green on that same head.

## Current instruction from user

Continue GitHub Actions work on this branch and build features with the app through WebUI natural-language prompts.

## Current source-backed target

Natural WebUI feature-build proof gate:

- Existing workflow input `prompt` was present, but the old natural feature script only asked for proposal-style work.
- `scripts/smoke/natural-feature-work.sh` now requires a normal WebUI prompt to inspect files, make a repo-local edit, create/update a generated proof note, run syntax validation, capture browser proof, and write pass/fail summary artifacts.
- `.github/workflows/live-webui-feature-sprint.yml` now runs that script after the full benchmark proof and requires `natural-feature-work/summary.json` plus screenshot output before green.

## Latest accepted proof slice

Forge has same-head green proof for the full six-phase benchmark prompt through the WebUI at `9410a9d66806ec8f5fd14c0096f4410946864e35`.

The accepted artifact proves:

- Full benchmark prompt submitted through `/api/conversations/:id/chat/stream`.
- Real `nvidia_nim` provider and model `deepseek-ai/deepseek-v4-flash`.
- No local/scripted acceptance path: no `provider: local`, no `local_shortcut`, no `benchmark-phase`.
- 28 `tool-call` and 28 `tool-result` events.
- Browser and artifact proof files include `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `tool-lifecycle-webui.png`, `webui.png`, and `event-rail.png`.
- `full-benchmark-checker.json` passed with no failed checks.
- `opencode-workflow-checker.json` passed with no failed checks.
- `quality-score.json` passed at `100.0`.

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
- New current slice: natural WebUI feature-build prompt gate with dedicated proof artifacts under `natural-feature-work/`.

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

- `9410a9d66806ec8f5fd14c0096f4410946864e35` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7961800514`.
- `5b5e97c42a0c6b2daff1b23cfadf0360b8b7dc97` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7960159157`.
- `a49747de437d0f8e54ee5011861329f0423fef3b` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7942306628`.
- `c12789a7b7c59ba7bfe0ba22118892396356fc7c` — accepted same-head stale edit recovery proof; artifact `7941120525`.
- `74a5b5aa8836075fd187c2da404f25ac14c83229` — accepted same-head CI, Build Proof, and Live WebUI proof for full benchmark; artifact `7939934286`.
- `7a1951ec9b706bca9a3dea3d7204fff1e01f87cf` — earlier real full six-phase benchmark prompt through WebUI; artifact `7928099674`.
- `332b5bbf98c4faaa481fe8a63cd64bb2b1359f92` — live NIM model proof plus visible ToolPart lifecycle proof; artifact `7927706533`.
- `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed` — live model-backed browser proof; artifact `7927453969`.
- `04d35a5085a89658b158b7ee23f40510d9a949cd` — older local deterministic six-phase screenshot path. Do not use this as live model acceptance evidence.

## Still incomplete / do not overclaim

- True providerExecuted tool calls from provider-side execution are still incomplete for Forge-owned tools.
- OpenCode database-backed part IDs are not fully copied.
- Live LSP server/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.

## Next source-backed targets

1. Check same-head Actions for the new natural WebUI feature-build gate head and fix any real failures.
2. Continue formatter registry/config/runtime parity using upstream `packages/opencode/src/format/index.ts` and `packages/opencode/src/format/formatter.ts` semantics already inspected.
3. Continue live LSP diagnostics.
4. Continue deeper watcher parity or NIM-backed compaction.
