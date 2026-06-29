# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest selected head before this slice: `9410a9d66806ec8f5fd14c0096f4410946864e35`
- Latest accepted same-head proof baseline before this slice: `9410a9d66806ec8f5fd14c0096f4410946864e35`.
- Same-head workflow state before this slice: CI `28395411258` success; Build Proof `28395411263` success; Live WebUI Feature Sprint `28395411267` success; Live WebUI artifact `7961800514`.
- Latest source-backed slice: natural WebUI feature-build proof gate in `scripts/smoke/natural-feature-work.sh` and `.github/workflows/live-webui-feature-sprint.yml`.
- Latest proof doc: `docs/generated/proof/webui-natural-feature-build-gate-20260629T1915Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on that exact head.

## Latest workflow state inspected

- Head: `9410a9d66806ec8f5fd14c0096f4410946864e35`.
- PR #3: open, non-draft, mergeable in PR metadata.
- CI `28395411258`: success.
- Build Proof `28395411263`: success.
- Live WebUI Feature Sprint `28395411267`: success.
- Live WebUI proof artifact: `live-webui-feature-sprint-proof`, artifact ID `7961800514`, digest `sha256:41d8c4196a53c2c4516782f2bb590478210d018bed8d697de1e7c5e1aa1ab734`.
- Downloaded proof confirms provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, quality score `100.0`, 28 tool-call events, 28 tool-result events, full benchmark checker passed, workflow checker passed, manifest passed, and browser screenshots present.

## Latest implementation changes

- `scripts/smoke/natural-feature-work.sh` now asks Forge to run a normal natural-language WebUI feature-build prompt instead of only proposing a next slice.
- The natural feature-build prompt requires file inspection, a repo-local edit, a generated proof note, validation command output, browser proof, and a final human-readable summary with files/tests/risks/confidence.
- `.github/workflows/live-webui-feature-sprint.yml` now runs this prompt after the benchmark proof and requires `forge-proof/live-webui-feature-sprint/natural-feature-work/summary.json` plus screenshot output before the workflow can be green.
- This is a proof-gate feature. The next Actions run must prove the new head.

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
- `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts` — concise final turn summary carrying model/status metadata.
- `anomalyco/opencode:packages/opencode/src/session/prompt.ts` — prompt/session path for file references and delegated prompt operations.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI prompts must render live ToolPart lifecycle cards with provider metadata.
- File-change and event receipts are visible in chat.
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
- The Live WebUI Feature Sprint workflow now also requires a dedicated natural feature-build prompt artifact under `natural-feature-work/`.
