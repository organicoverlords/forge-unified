# Source-backed fuzzy file edit matching proof

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

- Source of truth target branch remained PR #3: `mvp/nim-freellmapi-router-20260626`.
- PR #3 was open, non-draft, and mergeable in metadata before this slice.
- Same-head workflows for `83b6ba30bb064d2f0aa92bb44beb3d75d69db3d8` showed CI and Build Proof green, while Live WebUI Feature Sprint failed.
- The failed Live WebUI run used real NVIDIA NIM model evidence but reached stale file-edit recovery and then timed out.

## OpenCode source backing

Exact upstream paths inspected and used:

- `anomalyco/opencode:packages/opencode/src/tool/edit.ts`
  - `replace`
  - `SimpleReplacer`
  - `LineTrimmedReplacer`
  - `WhitespaceNormalizedReplacer`
  - `IndentationFlexibleReplacer`
  - `TrimmedBoundaryReplacer`
  - conservative not-found / ambiguous-match errors

Relevant upstream behavior:

- Try exact replacement first.
- Fall back to normalized/fuzzy replacement strategies before failing stale edits.
- Refuse ambiguous matches instead of blindly editing.
- Preserve the error path when no safe unique replacement exists.

## Forge implementation

Changed file:

- `crates/engine/src/tool/file_ops.rs`

Runtime behavior added:

- `file_edit` now attempts exact replacement first.
- If exact replacement fails, it attempts OpenCode-backed fuzzy strategies:
  - line-trimmed matching
  - whitespace-normalized matching
  - indentation-flexible matching
  - trimmed-boundary matching
- Fuzzy matches must be unique unless `replace_all` is explicitly requested.
- Successful fuzzy edits include:
  - `edit_match_strategy`
  - `edit_matched_old_string_preview`
  - `forge_edit_replacer_contract`
- Stale edit failures now include fuzzy-replacer contract metadata and recovery guidance.

## Why this is not docs-only

This changes the runtime file editing tool in the engine. It reduces unnecessary stale-edit recovery loops in natural-language WebUI runs while keeping conservative no-edit behavior for ambiguous or unsafe matches.

## Current proof status

- Latest accepted full WebUI/NVIDIA NIM proof remains prior baseline artifact `7945828859` on head `8c20dbcc317b51ab69f16beeaf621cebaad939d6`.
- This new fuzzy-edit head is not same-head WebUI proven until CI, Build Proof, and Live WebUI Feature Sprint pass on `5627777a5ef167e0879bebc6a1a34e9437734dcf` or a later head containing this change.
