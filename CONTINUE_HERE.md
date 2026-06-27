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
- Latest fully green baseline: `71a0ad32b979c656ea18e92eaf460b63b91425a6`.
- Latest proof artifact: `live-webui-feature-sprint-proof-71a0ad3.zip`.
- Current docs-updated HEAD still needs Actions before a fresh green claim.

## Latest OpenCode-source slice

Forge now persists OpenCode-shaped ToolPart lifecycle receipts:

- `tool_lifecycle_parts`
- pending ToolState
- running ToolState
- completed/error ToolState
- schema source: `packages/schema/src/v1/session.ts`
- processor source: `packages/opencode/src/session/processor.ts`

Live proof at `71a0ad3` confirms lifecycle receipts in stream proof, persisted conversation JSON, and browser DOM proof.

This is still not perfect OpenCode storage semantics; Forge records lifecycle receipts, not a single mutable part row updated in place.

## Previous proven slice

Approved `apply_patch` results persist OpenCode-shaped post-edit receipts for filesystem edits, watcher updates, LSP touch targets, and diagnostics touch metadata.

## Next source-backed targets

1. Real watcher/file edited event bus beyond metadata receipts.
2. Live LSP diagnostics beyond touched-file receipts.
3. Full OpenCode compaction process parity beyond the request marker.
4. `AgentPart` / subtask behavior only if backed by a real Forge path.
5. `RetryPart` receipts if a deterministic retry path exists.
