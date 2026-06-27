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
- Latest fully green baseline: `6a34928048b86e6d7b91468789eeef4489744ae8`.
- Latest proof artifact: `live-webui-feature-sprint-proof-6a34928.zip`.
- Current docs-updated HEAD still needs Actions before a fresh green claim.

## Latest OpenCode-source slice

Forge now records source-shaped receipts after approved `apply_patch` mutations:

- `opencode_event_source`
- `opencode_watcher_updates`
- `opencode_filesystem_edits`
- `lsp_touches`
- `diagnostics.touched_files`

Upstream sources studied:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/edit.ts`

Live proof at `6a34928` confirms these receipts in approval response, persisted conversation JSON, and browser DOM proof.

This is not yet a real live watcher bus or live LSP diagnostics implementation; it is the receipt/proof slice.

## Next source-backed targets

1. Full durable OpenCode `ToolPart` lifecycle parity.
2. Real watcher/file edited event bus beyond metadata receipts.
3. Live LSP diagnostics beyond touched-file receipts.
4. Full OpenCode compaction process parity beyond the request marker.
5. `AgentPart` / subtask behavior only if backed by a real Forge path.
6. `RetryPart` receipts if a deterministic retry path exists.
