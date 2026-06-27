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
- Selected because it is the newest active open PR and the repo's latest meaningful app work.
- Last checked red head before this slice: `cecf545fa70de2a2a617df5182644da657152134`.
- Failed workflow runs on that head: CI `28280261010`, Build Proof `28280261002`, Live WebUI Feature Sprint `28280261000`.
- Root failure found in CI Smoke Test: app compiled/tests passed, but WebUI proof hit `127.0.0.1:3320` before the server was reliably ready; artifact `webui-smoke-proof`, id `7921588063`, was uploaded for the failed attempt.
- Current slice HEAD after compaction event lifecycle and proof hardening: wait for Actions before calling this head green.

## Latest OpenCode-source slice

Forge now extends deterministic compaction into visible OpenCode-style lifecycle receipts and hardens browser proof readiness:

- Upstream source path:
  - `packages/core/src/session/compaction.ts`
- OpenCode behaviors copied:
  - split older conversation head from preserved recent tail;
  - serialize user, assistant, tool, and system context;
  - produce structured Markdown summary sections (`Goal`, `Constraints & Preferences`, `Progress`, `Key Decisions`, `Next Steps`, `Critical Context`, `Relevant Files`);
  - store recent tail context separately as `compaction_recent`;
  - keep a recent tail after compaction;
  - publish compaction start/end lifecycle events around summary creation so the WebUI event rail shows the feature doing work.
- Forge files touched:
  - `crates/engine/src/orchestrator.rs`
  - `crates/engine/src/agent.rs`
  - `scripts/smoke/live-webui-feature-sprint.sh`
  - `OPENCODE-PARITY.md`
  - `CONTINUE_HERE.md`

This is a meaningful parity slice, but it is **not full compaction parity**. Forge still does not stream the summary through NVIDIA NIM, and exact OpenCode session storage semantics remain incomplete.

## Previous slice

Forge extended context compaction from a request marker into an OpenCode-shaped deterministic compaction summary path:

- Upstream source path:
  - `packages/core/src/session/compaction.ts`
- Forge files touched:
  - `crates/engine/src/conversation.rs`
  - `scripts/smoke/live-webui-feature-sprint.sh`
  - `OPENCODE-PARITY.md`
  - `CONTINUE_HERE.md`

## Older proven slice

Forge extends the approved `apply_patch` post-edit activity path with OpenCode-shaped LSP diagnostic report envelopes and visible WebUI activity rail support:

- Event metadata source paths:
  - `packages/opencode/src/tool/apply_patch.ts`
  - `packages/opencode/src/event-v2-bridge.ts`
  - `packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts`
  - `packages/opencode/src/lsp/index.ts`
- Forge files touched:
  - `crates/engine/src/tool/patch_events.rs`
  - `crates/engine/src/tool.rs`
  - `crates/webui/src/change_events.rs`

This is **not full LSP parity**. Forge still does not run a language server; it emits the OpenCode-shaped diagnostic event envelope after edits so the UI/event stream has the correct durable shape for the next backend step.

## Older proven ToolPart slice

Forge persists OpenCode-shaped ToolPart lifecycle receipts:

- `tool_lifecycle_parts`
- pending ToolState
- running ToolState
- completed/error ToolState
- schema source: `packages/schema/src/v1/session.ts`
- processor source: `packages/opencode/src/session/processor.ts`

Live proof at `71a0ad3` confirmed lifecycle receipts in stream proof, persisted conversation JSON, and browser DOM proof.

This is still not perfect OpenCode storage semantics; Forge records lifecycle receipts, not a single mutable part row updated in place.

## Next source-backed targets

1. Check Actions for current compaction lifecycle/proof-hardening HEAD.
2. Make compaction LLM-backed through NVIDIA NIM only.
3. Attach a real LSP diagnostics backend to replace `pending_service` envelopes.
4. Add BOM preservation and formatter hooks.
5. Continue real watcher/file edited event bus behavior beyond in-memory history.
6. `AgentPart` / subtask behavior only if backed by a real Forge path.
7. `RetryPart` receipts if a deterministic retry path exists.
