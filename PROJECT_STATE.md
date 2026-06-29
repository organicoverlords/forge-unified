# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest pre-slice verified head: `59f9d4a71625d0dfe7125df9c816b8f47930fce5`
- Latest accepted same-head proof before manifest-gate follow-ups: `59f9d4a71625d0dfe7125df9c816b8f47930fce5`
- Accepted same-head workflows for that baseline: CI `28382878597`, Build Proof `28382878610`, Live WebUI Feature Sprint `28382878593`.
- Accepted Live WebUI artifact for that baseline: `7956715745`, `live-webui-feature-sprint-proof`, digest `sha256:5ce3895a333ba27b5a1ddc09c07b01587a9f0fe1c76d72d6cbc587abadc3f5f9`.
- Accepted Build Proof artifact for that baseline: `7956402464`, `build-proof`, digest `sha256:c5381969921f8c71f6a18b56ec9280630ab6e1d06a5c2845bc5d0845282ce64b`.
- Latest source-backed slice: Live WebUI proof manifest marker validation now aggregates browser proof JSON, conversation JSON, and SSE stream text instead of requiring all final-report markers to appear only in the browser-proof JSON file.
- Latest proof doc: `docs/generated/proof/live-webui-manifest-aggregate-evidence-20260629T1614Z.md`.
- Do not claim the latest head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on `76e999a094bacc4f1709b51781e11d3a5930f743` or a later head containing this gate.

## Latest verified live state

Latest same-head status before the manifest-gate follow-up slices:

- Head: `59f9d4a71625d0dfe7125df9c816b8f47930fce5`.
- Build Proof `28382878610`: success.
- CI `28382878597`: success.
- Live WebUI Feature Sprint `28382878593`: success.
- Live artifact `7956715745`: `live-webui-feature-sprint-proof`, digest `sha256:5ce3895a333ba27b5a1ddc09c07b01587a9f0fe1c76d72d6cbc587abadc3f5f9`.
- Build artifact `7956402464`: `build-proof`, digest `sha256:c5381969921f8c71f6a18b56ec9280630ab6e1d06a5c2845bc5d0845282ce64b`.

## Previous failed live runs inspected

Latest failed live manifest run inspected before this slice:

- Head: `7f60f6cedc4bb7c7b0a131637adcb74eee65287f`.
- CI `28384854521`: success.
- Build Proof `28384854694`: success.
- Live WebUI Feature Sprint `28384854653`: failure.
- Live job `84096737028` failed in `Check full benchmark evidence and quality score`; the live WebUI sprint step itself succeeded.
- Live artifact `7957561694`: `live-webui-feature-sprint-proof`, digest `sha256:f37d708474d4b99a07098a319ab26820e515cb6ff99df2fb3fd525c15aad2f4c`.
- OpenCode source inspected before patching: `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Source-backed diagnosis: max-step finalization requires a text-only final response summarizing work, remaining work, and next steps. Browser-visible evidence for that final response can be split across browser DOM proof, conversation JSON, and SSE stream artifacts; the manifest gate should validate the combined uploaded proof bundle, not only one JSON file.

Previous same-head status before the quality-gate slice:

- Head: `65fee6348197ee973af21809e97f9d1cc5cb966e`.
- Build Proof `28381036315`: success.
- CI `28381036299`: success.
- Live WebUI Feature Sprint `28381036286`: failure.
- Live artifact `7955936640`: `live-webui-feature-sprint-proof`, digest `sha256:59d7889e98b26db786ccd9261c0b6d4fe9144270595cc1dea0a2e08838a32c4f`.
- Live job `84083393327` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- The checked script already treats `Founder report` and `Technical report` as case-insensitive in browser proof, but the quality scorer used exact case-sensitive final-report label matching.
- The inspected OpenCode source requires text-only finalization with a summary, remaining tasks, and recommendations; it does not require exact capitalization of report headings.

Previous same-head status before the finalization-stop slice:

- Head: `9ad3db25c029c97f0c36b575add4ddebbe06b033`.
- Build Proof `28378470751`: success.
- CI `28378470831`: success.
- Live WebUI Feature Sprint `28378470554`: failure.
- Live job `84074310768` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- The stream used real `nvidia_nim` with model `deepseek-ai/deepseek-v4-flash` and reached 28 tool-call/tool-result events.
- The benchmark passed Phase 1 repo evidence, Phase 2 long-tool-loop evidence, Phase 3 file write/read/delete evidence, Phase 4 real low-risk edit evidence, Phase 4 validation-command evidence, and cleanup/state evidence.
- Remaining failed checks were final-answer/report quality checks: confidence labels, risk/rollback wording, Founder report, Technical report, and final report files/tests/risks/confidence sections.
- The observed failure mode was extra tool-loop continuation after the evidence-completing validation command instead of immediate text-only final report.

Previous same-head status before the final-report contract gate:

- Head: `6d020ea0458273046eca089db654d336718db9c3`.
- Build Proof `28373524856`: success.
- CI `28373524848`: success.
- Live WebUI Feature Sprint `28373524809`: failure.
- Live job `84061799401` failed in `Run live WebUI feature sprint` and `Check full benchmark evidence and quality score`.
- The benchmark reached and passed Phase 1 repo evidence, Phase 2 long-tool-loop evidence, Phase 3 file write/read/delete evidence, Phase 4 real low-risk edit evidence, and Phase 4 validation-command evidence.
- Remaining failed checks were final-answer/report quality checks: confidence labels, risk/rollback wording, Founder report, Technical report, and final report files/tests/risks/confidence sections.

## Accepted live full benchmark proof

Forge has accepted real browser proof for the full six-phase agentic benchmark prompt through the WebUI on `59f9d4a71625d0dfe7125df9c816b8f47930fce5`.

Proof requirements satisfied by artifact `7956715745`:

- The full benchmark prompt is sent through `/api/conversations/:id/chat/stream` and the WebUI proof helper.
- The proof rejects local/scripted paths: no `provider: local`, no truthy `local_shortcut`, no `benchmark-phase`.
- The run uses real `nvidia_nim` with model recorded in conversation/stream artifacts.
- The full benchmark stream contains real `tool-call` and `tool-result` events.
- Browser proof includes `Full six-phase agentic benchmark prompt`, `Phase 1`, `Phase 2`, `Founder report`, and `Technical report`.
- Artifact includes `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `full-benchmark-checker.json`, `opencode-workflow-checker.json`, `quality-score.json`, `tool-lifecycle-webui.png`, `webui.png`, and `event-rail.png`.

## Latest implementation changes

- Updated `scripts/smoke/check-live-webui-proof-manifest.py` so final-report/browser marker validation is case-insensitive and uses the aggregate browser proof JSON + conversation JSON + SSE stream proof bundle.
- Kept the manifest gate strict on non-empty screenshot proof, hard checker pass, OpenCode workflow checker pass, quality score, NVIDIA NIM provider/model evidence, tool evidence, status paths, and local-shortcut rejection.
- Updated `.github/workflows/live-webui-feature-sprint.yml` so the Live WebUI workflow runs the manifest gate after the hard checker, OpenCode workflow checker, and quality score.
- Retained `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts` as the source anchor for no-tools text finalization behavior and browser-visible final-report proof requirements.
- Existing final-report contract gate remains: `scripts/smoke/check-final-report-template-contract.py` in CI.
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
