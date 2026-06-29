# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest pre-slice selected head: `9e1bab911ff13ec599fe3fc068fa3bc08dacf5c0`
- Latest pushed head: `28c144513952fb9fa3e661865223f32cbed51baa`
- Latest accepted same-head proof baseline: `59f9d4a71625d0dfe7125df9c816b8f47930fce5`
- Accepted same-head workflows for that baseline: CI `28382878597`, Build Proof `28382878610`, Live WebUI Feature Sprint `28382878593`.
- Accepted Live WebUI artifact for that baseline: `7956715745`, `live-webui-feature-sprint-proof`, digest `sha256:5ce3895a333ba27b5a1ddc09c07b01587a9f0fe1c76d72d6cbc587abadc3f5f9`.
- Accepted Build Proof artifact for that baseline: `7956402464`, `build-proof`, digest `sha256:c5381969921f8c71f6a18b56ec9280630ab6e1d06a5c2845bc5d0845282ce64b`.
- Latest source-backed slice: Forge engine finalization evidence now flattens nested `batch_parallel` tool results before deciding the full WebUI benchmark evidence is complete.
- Latest proof doc: `docs/generated/proof/live-webui-nested-batch-finalization-20260629T1716Z.md`.
- Do not claim the latest head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on the latest branch head containing this slice.

## Latest failed live run inspected

- Head: `9e1bab911ff13ec599fe3fc068fa3bc08dacf5c0`.
- Build Proof `28388644197`: success.
- CI `28388644209`: failure in `Smoke Test` / `Validate WebUI proof harness`.
- Live WebUI Feature Sprint `28388644200`: failure.
- Live job `84109733346` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- Live artifact `7959103317`, `live-webui-feature-sprint-proof`, digest `sha256:dd29545f142a0cbec4ae2f2d75df89282c1b728da12f69db3c17ef3978fdea6d`.
- The run used real NIM secrets in the workflow environment and the 840 second finalization budget.
- Failure class remains evidence/finalization alignment, not Build Proof failure.

## Latest implementation changes

- Updated `crates/engine/src/orchestrator.rs` so `benchmark_evidence_ready()` evaluates direct tool results plus nested `batch_parallel` child results.
- Updated `final_evidence_digest()` to include nested `batch_parallel` child results so deterministic final reporting can cite the same evidence the checker accepts.
- Retained strict Live WebUI proof requirements: browser screenshot proof, conversation JSON, SSE stream, hard checker, OpenCode workflow checker, quality score, manifest, NVIDIA NIM provider/model evidence, and tool evidence.

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
