# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current selected baseline before this slice: `208f24329f361e87d82842129ed83b469df36ca5`
- Latest same-head green proof before this slice: all required workflows passed on `208f24329f361e87d82842129ed83b469df36ca5`.
- Latest implementation/proof slice: typed WebUI tool-card renderer in `crates/webui/src/chat_ui_enhancements.html`, with deterministic WebUI proof gate updates in `scripts/smoke/check-webui-proof-part-contract.py`.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added browser-side typed tool-card summaries to `crates/webui/src/chat_ui_enhancements.html`.
- Added visible action, target, and result affordances for common coding-agent tools.
- Added UI proof markers: `typed-tool-renderer`, `tool-target-visible`, `tool-result-toggle`, and `tool-duration-visible`.
- Updated `scripts/smoke/check-webui-proof-part-contract.py` to require the new typed tool-card renderer markers and exact source anchors.
- Added proof doc `docs/generated/proof/webui-typed-tool-card-renderer-20260630T0152Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, `stable-session-receipts`, `typed-tool-renderer`, `tool-target-visible`, and `tool-result-toggle`.
- This slice improves browser proof readability. It does not claim full parity, production readiness, or backend fork/revert/session checkpoint semantics.

## Compatibility proof trail retained for deterministic gates

These exact source anchors and behavior tokens are kept because CI smoke gates read this project state and `docs/generated/proof/*.md` as durable proof memory.

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, and `crates/webui/src/chat_ui_enhancements.html`.

### Durable tool lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `packages/opencode/src/session/processor.ts`, `packages/schema/src/v1/session.ts`, `packages/web/src/components/share/part.tsx`.
- Required trail tokens: tool lifecycle, ToolPart, pending tool part, running tool part, started lifecycle parts, completed/error finished dispatch, same `callID` across request/result-derived parts, completed output metadata, error metadata, duration timing, and file attachments for file-changing tools.
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

- Latest pushed head after the typed tool-card renderer slice: pending same-head workflow completion after the newest commits.
- Same-head workflows are required and must be inspected before acceptance.
