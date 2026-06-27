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
- Latest fully green pre-slice head checked before this work: `f71fcbc8627958979ee36a7ace6d9f0127a2b2c5`.
- Pre-slice workflow proof on that head: CI `28278959786`, Build Proof `28278959788`, Live WebUI Feature Sprint `28278959801`.
- Pre-slice proof artifact: `live-webui-feature-sprint-proof`, artifact id `7921179714`.
- Current slice HEAD after compaction work: `34a610ac16d927f3177765a586e908fa6dab3e86`; wait for Actions before calling this head green.

## Latest OpenCode-source slice

Forge now extends context compaction from a request marker into an OpenCode-shaped deterministic compaction summary path:

- Upstream source path:
  - `packages/core/src/session/compaction.ts`
- OpenCode behaviors copied:
  - split older conversation head from preserved recent tail;
  - serialize user, assistant, tool, and system context;
  - produce the same structured Markdown summary sections (`Goal`, `Constraints & Preferences`, `Progress`, `Key Decisions`, `Next Steps`, `Critical Context`, `Relevant Files`);
  - store recent tail context separately as `compaction_recent`;
  - keep a recent tail after compaction so the next turn can continue without replaying stale context.
- Forge files touched:
  - `crates/engine/src/conversation.rs`
  - `scripts/smoke/live-webui-feature-sprint.sh`
  - `OPENCODE-PARITY.md`
  - `CONTINUE_HERE.md`

This is a meaningful parity slice, but it is **not full compaction parity**. Forge still does not stream the summary through the selected model, and it does not yet publish OpenCode-equivalent compaction start/end events.

## Previous proven slice

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
- Behavior added:
  - approved patch metadata now contains `opencode_lsp_diagnostics` report envelopes;
  - the event rail explicitly recognizes `lsp.diagnostics` events and displays diagnostic count/status;
  - docs distinguish diagnostic event-envelope parity from a real language-server backend.

This is **not full LSP parity**. Forge still does not run a language server; it emits the OpenCode-shaped diagnostic event envelope after edits so the UI/event stream has the correct durable shape for the next backend step.

## Older proven slice

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

1. Check Actions for current compaction slice HEAD.
2. Add OpenCode-style compaction start/end events.
3. Make compaction LLM-backed through NVIDIA NIM only.
4. Attach a real LSP diagnostics backend to replace `pending_service` envelopes.
5. Continue real watcher/file edited event bus behavior beyond in-memory history.
6. `AgentPart` / subtask behavior only if backed by a real Forge path.
7. `RetryPart` receipts if a deterministic retry path exists.
