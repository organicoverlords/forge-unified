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
- Latest pre-slice green head checked before this work: `3e609c24db429c6698f9a4c0e2c94e979945470b`.
- Pre-slice workflow proof on that head: CI `28278250979`, Build Proof `28278250967`, Live WebUI Feature Sprint `28278250966`.
- Pre-slice proof artifact: `live-webui-feature-sprint-proof`, artifact id `7920949901`.
- Current slice HEAD after LSP event work: `da6058ab0fd714464b70573b5609ed2fe383ac63`; wait for Actions before calling this head green.

## Latest OpenCode-source slice

Forge now extends the approved `apply_patch` post-edit activity path with OpenCode-shaped LSP diagnostic report envelopes and visible WebUI activity rail support:

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
  - docs now distinguish diagnostic event-envelope parity from a real language-server backend.

This is a meaningful parity slice, but it is **not full LSP parity**. Forge still does not run a language server; it emits the OpenCode-shaped diagnostic event envelope after edits so the UI/event stream has the correct durable shape for the next backend step.

## Previous proven slice

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

1. Wire `opencode_lsp_diagnostics` into `apply_patch` result metadata if not already present in the final checked head.
2. Attach a real LSP diagnostics backend to replace `pending_service` envelopes.
3. Continue real watcher/file edited event bus behavior beyond in-memory history.
4. Full OpenCode compaction process parity beyond the request marker.
5. `AgentPart` / subtask behavior only if backed by a real Forge path.
6. `RetryPart` receipts if a deterministic retry path exists.
