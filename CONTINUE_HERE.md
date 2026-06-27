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
- Latest fully green baseline before this slice: `d052a279d7a5c37b275043ad0e52fb966a0be4eb`.
- Latest proven browser proof artifact before this slice: `7924965603` from Live WebUI Feature Sprint run `28290889903`.
- Current branch HEAD after this slice needs Actions before calling it green.

## Latest OpenCode-source slice

Forge now copies more of OpenCode SessionProcessor's mutable ToolPart behavior.

Upstream source paths:

- `packages/opencode/src/session/processor.ts`
- `packages/schema/src/v1/session.ts`

Copied behavior:

- OpenCode `readToolCall` locates the existing ToolPart for a `callID`.
- OpenCode `updateToolCall` writes changes back into the same part row.
- OpenCode `completeToolCall` replaces the running state with a completed state, preserving the input and adding output, title, metadata, time, and optional attachments.
- OpenCode `failToolCall` replaces the running state with an error state.
- Forge now updates the prior assistant message's `tool_parts` row for the matching `callID` when tool results are recorded.
- Forge records `mutable_tool_part_updates` receipts with `before_status`, `after_status`, and `opencode_mutable_tool_part_source`.
- The natural WebUI proof now requires these receipts in conversation JSON.

Forge files touched:

- `crates/engine/src/conversation.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`
- `CONTINUE_HERE.md`

Still incomplete / do not overclaim:

- OpenCode's database-backed part IDs are not fully copied.
- `providerExecuted` delta updates are still partial.
- Live OS filesystem watcher integration remains receipt-backed.
- Live language-server diagnostics remain event-envelope/touch receipts, not a running LSP service.
- BOM and formatter parity remain incomplete.
- Full NIM-backed streamed compaction remains incomplete.

## Previous proven slices

- `d052a279d7a5c37b275043ad0e52fb966a0be4eb` — OpenCode SessionProcessor lifecycle stream parity; CI, Build Proof, and Live WebUI Feature Sprint were green with proof artifact `7924965603`.
- `98b408b0f8f8a132ba7df18617d103ea63d43ce1` — ToolStateCompleted FilePart attachment parity.
- `d24d8e7183216aa8a50627b1bc280251d9171ee4` — OpenCode session compaction event-type parity.
- `1734ae285237bee4c4bd06a418ecd719a1ccf87a` — durable OpenCode EventV2Bridge-style change bus replay.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.
- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.

## Next source-backed targets

1. Check Actions for the current mutable ToolPart HEAD.
2. If Rust compile/test fails, inspect the exact job logs; do not rerun deterministic failures blindly.
3. If WebUI smoke fails on port `3320`, inspect `server.log` first.
4. Inspect proof artifact screenshots and DOM after green.
5. Continue toward real OS-backed watcher/file edited events, live LSP diagnostics, formatter/BOM parity, or NIM-backed compaction.
