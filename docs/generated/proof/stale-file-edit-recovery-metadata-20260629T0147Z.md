# Stale file_edit recovery metadata proof

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

PR #3 remains the selected work: open, non-draft, mergeable. Head before this slice was `3b9141c19f0a960ae8d3ebfae845fbe659a0dd4b`. CI `28342515471` and Build Proof `28342515478` passed. Live WebUI Feature Sprint `28342515474` failed, so the head was not accepted.

Prior accepted same-head browser/NIM artifact remains `7939934286` on head `74a5b5aa8836075fd187c2da404f25ac14c83229`.

## Failure addressed

The active gap was stale exact `file_edit` replacement recovery. Forge returned a bare `Old string not found` result, giving the next model round too little state to recover.

## Source backing

Inspected upstream source before patching:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `completeToolCall`
  - `failToolCall`
  - `updateToolCall`

Relevant copied behavior: failed running tool calls are first-class tool states with original input, explicit error text, and enough state for the next model round to recover.

## Implementation

Changed `crates/engine/src/tool/file_ops.rs`:

- Missing `old_string` now returns `stale_exact_replacement_old_string_not_found`.
- Failed result metadata now includes path, stale marker, current/old line counts, previews, recovery hint, recommended next tools, and a Forge failure lifecycle marker.
- New runtime failure metadata is Forge-owned and does not expose upstream source paths.

## Proof status

Committed implementation. Same-head CI / Build Proof / Live WebUI proof is still required before claiming this head proven.
