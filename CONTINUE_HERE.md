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
- Latest code commit before this docs sync: `541e67fe40ef51dff5dc5b2507606dd68f7a0e2c`

## Latest proven green baselines

- `e31d678277c0527d36f14f8eac8fc65f07c3b265` was fully green for CI, Build Proof, and Live WebUI Feature Sprint.
- `541e67fe40ef51dff5dc5b2507606dd68f7a0e2c` was fully green for CI, Build Proof, and Live WebUI Feature Sprint.
- The latest docs-updated HEAD after this sync still needs its own Actions check before merge/green claims.

## Latest OpenCode-source slice

A new `apply_patch` parity slice was added and proved green at `541e67f`.

Upstream sources studied:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/patch/index.ts`

Forge changes:

- Added `crates/engine/src/tool/patch_ops.rs`.
- Split patch behavior out of `task_ops.rs` to keep all checked source files under 500 lines.
- `apply_patch` now:
  - accepts `patchText`;
  - rejects empty/malformed Begin/End patch text;
  - parses add/update/delete/move hunks;
  - rejects absolute, parent-dir, Windows-style, colon, and NUL paths before mutation;
  - records per-file metadata and edit-permission metadata;
  - returns OpenCode-style `A/D/M` summary lines;
  - still does **not** mutate files yet.

## Current direction

The product goal is OpenCode-equivalent behavior inside Forge's Rust/WebUI app. Do not implement custom approximations when OpenCode has a source file that defines the behavior.

## Required workflow for new feature work

1. Identify the exact OpenCode behavior to copy.
2. Fetch and study the upstream OpenCode file first.
3. Record the upstream path in `OPENCODE-PARITY.md`.
4. Implement only the smallest source-grounded slice.
5. Keep checked source files under the hard 500-line gate.
6. Update docs in the same branch before claiming done.
7. Validate with CI, Build Proof, and Live WebUI Feature Sprint.

## Current next target

Check latest docs-updated Actions first. If green, continue `apply_patch` parity from review metadata to safe file mutation.

Study first:

- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/patch/*`
- `packages/opencode/src/tool/edit.ts`
- `packages/opencode/src/session/processor.ts`

Then copy behavior in small validated slices.

## UX proof rule

Screenshot proof must show a completed, human-readable answer in the WebUI. Marker-only answers are invalid UX proof.
