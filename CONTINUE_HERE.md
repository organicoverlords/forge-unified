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
- Latest fully green baseline: `a83ddac8542264cf69bd18988cd6e7dc6f518d95` for real OpenCode-style edit approval gating.
- Latest Live WebUI proof artifact: `live-webui-feature-sprint-proof-a83ddac.zip`.
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
- `a83ddac8542264cf69bd18988cd6e7dc6f518d95` — real edit approval-before-write gate for `apply_patch`; CI, Build Proof, and Live WebUI Feature Sprint all passed.

## Latest OpenCode-source slices

### Real edit approval gate

Upstream source studied:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`

Copied behavior:

- OpenCode calls `ctx.ask({ permission: "edit", patterns, always: ["*"], metadata: { filepath, diff, files } })` before file mutation.
- Forge no longer applies `apply_patch` immediately by default.
- First `apply_patch` call returns a durable pending edit approval with:
  - `permission_request`
  - `pending_edit_approval`
  - `approval_state.status=pending`
  - `applied=false`
- The file is not written before approval.
- `POST /api/conversations/:id/approvals/:approval_id/approve` re-runs the same patch with `approved=true`.
- Approved result records:
  - `approval_state.status=approved`
  - `approved_via_api=true`
  - `applied=true`
  - file events
  - FilePart and PatchPart only after approval.
- WebUI renders an `OpenCode edit permission request` card with an `Approve edit` control and `Edit approval metadata`.
- Live proof asserts the proof note does not exist before approval and does exist after approval.

### Session part stack

Upstream sources studied:

- `anomalyco/opencode`, branch `dev`, `packages/schema/src/v1/session.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/session/compaction.ts`

Forge behavior present and proofed:

- `TextPart` for user and assistant public text.
- `ReasoningPart` for safe public progress summaries only; never private chain-of-thought.
- `SnapshotPart` for explicit snapshot saves.
- `CompactionPart` for durable compaction request markers.
- `FilePart` for files changed by approved `apply_patch`, including `workspace://...` URLs.
- `ToolPart` running/completed/error metadata cards.
- `PatchPart` hashes and changed file lists for approved patches.

## Still incomplete versus upstream OpenCode

- Watcher/file edited events are not yet published as a real event bus.
- LSP touch/diagnostics collection is not yet implemented.
- BOM preservation and formatter hooks are not yet equivalent.
- Tool parts are durable enough for visible WebUI proof, but not full OpenCode pending/running/completed/error lifecycle parity.
- Orchestrator/system prompt is not yet fully copied from OpenCode prompt behavior.
- ReasoningPart is a safe public summary only, not hidden chain-of-thought.
- CompactionPart is a durable request marker and optional local pruning, not full OpenCode compaction process parity.

## Required workflow for new feature work

1. Identify the exact OpenCode behavior to copy.
2. Fetch and study the upstream OpenCode file first.
3. Record the upstream path in `OPENCODE-PARITY.md`.
4. Implement only a source-grounded slice.
5. Keep checked source files under the hard 500-line gate.
6. Update docs in the same branch before claiming done.
7. Validate with CI, Build Proof, and Live WebUI Feature Sprint.

## Current next target

After this docs head is green, continue with one of these source-backed slices:

1. Watcher/file edited event bus and LSP diagnostics for approved patch changes.
2. Full durable OpenCode `ToolPart` lifecycle parity.
3. Full OpenCode compaction process parity beyond the request marker.
4. OpenCode `AgentPart` / subtask behavior only if backed by a real Forge path.
5. Visible retry/fallback receipts with `RetryPart` if a deterministic retry path exists.

Do not add a broad invented workflow. Keep the natural browser proof style: normal user prompts, real tool execution, human summary, screenshot artifact.

## UX proof rule

Screenshot proof must show a completed, human-readable answer in the WebUI. Marker-only answers, JSON-only cards, or empty app-shell screenshots are invalid UX proof.
