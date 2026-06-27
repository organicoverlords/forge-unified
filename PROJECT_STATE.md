# Forge Unified — Current State

Updated: 2026-06-27

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest fully green baseline: `6a34928048b86e6d7b91468789eeef4489744ae8`.
- Latest proof artifact: `/mnt/data/live-webui-feature-sprint-proof-6a34928.zip`.
- Current docs-updated HEAD still needs Actions before a fresh green claim.

## Latest proven slices

- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.

## Current behavior

- WebUI uses the newer dark Codex/OpenCode-like theme.
- Natural proof note prompt creates a pending edit approval before writing.
- Approval route applies the patch and records FilePart/PatchPart only after approval.
- Approved `apply_patch` results now persist OpenCode-shaped post-edit receipts for filesystem edits, watcher updates, LSP touch targets, and diagnostics touch metadata.
- Repo inspection still runs real `repo_info` and `file_list` tools with compact visible output and raw metadata preserved.
- Existing session part cards remain: TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, PatchPart.

## OpenCode sources used

- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/tool/edit.ts`
- `packages/opencode/src/patch/index.ts`
- `packages/opencode/src/session/processor.ts`
- `packages/schema/src/v1/session.ts`
- `packages/opencode/src/session/compaction.ts`

## Current gaps

- Event receipts are not yet a real live watcher bus.
- LSP touch receipts are not yet live diagnostics.
- BOM and formatter parity are still incomplete.
- ToolPart lifecycle parity is still incomplete.
- Compaction process parity is still incomplete.

## Next targets

1. Check latest Actions for docs-updated HEAD and fix exact failures.
2. Full durable OpenCode ToolPart lifecycle parity.
3. Real watcher/file edited event bus beyond receipts.
4. Live LSP diagnostics beyond touched-file receipts.
