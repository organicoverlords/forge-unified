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
- Latest fully green baseline before this slice: `04d35a5085a89658b158b7ee23f40510d9a949cd`.
- Latest proven browser proof before this slice: Live WebUI Feature Sprint run `28297659029`, artifact `7926961624`.
- Current branch HEAD after this slice needs Actions before calling it green.

## Latest source-backed slice

Forge now starts an OS-backed native workspace watcher and publishes real watcher updates, instead of only using receipt-backed watcher metadata from tool results.

Upstream source paths:

- `packages/core/src/filesystem/watcher.ts`
- `packages/opencode/src/event-v2-bridge.ts`
- `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts`

Copied behavior:

- OpenCode detects watcher backend names (`windows`, `fs-events`, `inotify`).
- OpenCode keeps `SUBSCRIBE_TIMEOUT_MS = 10_000` as watcher subscription metadata.
- OpenCode subscribes to workspace changes and publishes `Watcher.Event.Updated` for create/update/delete.
- Forge maps native create/update/delete to add/change/unlink and publishes `watcher.updated` from `opencode.native_filewatcher`.
- Forge exposes active native watcher status through `/api/events/status`: `watcher_native_binding`, `native_filewatcher_active`, backend, ignore/protected paths, and copied source path.
- The live WebUI proof now creates, edits, and deletes `native-watch-proof.txt` and requires the native watcher event in `/api/events/recent` and browser-captured event rail proof.

Forge files touched:

- `Cargo.toml`
- `crates/engine/Cargo.toml`
- `crates/engine/src/change_bus.rs`
- `crates/engine/src/tool.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`
- `CONTINUE_HERE.md`

Still incomplete / do not overclaim:

- Current HEAD is not yet workflow/browser-proof green.
- Native watcher config is partial: exact OpenCode config-driven ignore entries, protected path service, VCS-directory watch behavior, and scoped finalizer semantics are not fully copied.
- Live LSP server/client diagnostics are not implemented yet; current LSP slice improves source-backed visibility of the copied diagnostic envelope.
- Full OpenCode formatter catalog/config/runtime remains partial; only rustfmt `.rs` path is wired.
- OpenCode database-backed part IDs are not fully copied.
- `providerExecuted` delta updates are still partial.
- Full NIM-backed streamed compaction remains incomplete.

## Previous proven slices

- `04d35a5085a89658b158b7ee23f40510d9a949cd` — six-phase natural WebUI repo benchmark path; CI `28297659041`, Build Proof `28297659050`, and Live WebUI Feature Sprint `28297659029` were green with proof artifact `7926961624`.
- `e562d783538b884b16558b8a62c4e495423f02b3` — formatter proof path repaired; CI `28295482729`, Build Proof `28295482721`, and Live WebUI Feature Sprint `28295482726` were green with proof artifact `7926326967`.
- `86fca8e036937f7531ddbf3d09df299119adcc81` — formatter hook metadata and contained formatter execution; CI `28293770704`, Build Proof `28293770703`, and Live WebUI Feature Sprint `28293770706` were green with proof artifact `7925827340`.
- `d2ecc6a4e9ca89a05fb7d8551b9a1b1c938bf114` — OpenCode FileMutation BOM preservation; CI `28293331161`, Build Proof `28293331148`, and Live WebUI Feature Sprint `28293331147` were green.
- `2680e673645ced1a799b3a5053885b11996301e0` — OpenCode LSP diagnostic report shape; CI `28292308520`, Build Proof `28292308511`, and Live WebUI Feature Sprint `28292308525` were green with proof artifact `7925391830`.
- `c3b826d7136298c7bb7d62ba30e11fd12cfeff70` — watcher status + local mutable ToolPart proof path; CI `28291374005`, Build Proof `28291373988`, and Live WebUI Feature Sprint `28291373990` were green with proof artifact `7925108696`.
- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity; CI, Build Proof, and Live WebUI Feature Sprint were green with proof artifact `7924965603`.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — ToolStateCompleted FilePart attachment parity.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.

## Next source-backed targets

1. Check Actions for the current native watcher HEAD.
2. If Rust compile/test fails, inspect exact logs; do not rerun deterministic failures blindly.
3. If WebUI smoke fails, inspect proof artifact, native watcher event status, event rail DOM, and `server.log` first.
4. Inspect proof artifact screenshots after green.
5. Continue toward live LSP server/client diagnostics, full formatter catalog/config, deeper watcher parity, or NIM-backed compaction.
