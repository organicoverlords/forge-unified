# OpenCode search/glob human-output proof trail

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`

## Source backing

- `anomalyco/opencode:packages/opencode/src/tool/glob.ts`
- `anomalyco/opencode:packages/opencode/src/tool/grep.ts`

## Behavior retained for Forge

The deterministic CI gate `scripts/smoke/check-opencode-search-glob-contract.py` keeps Forge search/glob proof tied to the OpenCode source anchors above.

The retained contract is:

- path resolution before search execution;
- result count metadata for glob/search results;
- `No files found` behavior for empty results;
- bounded output / result limit behavior to avoid unbounded tool spam;
- human-readable output expectation for final tool answers, matching OpenCode-style glob/grep output rather than opaque-only receipts.

## Current implementation boundary

`crates/engine/src/tool/file_ops.rs` currently contains the Forge search/glob tool entrypoints and metadata path under guard. This proof trail does **not** claim complete search/glob parity by itself; it preserves the evidence contract that the next runtime implementation must satisfy and prevents silent deletion of the OpenCode-backed proof trail.

## Validation

- `python3 -m py_compile scripts/smoke/check-opencode-search-glob-contract.py`
- `python3 scripts/smoke/check-opencode-search-glob-contract.py`

## Rollback

Remove this proof note and revert the associated checker update if the search/glob parity direction changes.
