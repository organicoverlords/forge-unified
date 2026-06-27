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
- Latest fully green baseline before this slice: `c3b826d7136298c7bb7d62ba30e11fd12cfeff70`.
- Latest proven browser proof artifact before this slice: `7925108696` from Live WebUI Feature Sprint run `28291373990`.
- Current branch HEAD after this slice needs Actions before calling it green.

## Latest OpenCode-source slice

Forge now copies more of OpenCode's LSP diagnostic reporting shape for post-edit receipts.

Upstream source paths:

- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/lsp/lsp.ts`
- `packages/opencode/src/lsp/diagnostic.ts`
- `packages/opencode/src/tool/read.ts`

Copied behavior:

- OpenCode apply_patch touches edited documents, collects `lsp.diagnostics()`, and emits `LSP.Diagnostic.report(target, diagnostics[path])` for each edited target with errors.
- OpenCode `Diagnostic.report` caps output at 20 errors per file.
- OpenCode `Diagnostic.pretty` formats one-based `ERROR [line:col] message` strings.
- Forge diagnostics receipts now include `severity_counts`, `diagnostic_count`, `max_per_file`, `report_block`, `report_emitted`, `lsp_client_status`, and copied source metadata.
- Forge still contains missing live LSP clients instead of failing the edit path, matching the safety direction from OpenCode's optional warmup containment path.

Forge files touched:

- `crates/engine/src/tool/patch_events.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`
- `CONTINUE_HERE.md`

Still incomplete / do not overclaim:

- Live LSP server/client diagnostics are not implemented yet.
- OpenCode's database-backed part IDs are not fully copied.
- `providerExecuted` delta updates are still partial.
- Live OS filesystem watcher integration remains receipt-backed.
- BOM and formatter parity remain incomplete.
- Full NIM-backed streamed compaction remains incomplete.

## Previous proven slices

- `c3b826d7136298c7bb7d62ba30e11fd12cfeff70` — watcher status + local mutable ToolPart proof path; CI `28291374005`, Build Proof `28291373988`, and Live WebUI Feature Sprint `28291373990` were green with proof artifact `7925108696`.
- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity; CI, Build Proof, and Live WebUI Feature Sprint were green with proof artifact `7924965603`.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — ToolStateCompleted FilePart attachment parity.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.

## Next source-backed targets

1. Check Actions for the current LSP diagnostic report HEAD.
2. If Rust compile/test fails, inspect the exact job logs; do not rerun deterministic failures blindly.
3. If WebUI smoke fails on port `3320`, inspect `server.log` first.
4. Inspect proof artifact screenshots and DOM after green.
5. Continue toward real LSP server/client diagnostics, OS-backed watcher/file edited events, formatter/BOM parity, or NIM-backed compaction.
