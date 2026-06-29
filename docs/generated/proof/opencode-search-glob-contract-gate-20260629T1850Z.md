# OpenCode search/glob contract gate — 2026-06-29T18:50Z

## Selection

- Repository: `organicoverlords/forge-unified`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- Selection basis: target URL branch remains the source of truth and PR #3 is open/non-draft/mergeable; no newer open PR superseded it during this run.
- Pre-slice same-head proof baseline: `5b5e97c42a0c6b2daff1b23cfadf0360b8b7dc97`.

## Workflow baseline inspected before this slice

- CI `28391729746`: success.
- Build Proof `28391729720`: success.
- Live WebUI Feature Sprint `28391729702`: success.
- Live WebUI artifact: `live-webui-feature-sprint-proof`, artifact ID `7960159157`, digest `sha256:5ee681bf905142972ad7af4677af9c427f345bad04937b2eb06c50838e3972b5`.

## OpenCode source backing

- `anomalyco/opencode:packages/opencode/src/tool/glob.ts`.
  - Relevant behavior: resolves optional path against the current directory, rejects file paths for glob directory search, returns output lines, records result count metadata, and records truncation metadata.
- `anomalyco/opencode:packages/opencode/src/tool/grep.ts`.
  - Relevant behavior: resolves search path, supports include/file pattern behavior, returns `No files found` for empty results, records matches metadata, and records truncation metadata.

## Slice implemented

Added deterministic CI smoke coverage in `scripts/smoke/check-opencode-search-glob-contract.py`.

The gate checks that:

- Forge source retains `execute_file_glob` and `execute_file_search` in `crates/engine/src/tool/file_ops.rs`.
- Forge source retains result count metadata tokens for glob/search.
- Durable proof/state files retain `packages/opencode/src/tool/glob.ts` and `packages/opencode/src/tool/grep.ts` as source backing.
- Durable proof/state files retain behavior tokens for path resolution, result count metadata, and `No files found`.

## Claim boundary

This is a source-backed parity guard, not a same-head runtime parity claim.
The latest head must still pass CI, Build Proof, and Live WebUI Feature Sprint with NVIDIA NIM browser proof before it is called same-head proven.
