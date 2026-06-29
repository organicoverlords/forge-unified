# Forge Unified — Current State

Updated: 2026-06-29

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest accepted same-head proof HEAD: `c12789a7b7c59ba7bfe0ba22118892396356fc7c`
- Accepted same-head workflows: CI `28343848325`, Build Proof `28343848326`, Live WebUI Feature Sprint `28343848306`.
- Accepted Live WebUI artifact: `7941120525`, `live-webui-feature-sprint-proof`, digest `sha256:7f9758084a3701b759191daad22b04fae6eed8b259429be665a174e5aaa9c5c5`.
- Latest inspected failure: Live WebUI `28346413121` on head `2f02415c3008f4e476b6176a9028fec7cb3293f1` failed only `phase4_real_low_risk_edit`; CI `28346413124` and Build Proof `28346413122` passed.
- Latest source-backed slice: phase4 benchmark ordering hardening. The live benchmark prompt now blocks final reporting until a dedicated Phase 4 repository file-editing tool result outside `.agent_test` exists, and makes the Phase 4 edit the immediate next operation after `.agent_test` verification.
- Latest proof doc: `docs/generated/proof/live-benchmark-phase4-edit-ordering-20260629T0350Z.md`.
- Do not claim the latest Phase 4 ordering head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete on the newer commit.

## Accepted live full benchmark proof

Forge has accepted real browser proof for the full six-phase agentic benchmark prompt through the WebUI on `c12789a7b7c59ba7bfe0ba22118892396356fc7c`.

Proof requirements satisfied by artifact `7941120525`:

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

## OpenCode source anchors retained in developer docs only

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, provider-executed state, same-call ToolPart update semantics, complete/fail tool-call handling, tool-result output, normalized attachments, repeated-call comparison, and interrupted tool cleanup metadata.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — part base, ToolPart, ToolState, and FilePart schema shape.
- `anomalyco/opencode:packages/schema/src/session-id.ts` — SessionID prefix semantics.
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts` — EventV2Bridge receipt behavior.
- `anomalyco/opencode:packages/opencode/src/tool/write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts`, and `apply_patch.ts` — tool catalog behavior anchors.
- `anomalyco/opencode:packages/opencode/src/lsp/lsp.ts` and `packages/opencode/src/lsp/diagnostic.ts` — LSP touch, diagnostic collection, max error cap, report block, severity formatting, and warm-up containment anchors.
- `anomalyco/opencode:packages/core/src/file-mutation.ts` — BOM-preserving file mutation behavior anchor.

## Current behavior retained

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI tool prompt renders live ToolPart lifecycle cards with provider metadata.
- File-change and EventV2Bridge receipts are visible in chat.
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, and normalized tool attachment metadata.
