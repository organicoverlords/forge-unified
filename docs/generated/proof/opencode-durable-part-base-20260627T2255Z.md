# OpenCode parity proof — durable part base fields

Date: 2026-06-27T22:55Z
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Source of truth

Target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

Selected branch remains `mvp/nim-freellmapi-router-20260626` because PR #3 is open, non-draft, and its head branch is the requested target branch.

## OpenCode source backing

Upstream paths used:

- `anomalyco/opencode/packages/schema/src/v1/session.ts`
  - `partBase` requires `id`, `sessionID`, and `messageID` on session parts.
  - `PartID` starts with `prt`.
  - `MessageID` starts with `msg`.
  - `ToolPart`, `TextPart`, `ReasoningPart`, `SnapshotPart`, `CompactionPart`, `FilePart`, and `PatchPart` all spread `partBase`.
- `anomalyco/opencode/packages/schema/src/session-id.ts`
  - `SessionID` starts with `ses`.

## Implemented Forge slice

`crates/engine/src/tool_parts.rs` now attaches deterministic OpenCode-style part base fields to generated session part payloads:

- `id: prt_forge_<hash>`
- `sessionID: ses_forge_<hash>`
- `messageID: msg_forge_<hash>`
- `metadata.opencode_part_base_source` recording the exact OpenCode schema path and copied fields.

The slice applies to text, reasoning, snapshot, compaction, file attachment, patch, and tool lifecycle parts. Existing ToolPart lifecycle behavior remains unchanged, but the generated records are closer to OpenCode schema shape and easier to prove/export in the WebUI.

## Files changed

- `crates/engine/src/tool_parts.rs`
- `docs/generated/proof/opencode-durable-part-base-20260627T2255Z.md`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`

## Validation status

Connector-side code editing completed through GitHub API. Same-head GitHub Actions must prove compile/browser behavior. Browser screenshot proof with NVIDIA NIM only is still required for the final HEAD before live parity can be claimed.

## Do not overclaim

This is not full OpenCode parity. It closes one schema-shape gap for generated session parts. Database-backed OpenCode part persistence and live browser proof remain incomplete until same-head workflows pass and artifacts are available.
