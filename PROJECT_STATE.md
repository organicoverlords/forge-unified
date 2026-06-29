# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest source-fix head before this state update: `15233c6466a8f9cec24055485dff83fd6a9fcc90`
- Latest accepted same-head proof before this final-report gate slice: `8c20dbcc317b51ab69f16beeaf621cebaad939d6`
- Accepted same-head workflows for that baseline: CI `28356929367`, Build Proof `28356929398`, Live WebUI Feature Sprint `28356929402`.
- Accepted Live WebUI artifact for that baseline: `7945828859`, `live-webui-feature-sprint-proof`, digest `sha256:14420500e647c221a08c4c1873ded70797b1a5a8f3ec74a8d5806f1b45fec79f`.
- Accepted Build Proof artifact for that baseline: `7945709891`, `build-proof`, digest `sha256:cab73986015524f5256b56d6767b4ae86d338deefe461dbc355d4a1e720aa9dc`.
- Latest source-backed slice: OpenCode-backed final report template CI contract gate.
- Latest proof doc: `docs/generated/proof/final-report-template-contract-ci-gate-20260629T1358Z.md`.
- Do not claim this latest head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on `15233c6466a8f9cec24055485dff83fd6a9fcc90` or a later head containing the fix.

## Latest failed live run inspected

Latest same-head status before this final-report contract gate:

- Head: `6d020ea0458273046eca089db654d336718db9c3`.
- Build Proof `28373524856`: success.
- CI `28373524848`: success.
- Live WebUI Feature Sprint `28373524809`: failure.
- Live job `84061799401` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- The benchmark reached and passed Phase 1 repo evidence, Phase 2 long-tool-loop evidence, Phase 3 file write/read/delete evidence, Phase 4 real low-risk edit evidence, and Phase 4 validation-command evidence.
- Remaining failed checks were final-answer/report quality checks: confidence labels, risk/rollback wording, Founder report, Technical report, and final report files/tests/risks/confidence sections.

Previous same-head status before the fuzzy file edit slice:

- Head: `83b6ba30bb064d2f0aa92bb44beb3d75d69db3d8`.
- Build Proof `28371596991`: success.
- CI `28371597011`: success.
- Live WebUI Feature Sprint `28371596978`: failure.
- Live job `84050445860` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- The live stream used real `nvidia_nim` / `deepseek-ai/deepseek-v4-flash` and reached long-running tool activity before timeout.
- The failure path exposed stale `file_edit` recovery followed by extra read/write work; Forge still used exact-only replacement while upstream OpenCode uses a layered edit replacement pipeline.

## Accepted live full benchmark proof

Forge has accepted real browser proof for the full six-phase agentic benchmark prompt through the WebUI on `8c20dbcc317b51ab69f16beeaf621cebaad939d6`.

Proof requirements satisfied by artifact `7945828859`:

- The full benchmark prompt is sent through `/api/conversations/:id/chat/stream` and the WebUI proof helper.
- The proof rejects local/scripted paths: no `provider: local`, no truthy `local_shortcut`, no `benchmark-phase`.
- The run uses real `nvidia_nim` with model recorded in conversation/stream artifacts.
- The full benchmark stream contains real `tool-call` and `tool-result` events.
- Browser proof includes `Full six-phase agentic benchmark prompt`, `Phase 1`, `Phase 2`, `Founder report`, and `Technical report`.
- Artifact includes `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `full-benchmark-checker.json`, `opencode-workflow-checker.json`, `tool-lifecycle-webui.png`, `webui.png`, and `event-rail.png`.

## Latest implementation changes

- Added `scripts/smoke/check-final-report-template-contract.py`.
- Wired the final-report contract gate into `.github/workflows/ci.yml` smoke validation.
- The gate enforces that Forge max-step finalization keeps the exact final Markdown labels required by the live benchmark.
- The gate enforces that the forced finalizer disables tools with `tools: None` and `tool_choice: None`.
- The gate enforces that malformed or empty provider final text is rejected by `looks_like_final_report` and replaced by deterministic `fallback_final_report` Markdown.
- Existing fuzzy file-edit behavior remains: exact replacement first, then OpenCode-backed line-trimmed, whitespace-normalized, indentation-flexible, and trimmed-boundary matching with conservative uniqueness.

## OpenCode source anchors retained in developer docs only

- `anomalyco/opencode:packages/opencode/src/tool/edit.ts` — `replace`, `SimpleReplacer`, `LineTrimmedReplacer`, `WhitespaceNormalizedReplacer`, `IndentationFlexibleReplacer`, `TrimmedBoundaryReplacer`, and conservative ambiguous/not-found edit errors.
- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, provider-executed state, same-call ToolPart update semantics, complete/fail tool-call handling, tool-result output, normalized attachments, repeated-call comparison, and interrupted tool cleanup metadata.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — part base, ToolPart, ToolState, and FilePart schema shape.
- `anomalyco/opencode:packages/schema/src/session-id.ts` — SessionID prefix semantics.
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts` — EventV2Bridge receipt behavior.
- `anomalyco/opencode:packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — tool catalog behavior anchors.
- `anomalyco/opencode:packages/opencode/src/lsp/lsp.ts` and `packages/opencode/src/lsp/diagnostic.ts` — LSP touch, diagnostic collection, max error cap, report block, severity formatting, and warm-up containment anchors.
- `anomalyco/opencode:packages/core/src/file-mutation.ts` — BOM-preserving file mutation behavior anchor.
- `anomalyco/opencode:packages/opencode/src/format/index.ts` — formatter service, extension matching, command probing/caching, contained formatter execution, status shape, and configuration/dependency-aware formatter activation.
- `anomalyco/opencode:packages/opencode/src/format/formatter.ts` — built-in formatter catalog, representative extensions, command semantics, and config/dependency-aware formatter enablement.
- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts` — max-step no-tools finalization, text-only summary, remaining work list, next-step recommendations, and evidence-bound command claims.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI tool prompt renders live ToolPart lifecycle cards with provider metadata.
- File-change and EventV2Bridge receipts are visible in chat.
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
