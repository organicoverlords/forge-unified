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
- Last code-fix commit before this docs refresh: `74eba32f57e9bfb682effaa202bdeac07f970c35`

## Latest recovery work

The previous failed handoff state was reconstructed from GitHub Actions and repo files.

- Failed head checked first: `6d2faa89a9b0e2637eb9f8a58c51459d5da55e77`
- Failure cause: CI and Build Proof failed only at the hard 500-line source gate.
- Offending source file: `crates/unifiedgraph/src/main.rs` at 632+ lines in the PR merge checkout.
- Fix applied:
  - Added `crates/unifiedgraph/src/cli.rs` for graphify CLI definitions.
  - Reduced `crates/unifiedgraph/src/main.rs` to a small dispatch entrypoint.
- First post-fix Actions signal: File Size Gate passed in CI; File line gate and Cargo check passed in Build Proof. Full workflow completion must still be verified on the latest HEAD before calling the branch green.

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

After the latest Actions are green, continue `apply_patch` parity. The current Forge surface accepts OpenCode-style `patchText`, but full OpenCode behavior is not complete yet.

Study first:

- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/patch/*`
- `packages/opencode/src/tool/edit.ts`
- `packages/opencode/src/session/processor.ts`

Then copy behavior in small validated slices.

## UX proof rule

Screenshot proof must show a completed, human-readable answer in the WebUI. Marker-only answers are invalid UX proof.
