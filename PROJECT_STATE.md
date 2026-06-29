# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest pre-slice selected head: `adcca81b32ac9d6968d1804077f53c36b22e5c41`
- Latest accepted same-head proof baseline: `59f9d4a71625d0dfe7125df9c816b8f47930fce5`
- Accepted same-head workflows for that baseline: CI `28382878597`, Build Proof `28382878610`, Live WebUI Feature Sprint `28382878593`.
- Accepted Live WebUI artifact for that baseline: `7956715745`, `live-webui-feature-sprint-proof`, digest `sha256:5ce3895a333ba27b5a1ddc09c07b01587a9f0fe1c76d72d6cbc587abadc3f5f9`.
- Accepted Build Proof artifact for that baseline: `7956402464`, `build-proof`, digest `sha256:c5381969921f8c71f6a18b56ec9280630ab6e1d06a5c2845bc5d0845282ce64b`.
- Latest source-backed slice: Live WebUI final-report streaming budget is now OpenCode-backed and CI-gated.
- Latest proof doc: `docs/generated/proof/live-webui-finalization-time-budget-20260629T1655Z.md`.
- Do not claim the latest head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on the latest branch head containing this slice.

## Latest failed live run inspected

- Head: `adcca81b32ac9d6968d1804077f53c36b22e5c41`.
- CI `28386515572`: success.
- Build Proof `28386515636`: success.
- Live WebUI Feature Sprint `28386515646`: failure.
- Live job `84102450955` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- The run used real `nvidia_nim` with model `deepseek-ai/deepseek-v4-flash`.
- The hard workflow checker passed with no failed checks.
- The full benchmark checker saw 25 tool-call events, 21 tool-result events, and 36 tool results.
- The failure was final-report timeout/quality completion: the stream was still writing the final Markdown report when the 660 second budget expired.

## Latest implementation changes

- Added `scripts/smoke/check-live-webui-time-budget.py`.
- Updated `.github/workflows/live-webui-feature-sprint.yml` so `FORGE_BENCH_TIMEOUT_SECONDS` is `840`.
- Updated `.github/workflows/ci.yml` so smoke validation compiles and runs the new time-budget gate.
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
