# Continue Here — Forge Unified

Updated: 2026-06-26

## Start here in every new chat / agent run

1. Read this file.
2. Read `PROJECT_STATE.md`.
3. Read `OPENCODE-PARITY.md`.
4. Read `FEATURE-AUDIT.md`.
5. Read `AGENTS.md`.
6. Verify branch, HEAD, dirty state, and latest CI/Build Proof/Live WebUI Feature Sprint status before editing.

## Current branch

- PR branch: `mvp/nim-freellmapi-router-20260626`
- Latest verified code head before this document sync: `b1b31301a1992dc3ea6accdecee1ac1b71dac256`
- That head was green for CI, Build Proof, and Live WebUI Feature Sprint.

## Current direction

The product goal is OpenCode-equivalent behavior inside Forge's Rust/WebUI app. Do not implement custom approximations when OpenCode has a source file that defines the behavior.

## Required workflow for new feature work

1. Identify the exact OpenCode behavior to copy.
2. Fetch and study the upstream OpenCode file first.
3. Record the upstream path in `OPENCODE-PARITY.md`.
4. Implement only the smallest source-grounded slice.
5. Update docs in the same branch before claiming done.
6. Validate with CI, Build Proof, and Live WebUI Feature Sprint.

## Current next target

Finish `apply_patch` parity. The current Forge surface accepts OpenCode-style `patchText`, but full OpenCode behavior is not complete yet.

Study first:

- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/patch/*`
- `packages/opencode/src/tool/edit.ts`
- `packages/opencode/src/session/processor.ts`

Then copy behavior in small validated slices.

## UX proof rule

Screenshot proof must show a completed, human-readable answer in the WebUI. Marker-only answers are invalid UX proof.
