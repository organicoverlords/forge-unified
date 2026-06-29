# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest selected head before this slice: `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`
- Latest accepted same-head proof baseline before this slice: `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`.
- Same-head workflow state before this slice: CI `28396533513` success; Build Proof `28396533470` success; Live WebUI Feature Sprint `28396533488` success; Live WebUI artifact `7962177971`.
- Latest source-backed slice: OpenCode search/glob human-output proof trail retained in `scripts/smoke/check-opencode-search-glob-contract.py` and `docs/generated/proof/opencode-search-glob-human-output-proof-trail-20260629T1947Z.md`.
- Latest proof doc: `docs/generated/proof/opencode-search-glob-human-output-proof-trail-20260629T1947Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on that exact head.

## Latest workflow state inspected

- Head: `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`.
- PR #3: open, non-draft, mergeable in PR metadata.
- CI `28396533513`: success.
- Build Proof `28396533470`: success.
- Live WebUI Feature Sprint `28396533488`: success.
- Live WebUI proof artifact: `live-webui-feature-sprint-proof`, artifact ID `7962177971`, digest `sha256:3075ddbafc7baed84ac93480c19b5dac730657210375745e4d3529febea46e14`.
- Downloaded proof confirms provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, quality score `95.24`, 36 tool-call events, 30 tool-result events, full benchmark checker passed, workflow checker passed, manifest passed, natural feature-build proof passed, and browser screenshots present.

## Latest implementation changes

- `scripts/smoke/check-opencode-search-glob-contract.py` now requires the search/glob proof trail to retain OpenCode source anchors plus human-readable output, bounded output, path resolution, result count metadata, and `No files found` behavior.
- `docs/generated/proof/opencode-search-glob-human-output-proof-trail-20260629T1947Z.md` records the exact upstream OpenCode source paths and the current implementation boundary.
- This is a source-backed proof-retention gate, not a complete runtime parity claim. The next Actions run must prove the new head.

## Search/glob contract evidence retained for CI

- OpenCode source backing: `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `anomalyco/opencode:packages/opencode/src/tool/grep.ts`.
- Required behavior tokens: path resolution, result count metadata, `No files found`, bounded output, human-readable output, and grep/glob proof trail retention.
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
- `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `packages/opencode/src/tool/grep.ts` — search/glob path resolution, result count metadata, bounded output, human-readable output, and `No files found` behavior.
- `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts` — concise final turn summary carrying model/status metadata.
- `anomalyco/opencode:packages/opencode/src/session/prompt.ts` — prompt/session path for file references and delegated prompt operations.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI prompts must render live ToolPart lifecycle cards with provider metadata.
- File-change and event receipts are visible in chat.
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
- The Live WebUI Feature Sprint workflow also requires a dedicated natural feature-build prompt artifact under `natural-feature-work/`.
