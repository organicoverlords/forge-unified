# Continue Here — Forge Unified

Updated: 2026-06-26

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
- Latest proven code baseline before this docs sync: `e160fa4bf9326c26d5731e9fb474574a4d068b2f`

## Latest proven green baselines

- `0da7281dc0f85bb16906103343d2e9d24827dafa` was fully green for the first OpenCode `apply_patch` mutation slice.
- `65c1cb5f5c534149d4e08000e8553a498767ed00` was fully green for the compact WebUI tool-card slice.
- `7f46ea1c0e7498a353fa18a3781b062580105236` was fully green for the natural proof-note + repo-inspection two-prompt proof.
- `e160fa4bf9326c26d5731e9fb474574a4d068b2f` was fully green for compact repo-inspection presentation: CI, Build Proof, and Live WebUI Feature Sprint all passed.
- The docs-updated HEAD after this sync must get its own Actions check before a fresh green claim.

## Latest OpenCode-source slices

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
- Live WebUI proof at `e160fa4` requires this compact output in the SSE stream, persisted conversation JSON, and browser DOM/screenshot proof.

## Still incomplete versus upstream OpenCode

- Interactive edit approval is recorded as metadata but not actually gated by a permission prompt.
- Watcher/file edited events are not yet published as a real event bus.
- LSP touch/diagnostics collection is not yet implemented.
- BOM preservation and formatter hooks are not yet equivalent.
- WebUI still does not fully persist OpenCode `ToolPart` pending/running/completed/error state as first-class durable parts.
- Orchestrator/system prompt is not yet fully copied from OpenCode prompt behavior.

## Required workflow for new feature work

1. Identify the exact OpenCode behavior to copy.
2. Fetch and study the upstream OpenCode file first.
3. Record the upstream path in `OPENCODE-PARITY.md`.
4. Implement only the smallest source-grounded slice.
5. Keep checked source files under the hard 500-line gate.
6. Update docs in the same branch before claiming done.
7. Validate with CI, Build Proof, and Live WebUI Feature Sprint.

## Current next target

After the docs sync is green, continue with the next small OpenCode-backed slice:

1. Real permission/edit approval flow for `apply_patch`, from `packages/opencode/src/tool/apply_patch.ts` and session approval handling.
2. Durable OpenCode-style tool part states from `packages/schema/src/v1/session.ts` and `packages/opencode/src/session/processor.ts`.
3. Watcher/file edited events and LSP diagnostics after patch mutations.

Do not add a broad invented workflow. Keep the natural browser proof style: normal user prompts, real tool execution, human summary, screenshot artifact.

## UX proof rule

Screenshot proof must show a completed, human-readable answer in the WebUI. Marker-only answers, JSON-only cards, or empty app-shell screenshots are invalid UX proof.
