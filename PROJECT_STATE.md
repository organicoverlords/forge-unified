# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest selected head before this slice: `5831bb38ed3fe3db4007aa87fa801dda2112bf3f`
- Latest accepted same-head proof baseline before this slice: `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`.
- Same-head accepted baseline: CI `28396533513` success; Build Proof `28396533470` success; Live WebUI Feature Sprint `28396533488` success; Live WebUI artifact `7962177971`.
- Latest UX/proof slice: readable WebUI proof cards and final proof summary in `crates/webui/src/chat_ui.rs`, gated by `scripts/smoke/capture-browser-proof.sh`.
- Latest proof doc: `docs/generated/proof/readable-webui-proof-cards-20260629T1955Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on that exact head.

## User rejection that drove this slice

- Previous tool cards exposed raw tool names as primary UI and were unintuitive.
- The full benchmark final screenshot mostly showed the benchmark prompt, not proof or the final answer.
- Process screenshots looked messy and too diagnostic-heavy for normal review.

## Latest workflow state inspected

- Accepted baseline head: `10a0f1d67c99bae0242faf4cff210fe61f5c62c0`.
- PR #3: open, non-draft, mergeable in PR metadata.
- CI `28396533513`: success.
- Build Proof `28396533470`: success.
- Live WebUI Feature Sprint `28396533488`: success.
- Live WebUI proof artifact: `live-webui-feature-sprint-proof`, artifact ID `7962177971`, digest `sha256:3075ddbafc7baed84ac93480c19b5dac730657210375745e4d3529febea46e14`.
- Downloaded proof confirmed provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, quality score `95.24`, 36 tool-call events, 30 tool-result events, full benchmark checker passed, workflow checker passed, manifest passed, natural feature-build proof passed, and browser screenshots present.

## Latest implementation changes

- `crates/webui/src/chat_ui.rs` now maps raw tool names to human labels such as `Read file`, `Write file`, `Edit file`, `Run command`, `Run tools in parallel`, and `Delegate subtask`.
- Tool cards now show a clear status badge, concise result, file chips, and collapsed technical details instead of leading with raw JSON/tool names.
- `proof=final` now renders a top `Run proof summary` panel with provider, model, tool count, actions used, files touched/inspected, and the final answer.
- `scripts/smoke/capture-browser-proof.sh` now gates readable proof markers: `Run proof summary`, `Final answer`, `actions used`, `proof-digest-visible`, and `human-tool-label`.
- This is a UI/proof-presentation repair, not a claim that model/tool execution semantics changed.

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
- Final proof screenshots must show a readable proof digest and final answer, not only the original benchmark prompt.
