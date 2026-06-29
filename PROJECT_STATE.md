# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest selected head before this slice: `5b5e97c42a0c6b2daff1b23cfadf0360b8b7dc97`
- Latest accepted same-head proof baseline before this slice: `5b5e97c42a0c6b2daff1b23cfadf0360b8b7dc97`.
- Same-head workflow state before this slice: CI `28391729746` success; Build Proof `28391729720` success; Live WebUI Feature Sprint `28391729702` success; Live WebUI artifact `7960159157`.
- Latest source-backed slice: OpenCode search/glob contract evidence gate in `scripts/smoke/check-opencode-search-glob-contract.py`.
- Latest proof doc: `docs/generated/proof/opencode-search-glob-contract-gate-20260629T1850Z.md`.
- Do not claim the latest head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on the latest branch head containing this slice.

## Latest workflow state inspected

- Head: `5b5e97c42a0c6b2daff1b23cfadf0360b8b7dc97`.
- PR #3: open, non-draft, mergeable in PR metadata.
- CI `28391729746`: success.
- Build Proof `28391729720`: success.
- Live WebUI Feature Sprint `28391729702`: success.
- Live WebUI proof artifact: `live-webui-feature-sprint-proof`, artifact ID `7960159157`, digest `sha256:5ee681bf905142972ad7af4677af9c427f345bad04937b2eb06c50838e3972b5`.

## Latest implementation changes

- Added `scripts/smoke/check-opencode-search-glob-contract.py` so CI can deterministically guard OpenCode-backed search/glob evidence without re-running the expensive provider benchmark inside CI.
- The gate validates that Forge keeps source-level `execute_file_glob` / `execute_file_search` coverage and that durable proof/state files record the OpenCode source paths and behavior tokens.
- This is not a parity claim by itself; same-head Live WebUI/NVIDIA NIM proof is still required after the new head is pushed.

## Search/glob contract evidence retained for CI

- OpenCode source backing: `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `anomalyco/opencode:packages/opencode/src/tool/grep.ts`.
- Required behavior tokens: path resolution, result count metadata, `No files found`, bounded result behavior, and grep/glob proof trail retention.
- Forge source path under guard: `crates/engine/src/tool/file_ops.rs`.

## Formatter activation evidence retained for CI

- The formatter proof trail must explicitly mention configuration/dependency-aware formatter activation.
- The formatter proof trail must preserve evidence for formatter service, extension matching, command probing/caching, contained formatter execution, status shape, and configuration/dependency-aware formatter activation.
- The formatter proof trail must preserve evidence for built-in formatter catalog, representative extensions, command semantics, and config/dependency-aware formatter enablement.

## OpenCode source anchors retained in developer docs only

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts` — max-step no-tools finalization, text-only summary, remaining work list, next-step recommendations, and evidence-bound command claims.
- `anomalyco/opencode:packages/opencode/src/tool/edit.ts` — conservative file edit replacement behavior.
- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, provider-executed state, same-call ToolPart update semantics, complete/fail tool-call handling, and tool-result output.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — part base, ToolPart, ToolState, and FilePart schema shape.
- `anomalyco/opencode:packages/opencode/src/format/index.ts` and `packages/opencode/src/format/formatter.ts` — formatter catalog and activation behavior.
- `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `packages/opencode/src/tool/grep.ts` — search/glob path resolution, result count metadata, bounded output, and `No files found` behavior.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI prompts must render live ToolPart lifecycle cards with provider metadata.
- File-change and event receipts are visible in chat.
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
