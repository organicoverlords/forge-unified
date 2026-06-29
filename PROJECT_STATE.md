# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest source-fix head: `eefd4d03c8285d1528614c605d669a8c0b809230`
- Latest accepted same-head proof before this quality-gate slice: `8c20dbcc317b51ab69f16beeaf621cebaad939d6`
- Accepted same-head workflows for that baseline: CI `28356929367`, Build Proof `28356929398`, Live WebUI Feature Sprint `28356929402`.
- Accepted Live WebUI artifact for that baseline: `7945828859`, `live-webui-feature-sprint-proof`, digest `sha256:14420500e647c221a08c4c1873ded70797b1a5a8f3ec74a8d5806f1b45fec79f`.
- Accepted Build Proof artifact for that baseline: `7945709891`, `build-proof`, digest `sha256:cab73986015524f5256b56d6767b4ae86d338deefe461dbc355d4a1e720aa9dc`.
- Latest source-backed slice: OpenCode-backed live-quality browser proof marker normalization for Phase 4 repo edits.
- Latest proof doc: `docs/generated/proof/phase4-browser-marker-20260629T1116Z.md`.
- Do not claim this latest head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on `eefd4d03c8285d1528614c605d669a8c0b809230` or a later head containing the fix.

## Latest failed live run inspected

Latest failed same-head gate before the source fix:

- Head: `117a0ebe9b11c84a760190a472c02cac05f1869b`.
- Build Proof `28367497968`: success.
- CI `28367497919`: failure.
- Live WebUI Feature Sprint `28367497952`: failure.
- CI failed in `Smoke Test` / `Validate WebUI proof harness`.
- Live WebUI Feature Sprint failed in `live-webui-feature-sprint` and `Check full benchmark evidence and quality score`.
- Live artifact metadata inspected: `7950149603`, `live-webui-feature-sprint-proof`, digest `sha256:f7d1bce77f63d38d6adb07bdc037ed27a5baee23799f1ccd3f7a8fdd91f247bd`.
- Source mismatch found: the quality scorer's browser usefulness gate required the literal `apply_patch` marker even though the Phase 4 evidence gate accepts a successful `FileEdit`, `FileWrite`, or `ApplyPatch` outside `.agent_test` plus status/diff command evidence.

Previous failed same-head gate before command-claim normalization:

- Head: `4ff7c2f6f7cfc758c3669fe59d5407163ccf70b1`.
- Build Proof `28364990761`: success.
- CI `28364990747`: failure.
- Live WebUI Feature Sprint `28364990741`: failure.
- CI failed in `Smoke Test` / `Validate WebUI proof harness`: `max-step finalization parity check failed: fallback report is conservative`.
- Live WebUI app execution used real `nvidia_nim` / `deepseek-ai/deepseek-v4-flash`; hard checker and OpenCode workflow checker passed with 40 tool-call events and 38 tool-result events.
- Live WebUI quality score failed on `test_and_build_claims_match_tool_commands` because human-readable validation claims were not normalized back to exact successful tool command metadata, plus a lower-weight bracket semantic check was too broad.

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

- Exposed repo/shell/search/file tools to the provider-facing tool schema.
- Added `scripts/smoke/full-agentic-benchmark-prompt.txt` as the benchmark fixture.
- Added a full benchmark prompt gate to `scripts/smoke/live-webui-feature-sprint.sh`.
- Accepted schema-shaped file path arguments like `{path: ...}` for read/delete tools.
- Returned failed tool executions to the model as tool results instead of dropping them.
- Normalized `/` and empty file paths to workspace root for repo-scoped file tools.
- Added clean no-tools finalization from a compact evidence digest when the model exhausts tool rounds.
- Added provider-executed metadata propagation for provider-selected tool results in the orchestrated model loop.
- Added deterministic base fields to generated TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart payloads.
- Added a repeated-tool doom-loop guard in `crates/engine/src/orchestrator.rs` using threshold `3`, with visible interruption text and run metadata.
- Added a structured doom-loop permission envelope.
- Added ToolPart-style result metadata for completed/failed tool states: status, title, call id, timing, output shape, and error fields.
- Added normalized attachment metadata for successful file/patch tool results.
- Repaired the live proof harness startup path so workflow artifacts include the exact launched command and useful server logs when readiness fails.
- Rewrote the live proof harness after repeated unmatched-quote failures so it uses self linting, quote-safe marker loops, Python JSON creation, and plain status output.
- Added provider-visible independence guards to the live proof harness for `/api/tools`.
- Removed upstream source paths and upstream-prefixed metadata keys from WebUI runtime ToolPart lifecycle/result payloads in `crates/webui/src/events.rs`.
- Added live conversation snapshot streaming in `crates/webui/src/events_live.rs` so long natural-language WebUI/NIM runs expose conversation, ToolPart, tool-result, file-change, and text evidence while the run is still executing.
- Tightened the full benchmark prompt contract so the live WebUI run asks the model for the same tool-backed evidence and report labels that the checker verifies.
- Added tolerant batch_parallel request normalization for explicit and one-key shorthand tool call shapes.
- Added timeout artifact salvage so failed full benchmark runs still upload `full-benchmark-conversation.json` and both checker JSON files when possible.
- Bounded the Live WebUI benchmark budget to 36 rounds / 540 seconds and tightened the prompt to stop Phase 2 once checker-required evidence exists.
- Aligned Build Proof with the same benchmark budget and both benchmark checkers, so it no longer runs a stale longer proof contract.
- Full benchmark and workflow checkers now count nested batch_parallel outputs as real evidence.
- Full benchmark prompt now explicitly prevents the doom-loop/internal-search path that blocked Phase 3.
- Orchestrator benchmark guidance now requires dedicated `file_write` evidence for temporary benchmark files and prevents unproven build/check/test claims in fallback or model-written final reports.
- Forced finalization evidence now includes exact tool `path` and `command` fields, matching the tool-result output semantics used by the upstream source-backed ToolPart lifecycle.
- Full benchmark prompt final-answer contract now requires an exact opening confidence block and final self-check for the uppercase labels `VERIFIED`, `LIKELY`, and `UNKNOWN`.
- Full benchmark prompt now explicitly rejects blank/JSON/tool-call final output and requires a Markdown final self-check for exact labels before sending.
- `scripts/smoke/check-live-webui-proof-manifest.py` now requires screenshot/browser/stream/conversation/checker/status artifacts, NIM provider/model evidence, tool-call/tool-result counts, absence of local/upstream identity shortcuts, PNG screenshot bytes, browser proof markers, conversation tool-result counts, and status-path consistency.
- `scripts/smoke/check-full-agentic-benchmark.py` now detects cargo test/build/check claims from exact command prose only, preserving strict overclaim rejection without misreading required labels or generic report prose as cargo command claims.
- `file_edit` missing-`old_string` failures now return structured stale-edit recovery metadata with path, previews, recovery hint, recommended next tools, and a Forge failure lifecycle marker.
- File tool and patch-event runtime metadata now emit Forge-owned contract keys (`forge_*`) rather than upstream-branded runtime metadata or package source paths; exact reference paths remain in proof/docs only.
- Full benchmark prompt now makes the Phase 4 repository edit the immediate next operation after `.agent_test` verification and explicitly blocks the final report until a dedicated file-editing tool result outside `.agent_test` exists.
- File write/edit formatting now uses a source-backed formatter catalog with OpenCode-derived formatter families while keeping formatter absence/failure contained in tool metadata and resynchronizing UTF-8 BOM after formatter mutation.
- Formatter coverage now includes the remaining upstream OpenCode formatter families: Elixir/Phoenix template files via `mix`, experimental JS/TS via `oxfmt`, R via `air`, PHP via `pint`, Haskell via `ormolu`, Clojure/EDN via `cljfmt`, and D via `dfmt`.
- `scripts/smoke/check-formatter-parity.py` now enforces the OpenCode-backed formatter catalog/contract in CI so formatter families, representative extensions, source anchors, containment statuses, and Forge-owned runtime metadata cannot silently regress.
- `scripts/smoke/check-formatter-activation-evidence.py` now enforces that formatter activation source anchors stay recorded in the proof trail while Forge runtime formatter metadata remains Forge-owned.
- `scripts/smoke/check-formatter-config-activation-gap.py` now enforces that formatter proof docs do not overclaim config/dependency activation beyond the current source reality.
- `scripts/smoke/check-max-step-finalization-parity.py` now enforces the OpenCode-backed max-step/evidence-ready no-tools finalization contract in CI and tolerates equivalent evidence-bound conservative fallback wording.
- `crates/engine/src/orchestrator.rs` rejects weak final model text unless it contains the exact quality-score final-report contract and uses a deterministic OpenCode-backed fallback report with exact Markdown headings, evidence-bound claims, remaining-work semantics, and scorer-compatible labels.
- `scripts/smoke/score-live-benchmark-quality.py` now normalizes human-readable command claims back to successful tool command metadata, ignores prohibitive command mentions, and only rejects actual placeholder brackets.
- `scripts/smoke/score-live-benchmark-quality.py` now accepts any real Phase 4 edit marker in browser proof (`apply_patch`, `file_edit`, `file_write`, or the matching visible tool-card titles) instead of false-failing valid `file_edit` / `file_write` repo edits.

## OpenCode source anchors retained in developer docs only

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
