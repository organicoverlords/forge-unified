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
- Latest fully green baseline: `a0efdb6372cd92ac6b579bd152f009bb3debefbd` for OpenCode `ReasoningPart`.
- Latest source/proof code head before this docs sync: `bd118d718e01469445fafaf266527b97511bbba5` for OpenCode `CompactionPart`.
- The CompactionPart docs-updated HEAD after this sync needs its own Actions check before a fresh green claim.

## Latest proven green baselines

- `0da7281dc0f85bb16906103343d2e9d24827dafa` — first OpenCode `apply_patch` mutation slice.
- `65c1cb5f5c534149d4e08000e8553a498767ed00` — compact WebUI tool-card slice.
- `7f46ea1c0e7498a353fa18a3781b062580105236` — natural proof-note + repo-inspection two-prompt proof.
- `e160fa4bf9326c26d5731e9fb474574a4d068b2f` — compact repo-inspection presentation.
- `b7b0e7eb88570900ad8e3252d8190004342678fd` — OpenCode `SnapshotPart` persistence.
- `c3f15e4a5ac9c84fb07a6a49ec87118c97c4c3e7` — OpenCode `FilePart` persistence.
- `a0efdb6372cd92ac6b579bd152f009bb3debefbd` — OpenCode `ReasoningPart`; CI, Build Proof, and Live WebUI Feature Sprint all passed.

## Latest OpenCode-source slices

### Session part stack

Upstream source studied:

- `anomalyco/opencode`, branch `dev`, `packages/schema/src/v1/session.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/session/compaction.ts`

Forge behavior present and proofed through `a0efdb6`:

- `TextPart` for user and assistant public text.
- `ReasoningPart` for safe public progress summaries only; never private chain-of-thought.
- `SnapshotPart` for explicit snapshot saves.
- `FilePart` for files changed by `apply_patch`, including `workspace://...` URLs.
- `ToolPart` running/completed/error metadata cards.
- `PatchPart` hashes and changed file lists for successful patches.

New CompactionPart slice at source/proof code head `bd118d7`:

- Adds OpenCode-shaped `CompactionPart` helper using upstream shape: `type`, `auto`, optional `overflow`, optional `tail_start_id`.
- Adds `/api/conversations/:id/compact` to create a durable compaction request marker and optionally compact message history when `keep_last` is lower than current size.
- WebUI renders `OpenCode CompactionPart` and collapsed `CompactionPart metadata`.
- Live proof now requires `compaction_parts`, `"type":"compaction"`, `"identifier":"CompactionPart"`, `opencode_compaction_part_source`, and browser DOM proof.
- This is not full OpenCode compaction parity yet: no LLM summary generation, replay, plugin transform, auto-continue, or overflow retry loop.

### `apply_patch` mutation and file cards

Upstream sources studied:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/patch/index.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/session/processor.ts`
- `anomalyco/opencode`, branch `dev`, `packages/schema/src/v1/session.ts`

Forge behavior now present:

- `apply_patch` accepts `patchText`, parses Begin/End patches, validates paths, mutates add/update/delete/move hunks, and returns human-readable `A/D/M` summaries.
- Tool results preserve metadata and raw details while the WebUI shows file-change cards and compact human-readable output first.
- Natural prompt proof works without marker prompts: `Please create a short proof note for this WebUI sprint.` creates a real file via `apply_patch`, persists the tool result, shows an `ADDED` file card, and gives a human summary.

### Natural repo inspection

- Normal prompt: `Please inspect this repository and summarize what you find.`
- Forge runs real `repo_info` and `file_list` tools.
- Visible output is compact:
  - `Repository status:`
  - `Top-level repository entries`
- Raw JSON is preserved under metadata (`raw_output`) instead of being the main visible card.
- Live WebUI proof requires this compact output in the SSE stream, persisted conversation JSON, and browser DOM/screenshot proof.

## Still incomplete versus upstream OpenCode

- Interactive edit approval is recorded as metadata but not actually gated by a permission prompt.
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

After the CompactionPart docs head is green, continue with one of these source-backed slices:

1. Real permission/edit approval flow for `apply_patch`, from `packages/opencode/src/tool/apply_patch.ts` and session approval handling.
2. OpenCode `AgentPart` / subtask part behavior from `packages/schema/src/v1/session.ts`, only if the current task/subagent path is real enough to prove without faking.
3. Visible retry/fallback receipts with `RetryPart` if a deterministic retry path exists.
4. Full OpenCode compaction process parity: LLM summary generation, overflow replay, and auto-continue.

Do not add a broad invented workflow. Keep the natural browser proof style: normal user prompts, real tool execution, human summary, screenshot artifact.

## UX proof rule

Screenshot proof must show a completed, human-readable answer in the WebUI. Marker-only answers, JSON-only cards, or empty app-shell screenshots are invalid UX proof.
