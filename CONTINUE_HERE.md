# Continue Here — Forge Unified

Updated: 2026-06-27

## Start here in every new chat / agent run

1. Read this file.
2. Read `PROJECT_STATE.md`.
3. Read `OPENCODE-PARITY.md`.
4. Read `FEATURE-AUDIT.md`.
5. Read `AGENTS.md`.
6. Verify repo, branch, HEAD, PR state, and latest CI / Build Proof / Live WebUI Feature Sprint status before editing.

## Current branch and PR

- Repo: `organicoverlords/forge-unified`
- PR branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, base `master`
- Default branch: `master`
- Latest fully green baseline: `6a34928048b86e6d7b91468789eeef4489744ae8` for OpenCode post-edit event and LSP touch receipts.
- Latest Live WebUI proof artifact: `live-webui-feature-sprint-proof-6a34928.zip`.
- The docs-updated HEAD after this sync needs its own Actions check before a fresh green claim.

## Latest proven green baselines

- `0da7281dc0f85bb16906103343d2e9d24827dafa` — first OpenCode `apply_patch` mutation slice.
- `65c1cb5f5c534149d4e08000e8553a498767ed00` — compact WebUI tool-card slice.
- `7f46ea1c0e7498a353fa18a3781b062580105236` — natural proof-note + repo-inspection two-prompt proof.
- `e160fa4bf9326c26d5731e9fb474574a4d068b2f` — compact repo-inspection presentation.
- `b7b0e7eb88570900ad8e3252d8190004342678fd` — OpenCode `SnapshotPart` persistence.
- `c3f15e4a5ac9c84fb07a6a49ec87118c97c4c3e7` — OpenCode `FilePart` persistence.
- `a0efdb6372cd92ac6b579bd152f009bb3debefbd` — OpenCode `ReasoningPart` persistence.
- `84e459ef3bd4d4f88636239c76136617a98b68e3` — OpenCode `CompactionPart` persistence.
- `a83ddac8542264cf69bd18988cd6e7dc6f518d95` — real edit approval-before-write gate for `apply_patch`.
- `805406542b55f803924401459f881f5df43680b7` — modern dark Codex/OpenCode-style WebUI theme.
- `6a34928048b86e6d7b91468789eeef4489744ae8` — OpenCode post-edit event receipts for approved `apply_patch`; CI, Build Proof, and Live WebUI Feature Sprint all passed.

## Latest OpenCode-source slices

### Post-edit event and LSP touch receipts

Upstream sources studied:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/edit.ts`

Copied behavior shape:

- OpenCode publishes filesystem-edited events after add/update/move edit targets.
- OpenCode publishes watcher-updated events with add/change/unlink event kinds after mutations.
- OpenCode touches changed files through LSP and then collects diagnostics.
- Forge now records durable OpenCode-shaped receipts for approved `apply_patch` results:
  - `opencode_event_source`
  - `opencode_watcher_updates`
  - `opencode_filesystem_edits`
  - `lsp_touches`
  - `diagnostics.touched_files`
- Live proof artifact confirms these receipts in approval response, persisted conversation JSON, and browser DOM proof.
- This is not yet a real live watcher bus or live LSP diagnostics implementation; it is a source-shaped event/diagnostics receipt slice.

### Real edit approval gate

Upstream source studied:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`

Copied behavior:

- Forge no longer applies `apply_patch` immediately by default.
- First `apply_patch` call returns a durable pending edit approval with `permission_request`, `pending_edit_approval`, `approval_state.status=pending`, and `applied=false`.
- The file is not written before approval.
- `POST /api/conversations/:id/approvals/:approval_id/approve` re-runs the same patch with `approved=true`.
- Approved result records approval state, file events, FilePart, and PatchPart.
- WebUI renders an `OpenCode edit permission request` card with an `Approve edit` control and metadata.

## Current next target

After this docs head is green, continue with one of these source-backed slices:

1. Full durable OpenCode `ToolPart` lifecycle parity.
2. Real watcher/file edited event bus beyond metadata receipts.
3. Live LSP diagnostics beyond touched-file receipts.
4. Full OpenCode compaction process parity beyond the request marker.
5. OpenCode `AgentPart` / subtask behavior only if backed by a real Forge path.
6. Visible retry/fallback receipts with `RetryPart` if a deterministic retry path exists.

Do not add a broad invented workflow. Keep the natural browser proof style: normal user prompts, real tool execution, human summary, screenshot artifact.
