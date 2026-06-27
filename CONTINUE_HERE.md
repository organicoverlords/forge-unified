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
- Red head inspected before this slice: `35aba23bc6bef9f56873d3b12fc5c405e55c8201`.
- Failed runs on that head: CI `28281595486`, Build Proof `28281595484`, Live WebUI Feature Sprint `28281595497`.
- Failure detail: Rust check/test/security passed; WebUI smoke failed in the live sprint script while connecting to local port `3320`. Failed proof artifact: `webui-smoke-proof`, id `7922057428`.
- Current branch HEAD after this slice: wait for Actions before calling it green.

## Latest OpenCode-source slice

Forge now extends the edit event path with optional LSP warmup containment and stronger natural browser proof readiness.

Upstream source paths:

- `packages/opencode/src/tool/read.ts`
- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/event-v2-bridge.ts`
- `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts`

Copied behavior:

- Optional LSP warmup defects are contained instead of breaking the user-visible path.
- Approved edits still emit filesystem, watcher, and LSP diagnostic report envelopes.
- Event rail now receives `lsp.warmup.contained` before diagnostics so the UI shows the LSP lifecycle.
- Natural WebUI proof now waits on health, index, event page, and event API readiness, retries transient localhost startup races, and requires chat plus event-rail screenshots.

Forge files touched:

- `crates/engine/src/tool/patch_events.rs`
- `crates/engine/src/tool/patch_ops.rs`
- `crates/engine/src/tool.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `OPENCODE-PARITY.md`
- `CONTINUE_HERE.md`

This is not full LSP parity. Forge still does not run a real language server; it records warmup containment and diagnostic event envelopes so the UI/event bus has the right shape for the next backend step.

## Previous compaction slice

- Upstream source path: `packages/core/src/session/compaction.ts`
- Forge now records structured compaction summaries, recent-tail preservation, and visible `session.compaction.started` / `session.compaction.finished` receipts.
- Still not full compaction parity: summary generation is deterministic and not yet streamed through NVIDIA NIM.

## Older proven ToolPart slice

Forge persists OpenCode-shaped ToolPart lifecycle receipts: pending, running, completed, and error states. Live proof at `71a0ad3` confirmed lifecycle receipts in stream proof, persisted conversation JSON, and browser DOM proof.

## Next source-backed targets

1. Check Actions for the current LSP warmup/proof-hardening HEAD.
2. Attach a real LSP diagnostics backend to replace warmup-contained / pending-service envelopes.
3. Make compaction LLM-backed through NVIDIA NIM only.
4. Add BOM preservation and formatter hooks.
5. Continue real watcher/file edited event bus behavior beyond in-memory history.
