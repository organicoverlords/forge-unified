# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest selected head before this slice: `17ab8228c2c1e7291f4847d18d1963115cecd981`
- Latest accepted same-head proof baseline before this slice: `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`.
- Same-head accepted baseline: CI `28396533513` success; Build Proof `28396533470` success; Live WebUI Feature Sprint `28396533488` success; Live WebUI artifact `7962177971`.
- Latest verified same-head workflow set before this slice: CI `28404596368` success; Build Proof `28404596364` success; App Build Proof `28404596398` success; Fast WebUI Proof `28404596399` success; Live WebUI Feature Sprint `28404596424` still in progress at inspection time.
- Latest proof slice: WebUI proof part contract CI gate in `scripts/smoke/check-webui-proof-part-contract.py`.
- Latest proof doc: `docs/generated/proof/webui-proof-part-contract-gate-20260629T2158Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI / Build Proof / Live WebUI Feature Sprint / Fast WebUI Proof complete on that exact head.

## User rejection that drove this slice

- Previous tool cards exposed raw tool names as primary UI and were unintuitive.
- The full benchmark final screenshot mostly showed the benchmark prompt, not proof or the final answer.
- Process screenshots looked messy and too diagnostic-heavy for normal review.
- Fast WebUI Proof previously proved the DOM could still contain raw implementation text/JSON in final proof mode.

## Latest workflow state inspected

- Current selected head before this slice: `17ab8228c2c1e7291f4847d18d1963115cecd981`.
- PR #3: open, non-draft, mergeable in PR metadata.
- CI `28404596368`: success.
- Build Proof `28404596364`: success.
- App Build Proof `28404596398`: success.
- Fast WebUI Proof `28404596399`: success.
- Live WebUI Feature Sprint `28404596424`: in progress at inspection time, job `84164044719`, step `Run natural feature-build prompt through WebUI`.

## Latest implementation changes

- `scripts/smoke/check-webui-proof-part-contract.py` now enforces a WebUI proof-part presentation contract backed by upstream OpenCode share-part source paths.
- `.github/workflows/ci.yml` now compiles and runs that gate in CI smoke validation.
- The guard keeps final proof screenshots focused on readable proof digest, final answer, provider/model route, human-readable tool labels, and OpenCode-style live/completed tool parts.
- The guard rejects primary `raw tool:` marker regressions and requires technical details to remain collapsed outside `proof=final` and absent from final proof DOM.
- This is a CI/proof-presentation parity guard, not a claim that complete UI parity or complete OpenCode parity is done.

## Search/glob contract evidence retained for CI

- OpenCode source backing: `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `anomalyco/opencode:packages/opencode/src/tool/grep.ts`.
- Required behavior tokens: path resolution, result count metadata, `No files found`, bounded output, human-readable output, and grep/glob proof trail retention.
- Forge source path under guard: `crates/engine/src/tool/file_ops.rs`.

## Formatter activation evidence retained for CI

- The formatter proof trail must explicitly mention configuration/dependency-aware formatter activation.
- The formatter proof trail must preserve evidence for formatter service, extension matching, command probing/caching, contained formatter execution, status shape, and configuration/dependency-aware formatter activation.
- The formatter proof trail must preserve evidence for built-in formatter catalog, representative extensions, command semantics, and config/dependency-aware formatter enablement.

## WebUI proof part contract evidence retained for CI

- OpenCode source backing: `anomalyco/opencode:packages/web/src/components/share/part.tsx` and `anomalyco/opencode:packages/web/src/components/share/part.module.css`.
- Forge source path under guard: `crates/webui/src/chat_ui.rs`.
- Required behavior tokens: `proof-final`, `proof-digest-visible`, `final-answer-visible`, `provider-model-visible`, `human-tool-label`, `opencode-tool-result-card`, `opencode-live-toolpart`, human label rendering, and collapsed technical details outside final proof mode.

## OpenCode source anchors retained in developer docs only

- `anomalyco/opencode:packages/web/src/components/share/part.tsx` — user-facing componentized tool rendering for completed tool parts.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css` — shared visual treatment for parts/tool UI.
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
- Final proof screenshots must show a readable proof digest and final answer, not only the original benchmark prompt.
