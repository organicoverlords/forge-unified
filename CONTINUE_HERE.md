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
- Latest fully green baseline before this slice: `2680e673645ced1a799b3a5053885b11996301e0`.
- Latest proven browser proof artifact before this slice: `7925391830` from Live WebUI Feature Sprint run `28292308525`.
- Current branch HEAD after this slice needs Actions before calling it green.

## Latest OpenCode-source slice

Forge now copies OpenCode's UTF-8 BOM preservation behavior for file write/edit tools.

Upstream source paths:

- `packages/core/src/file-mutation.ts`
- `packages/opencode/src/tool/write.ts`
- `packages/opencode/src/tool/edit.ts`

Copied behavior:

- OpenCode `FileMutation.writeTextPreservingBom` strips duplicate leading BOMs from new content.
- It preserves an existing UTF-8 BOM when a target already has one.
- It also preserves an input BOM when a new text payload starts with one.
- It writes at most one UTF-8 BOM.
- Forge `file_write` and `file_edit` now follow that behavior and record `bom`, `bom_preserved`, `bom_strategy`, and source metadata in ToolResult metadata.
- The natural WebUI file-tool proof now requires those markers in the SSE stream and conversation JSON.

Forge files touched:

- `crates/engine/src/tool/file_ops.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`
- `CONTINUE_HERE.md`

Still incomplete / do not overclaim:

- Formatter hooks after write/edit are still incomplete.
- Live LSP server/client diagnostics are not implemented yet.
- OpenCode's database-backed part IDs are not fully copied.
- `providerExecuted` delta updates are still partial.
- Live OS filesystem watcher integration remains receipt-backed.
- Full NIM-backed streamed compaction remains incomplete.

## Previous proven slices

- `2680e673645ced1a799b3a5053885b11996301e0` — OpenCode LSP diagnostic report shape; CI `28292308520`, Build Proof `28292308511`, and Live WebUI Feature Sprint `28292308525` were green with proof artifact `7925391830`.
- `c3b826d7136298c7bb7d62ba30e11fd12cfeff70` — watcher status + local mutable ToolPart proof path; CI `28291374005`, Build Proof `28291373988`, and Live WebUI Feature Sprint `28291373990` were green with proof artifact `7925108696`.
- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity; CI, Build Proof, and Live WebUI Feature Sprint were green with proof artifact `7924965603`.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — ToolStateCompleted FilePart attachment parity.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.

## Next source-backed targets

1. Check Actions for the current BOM preservation HEAD.
2. If Rust compile/test fails, inspect the exact job logs; do not rerun deterministic failures blindly.
3. If WebUI smoke fails on port `3320`, inspect `server.log` first.
4. Inspect proof artifact screenshots and DOM after green.
5. Continue toward formatter hooks, real LSP server/client diagnostics, OS-backed watcher/file edited events, or NIM-backed compaction.
