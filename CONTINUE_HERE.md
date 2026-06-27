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
- Latest fully green baseline before this slice: `d2ecc6a4e9ca89a05fb7d8551b9a1b1c938bf114`.
- Latest proven browser proof before this slice: Live WebUI Feature Sprint run `28293331147`.
- Current branch HEAD after this slice needs Actions before calling it green.

## Latest OpenCode-source slice

Forge now copies the post-write `Format.file(filepath)` hook shape used by OpenCode write/edit tools.

Upstream source paths:

- `packages/opencode/src/format/index.ts`
- `packages/opencode/src/tool/write.ts`
- `packages/opencode/src/tool/edit.ts`
- `packages/core/src/file-mutation.ts`

Copied behavior:

- OpenCode write/edit call `format.file(filepath)` after writing the file.
- If formatting ran, OpenCode syncs the desired BOM back to the file before publishing watcher/LSP work.
- OpenCode formatter matching is extension-based and returns false when no formatter is available.
- OpenCode contains formatter spawn/nonzero failures rather than failing the whole edit path.
- Forge now has a contained formatter hook for `file_write` and `file_edit`.
- `.rs` files use `rustfmt` when available.
- ToolResult metadata records `formatter_status`, `opencode_formatter_source`, formatter command/name/extension matching, status, exit code when present, and BOM resync state.
- The natural WebUI file-tool proof now uses a temporary `.rs` file and requires formatter markers in the SSE stream and conversation JSON.

Forge files touched:

- `crates/engine/src/tool/file_ops.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`
- `CONTINUE_HERE.md`

Still incomplete / do not overclaim:

- Full OpenCode formatter catalog/config/runtime remains partial; only rustfmt `.rs` path is wired.
- Live LSP server/client diagnostics are not implemented yet.
- OpenCode's database-backed part IDs are not fully copied.
- `providerExecuted` delta updates are still partial.
- Live OS filesystem watcher integration remains receipt-backed.
- Full NIM-backed streamed compaction remains incomplete.

## Previous proven slices

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

1. Check Actions for the current formatter-hook HEAD.
2. If Rust compile/test fails, inspect the exact job logs; do not rerun deterministic failures blindly.
3. If WebUI smoke fails on port `3320`, inspect `server.log` first.
4. Inspect proof artifact screenshots and DOM after green.
5. Continue toward full formatter catalog/config, real LSP server/client diagnostics, OS-backed watcher/file edited events, or NIM-backed compaction.
