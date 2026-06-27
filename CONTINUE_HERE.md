# Continue Here — Forge Unified

Updated: 2026-06-27

## Start here

1. Read `PROJECT_STATE.md`.
2. Read `OPENCODE-PARITY.md`.
3. Read `FEATURE-AUDIT.md`.
4. Verify repo, branch, HEAD, PR state, and latest CI / Build Proof / Live WebUI Feature Sprint before editing.

## Current branch and PR

- Repo: `organicoverlords/forge-unified`
- PR branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, base `master`
- Selected because it is the newest active open PR and latest meaningful app work.
- Red head inspected before this slice: `b0de9a6666a8b18db8e3faf7f2c41ee722bb00d6`.
- Failed runs on that head: CI `28282995384`, Build Proof `28282995380`, Live WebUI Feature Sprint `28282995387`.
- Failure detail: Rust check/test/security passed, but WebUI smoke failed while connecting to local port `3320`; CI smoke proof artifact id `7922553769` captured the failed proof directory.
- Current branch HEAD after this slice: wait for Actions before calling it green.

## Latest OpenCode-source slice

Forge now adds an OpenCode EventV2Bridge-style event status layer and proves it through the natural WebUI sprint path.

Upstream source paths:

- `packages/opencode/src/event-v2-bridge.ts`
- `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts`
- `packages/opencode/src/tool/write.ts`
- `packages/opencode/src/tool/edit.ts`
- `packages/opencode/src/tool/apply_patch.ts`

Copied behavior:

- Event payloads are not only appended to a rail; the bridge now has a status summary with sequence range, event-type counts, source counts, and recently touched files.
- SSE `server.connected` includes the same status summary so the UI can show bridge state immediately.
- `/api/events/recent` includes `status`; `/api/events/status` returns the status directly.
- Approved edits still emit filesystem, watcher, LSP warmup containment, and LSP diagnostic report envelopes.
- Event rail UI now shows a Bridge status panel, count, sequence range, raw summary, and source paths.
- Natural WebUI proof now waits on event status readiness and requires `opencode_event_v2_bridge_status` in API and browser DOM proof.

Forge files touched:

- `crates/engine/src/change_bus.rs`
- `crates/engine/src/tool.rs`
- `crates/engine/src/orchestrator.rs`
- `crates/engine/src/agent.rs`
- `crates/webui/src/change_events.rs`
- `crates/webui/src/lib.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `CONTINUE_HERE.md`

This is not full EventV2 parity. Forge still uses in-memory history, not durable aggregate replay/storage.

## Previous LSP warmup slice

Forge extended the edit event path with optional LSP warmup containment and stronger natural browser proof readiness.

- Upstream paths: `packages/opencode/src/tool/read.ts`, `packages/opencode/src/tool/apply_patch.ts`, `packages/opencode/src/event-v2-bridge.ts`, `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts`.
- Optional LSP warmup defects are contained instead of breaking the user-visible path.
- Event rail receives `lsp.warmup.contained` before diagnostics so the UI shows the LSP lifecycle.
- Still not full LSP parity: Forge does not run a real language server yet.

## Previous compaction slice

- Upstream source path: `packages/core/src/session/compaction.ts`
- Forge records structured compaction summaries, recent-tail preservation, and visible `session.compaction.started` / `session.compaction.finished` receipts.
- Still not full compaction parity: summary generation is deterministic and not yet streamed through NVIDIA NIM.

## Older proven ToolPart slice

Forge persists OpenCode-shaped ToolPart lifecycle receipts: pending, running, completed, and error states. Live proof at `71a0ad3` confirmed lifecycle receipts in stream proof, persisted conversation JSON, and browser DOM proof.

## Next source-backed targets

1. Check Actions for the current event bridge status HEAD.
2. If WebUI smoke still fails on port `3320`, inspect `server.log` first; do not assume feature failure.
3. Attach a real LSP diagnostics backend to replace warmup-contained / pending-service envelopes.
4. Make compaction LLM-backed through NVIDIA NIM only.
5. Add BOM preservation and formatter hooks.
6. Continue durable watcher/file edited event bus behavior beyond in-memory history.
