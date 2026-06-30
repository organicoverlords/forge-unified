# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current selected baseline before this slice: `a9be8be583ac1d5c81b675659fc413c6d1d07bbe`
- Latest same-head green proof before this slice: all required workflows passed on `484189c945d7b0ec90a70300ef960e868ed9a477`.
- Latest implementation/proof slice: browser session turn receipt grouping in `crates/webui/src/chat_ui_enhancements.html`, loaded by `crates/webui/src/chat_ui.rs`, with deterministic WebUI and Fast WebUI proof gates updated.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_enhancements.html` as a reviewable browser enhancement bundle.
- Updated `crates/webui/src/chat_ui.rs` to load both browser UI files with `concat!(include_str!(...))`.
- Added visible per-turn receipt grouping: `Receipts grouped by turn`, file receipt count, tool card count, and status chip count.
- Added Session timeline actions: `copy timeline`, `retry latest prompt`, and `copy latest files`.
- Added per-turn receipt actions: `copy receipts` and `copy files`.
- Updated `scripts/smoke/check-webui-proof-part-contract.py` to validate the enhancement bundle and proof trail tokens.
- Updated `scripts/smoke/fast-webui-proof.sh` to require the new static and browser-captured proof markers.
- Added proof doc `docs/generated/proof/session-turn-receipt-groups-20260630T0058Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, and `stable-session-receipts`.
- This slice improves browser proof readability. It does not claim full parity, production readiness, or backend fork/revert/session checkpoint semantics.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, and `crates/webui/src/chat_ui_enhancements.html`.

### Durable tool lifecycle contract

- Source anchors: `packages/opencode/src/session/processor.ts`, `packages/schema/src/v1/session.ts`, `packages/web/src/components/share/part.tsx`.
- Required trail tokens: pending tool part, running tool part, started lifecycle parts, completed/error finished dispatch, same `callID` across request/result-derived parts, completed output metadata, error metadata, duration timing, and file attachments for file-changing tools.
- Forge implementation path under guard: `crates/engine/src/tool_parts.rs`.

### Search and glob contract

- Source anchors: `packages/opencode/src/tool/glob.ts`, `packages/opencode/src/tool/grep.ts`.
- Required trail tokens: path resolution, result count metadata, `No files found`, bounded output, human-readable output, and grep/glob proof trail retention.
- Forge implementation path under guard: `crates/engine/src/tool/file_ops.rs`.

### Formatter activation contract

- Source anchors: `packages/opencode/src/format/index.ts`, `packages/opencode/src/format/formatter.ts`.
- Required trail tokens: configuration/dependency-aware formatter activation, formatter service, extension matching, command probing/caching, contained formatter execution, status shape, built-in formatter catalog, representative extensions, command semantics, and config/dependency-aware formatter enablement.

### Natural WebUI finished-stream transport contract

- Source anchors: `packages/core/src/session/runner/max-steps.ts`, `packages/opencode/src/cli/cmd/run/turn-summary.ts`, `packages/opencode/src/session/processor.ts`.
- Required trail tokens: recorded run completion, downstream browser/tool/evidence gates, `event: run-finish`, and transport noise handling after completed runs.

## Current head note

- Latest pushed head after the duplicate state refresh: `29795fa7978e43ee410857f16128c6080db3e094`.
- Same-head workflows are required and must be inspected before acceptance.
