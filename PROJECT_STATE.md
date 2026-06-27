# Forge Unified — Current State

Updated: 2026-06-27

## Current branch

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Server port: `3000`
- Latest fully green baseline: `6a34928048b86e6d7b91468789eeef4489744ae8` for OpenCode post-edit event and LSP touch receipts.
- Latest Live WebUI proof artifact: `/mnt/data/live-webui-feature-sprint-proof-6a34928.zip`.
- The docs-updated HEAD after this refresh needs its own Actions check before merge/green claims.

## Latest fully green baselines

- `0da7281dc0f85bb16906103343d2e9d24827dafa` — OpenCode `apply_patch` mutation slice.
- `65c1cb5f5c534149d4e08000e8553a498767ed00` — cleaner WebUI tool cards.
- `7f46ea1c0e7498a353fa18a3781b062580105236` — natural proof note + repo inspection proof.
- `e160fa4bf9326c26d5731e9fb474574a4d068b2f` — compact repo inspection output.
- `b7b0e7eb88570900ad8e3252d8190004342678fd` — OpenCode `SnapshotPart` persistence.
- `c3f15e4a5ac9c84fb07a6a49ec87118c97c4c3e7` — OpenCode `FilePart` persistence.
- `a0efdb6372cd92ac6b579bd152f009bb3debefbd` — OpenCode `ReasoningPart` persistence.
- `84e459ef3bd4d4f88636239c76136617a98b68e3` — OpenCode `CompactionPart` persistence.
- `a83ddac8542264cf69bd18988cd6e7dc6f518d95` — real edit approval-before-write gate.
- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event and LSP touch receipts.

## Latest product behavior

### Natural file creation with edit approval and post-edit receipts

A normal user prompt creates a pending edit approval first:

```text
Please create a short proof note for this WebUI sprint.
```

Proven behavior:

- Records the user message.
- Executes `apply_patch` locally without depending on a provider.
- Returns a pending OpenCode-style edit permission request before writing.
- Persists `pending_edit_approval`, `permission_request`, `approval_state.status=pending`, and `applied=false`.
- The proof note does not exist before approval.
- WebUI renders `OpenCode edit permission request`, `Approve edit`, and `Edit approval metadata`.
- Approval route re-runs the patch with `approved=true` and writes the file.
- Approved result persists `approved_via_api=true`, `approval_state.status=approved`, `applied=true`, file events, FilePart, and PatchPart.
- Approved result also persists OpenCode-shaped post-edit receipts: `opencode_event_source`, `opencode_watcher_updates`, `opencode_filesystem_edits`, `lsp_touches`, and `diagnostics.touched_files`.
- The approved assistant summary is human-readable.

### Natural repository inspection proof

- Runs real `repo_info` and `file_list` tools.
- Presents compact visible output for repository status and top-level entries.
- Preserves raw JSON under `metadata.raw_output` for details.
- Persists the tool results and assistant summary.

### OpenCode session parts

Implemented/proofed through `6a34928`:

- `TextPart` metadata for user/assistant public text.
- `ReasoningPart` card for safe public progress summaries only.
- `SnapshotPart` card for explicit snapshot save.
- `CompactionPart` card for durable compaction request markers.
- `FilePart` card for changed files after approved edits, including `workspace://...` URL.
- `ToolPart` cards for running/completed/error states.
- `PatchPart` card with patch hash and file list after approved edits.

## OpenCode-source work copied so far

Studied upstream sources:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/edit.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/patch/index.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/session/processor.ts`
- `anomalyco/opencode`, branch `dev`, `packages/schema/src/v1/session.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/session/compaction.ts`

Forge changes:

- Added `crates/engine/src/tool/patch_apply.rs` for mutation helpers.
- Added `crates/engine/src/tool/patch_events.rs` for OpenCode-shaped post-edit event and LSP touch receipts.
- Kept `crates/engine/src/tool/patch_ops.rs` as parser/entrypoint glue.
- Updated `crates/engine/src/tool.rs` to register `patch_apply`, `patch_events`, and advertise mutation support.
- Added WebUI file-change cards and compact tool-result presentation.
- Added natural local action paths for file creation and repository inspection.
- Added OpenCode-style session part helpers and WebUI cards for text, reasoning, snapshot, compaction, file, tool, and patch parts.
- Added real edit approval-before-write handling for `apply_patch`.
- Added post-edit event receipts for approved `apply_patch` results.

## Current gaps, highest priority

| Area | Feature | Priority |
|------|---------|----------|
| Engine/WebUI | Full durable OpenCode tool-part lifecycle parity | P0 |
| Engine | Real watcher/file edited event bus beyond receipts | P0 |
| Engine | Live LSP diagnostics beyond touched-file receipts | P1 |
| Engine | BOM preservation and formatter hooks | P1 |
| Engine | Full OpenCode context compaction process parity | P1 |
| Router | Visible routing/fallback receipts and cooldown policy | P1 |
| Benchmark | Artifact-backed adapter contract | P2 |

## What not to do

- Do not claim full OpenCode parity for `apply_patch` yet.
- Do not claim full OpenCode compaction parity yet.
- Do not add invented workflows when OpenCode has a source-defined behavior.
- Do not remove or weaken the 500-line hard gate.
- Do not expose private chain-of-thought through `ReasoningPart`; only public progress summaries are allowed.
- Do not accept JSON-only screenshots as UX proof.
