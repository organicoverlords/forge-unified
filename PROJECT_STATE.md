# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest pre-slice selected head: `a9517aad7cfa6dfcb6b62794b13529a70f38e5ad`
- Latest pushed head: `2651981b76151ba915c9cc11711aa742afbd7e64`
- Latest accepted same-head proof baseline before this slice: `a9517aad7cfa6dfcb6b62794b13529a70f38e5ad` for Build Proof and Live WebUI only; CI failed before this slice.
- Latest same-head workflow state before this slice: Build Proof `28390284139` success; Live WebUI Feature Sprint `28390284127` success; CI `28390284128` failure in Smoke Test / Validate WebUI proof harness.
- Latest source-backed slice: formatter activation evidence gate now reads both `PROJECT_STATE.md` and `docs/generated/proof/*.md`, so CI checks the durable proof trail instead of requiring all formatter activation evidence phrases to stay duplicated in one state file.
- Latest proof doc: `docs/generated/proof/formatter-activation-proof-trail-gate-20260629T1745Z.md`.
- Do not claim the latest head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on the latest branch head containing this slice.

## Latest failed CI run inspected

- Head: `a9517aad7cfa6dfcb6b62794b13529a70f38e5ad`.
- Build Proof `28390284139`: success.
- Live WebUI Feature Sprint `28390284127`: success.
- CI `28390284128`: failure.
- Failed job: `Smoke Test`, job `84115340620`.
- Failed step: `Validate WebUI proof harness`.
- Failure: `scripts/smoke/check-formatter-activation-evidence.py` reported missing formatter activation evidence phrases in `PROJECT_STATE.md`.
- Classification: proof harness/state consistency failure, not provider failure and not Rust build/test failure.

## Latest implementation changes

- Updated `scripts/smoke/check-formatter-activation-evidence.py` so the formatter activation proof gate scans the durable proof trail (`PROJECT_STATE.md` plus `docs/generated/proof/*.md`) for evidence phrases.
- Retained runtime guard that `crates/engine/src/tool/file_ops.rs` must contain Forge-owned formatter metadata/contracts and must not reintroduce `opencode_*` runtime metadata keys.
- This keeps the OpenCode source backing enforceable while avoiding brittle duplication across state files.

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

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI prompts must render live ToolPart lifecycle cards with provider metadata.
- File-change and event receipts are visible in chat.
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
