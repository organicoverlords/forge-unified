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
- Selected because it is the newest active open PR and latest meaningful app work.
- Latest fully green baseline before this run: `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed`.
- Latest proven browser proof before this run: Live WebUI Feature Sprint run `28299351117`, artifact `7927453969`.
- Current branch HEAD after this slice needs Actions before calling it green.

## Latest source-backed slice

Forge now exposes and renders the provider-visible OpenCode-style tool catalog so the model/tool stream can see the real tool surface and browser proof can verify it in the UI.

Upstream source paths:

- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/session/processor.ts`
- `packages/schema/src/v1/session.ts`
- `packages/opencode/src/tool/write.ts`
- `packages/opencode/src/tool/edit.ts`
- `packages/opencode/src/tool/read.ts`
- `packages/opencode/src/tool/bash.ts`
- `packages/opencode/src/tool/glob.ts`
- `packages/opencode/src/tool/grep.ts`
- `packages/opencode/src/tool/ls.ts`
- `packages/opencode/src/tool/webfetch.ts`

Copied / improved behavior:

- `tool_definitions()` now advertises the full Forge executor surface instead of only a minimal repo/file/shell subset.
- `apply_patch` is provider-visible with `patchText` schema and OpenCode source metadata.
- `task`, `batch_parallel`, `web_fetch`, `web_search`, `browser_proof`, `vision_review`, `graph_build`, `graph_query`, `terminal_run`, and `switch_mode` are provider-visible.
- `/api/tools` exposes `opencode_provider_tool_catalog`, names, count, required tools, source paths, and full schemas.
- The WebUI renders an `OpenCode Tool Catalog` panel with visible `apply_patch`.
- Live WebUI smoke requires the catalog API and browser DOM to show the provider-visible catalog and `apply_patch` during natural prompt proof.

Forge files touched:

- `crates/engine/src/tool.rs`
- `crates/webui/src/routes.rs`
- `crates/webui/src/lib.rs`
- `crates/webui/src/chat_ui.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`
- `CONTINUE_HERE.md`

Still incomplete / do not overclaim:

- Current HEAD is not yet workflow/browser-proof green.
- Provider-visible catalog is now exposed, but true providerExecuted tool calls from the NIM/provider stream still need deeper implementation.
- OpenCode database-backed part IDs are not fully copied.
- Live LSP server/client diagnostics are not implemented yet.
- Full OpenCode formatter catalog/config/runtime remains partial.
- Full NIM-backed streamed compaction remains incomplete.

## Previous proven slices

- `8da0b7cf6e29c1e63d50042ec00523d4c198e1ed` — live model-backed browser proof; CI `28299351121`, Build Proof `28299351118`, and Live WebUI Feature Sprint `28299351117` were green with proof artifact `7927453969`.
- `04d35a5085a89658b158b7ee23f40510d9a949cd` — six-phase natural WebUI repo benchmark path; CI `28297659041`, Build Proof `28297659050`, and Live WebUI Feature Sprint `28297659029` were green with proof artifact `7926961624`.
- `e562d783538b884b16558b8a62c4e495423f02b3` — formatter proof path repaired; CI `28295482729`, Build Proof `28295482721`, and Live WebUI Feature Sprint `28295482726` were green with proof artifact `7926326967`.
- `86fca8e036937f7531ddbf3d09df299119adcc81` — formatter hook metadata and contained formatter execution; CI `28293770704`, Build Proof `28293770703`, and Live WebUI Feature Sprint `28293770706` were green with proof artifact `7925827340`.
- `d2ecc6a4e9ca89a05fb7d8551b9a1b1c938bf114` — OpenCode FileMutation BOM preservation.
- `2680e673645ced1a799b3a5053885b11996301e0` — OpenCode LSP diagnostic report shape.
- `c3b826d7136298c7bb7d62ba30e11fd12cfeff70` — watcher status + local mutable ToolPart proof path.
- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — ToolStateCompleted FilePart attachment parity.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.

## Next source-backed targets

1. Check Actions for the current provider tool catalog HEAD.
2. If Rust compile/test fails, inspect exact logs; do not rerun deterministic failures blindly.
3. If WebUI smoke fails, inspect `tool-catalog.json`, browser-proof DOM markers, `tool-lifecycle-stream.sse`, and `server.log` first.
4. Inspect proof artifact screenshots after green.
5. Continue toward true providerExecuted tool calls from NIM/provider stream, live LSP server/client diagnostics, full formatter catalog/config, deeper watcher parity, or NIM-backed compaction.
