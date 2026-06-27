# OpenCode Parity Tracker

Updated: 2026-06-27

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Source-backed status

| OpenCode source | Forge status |
|---|---|
| `packages/opencode/src/tool/apply_patch.ts` | Forge parses/validates patches, pauses for edit approval before mutation, applies add/update/delete/move after approval, records changed-file summary lines, and now records source-shaped post-edit receipts. Live watcher bus, live LSP diagnostics, BOM, and formatter hooks remain incomplete. |
| `packages/opencode/src/tool/edit.ts` | Used as an additional source for filesystem edit events, watcher update events, formatting hooks, and LSP touch/diagnostics behavior. Forge applies this currently through apply_patch receipts; deeper file_edit parity remains. |
| `packages/opencode/src/patch/index.ts` | Parser and derive/apply replacement behavior partially copied into Rust helpers. |
| `packages/schema/src/v1/session.ts` | TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart shapes are persisted/rendered enough for proof. |
| `packages/opencode/src/session/processor.ts` | Tool result cards and metadata are partially copied; full lifecycle parity remains. |
| `packages/opencode/src/session/compaction.ts` | First durable CompactionPart request marker copied; full compaction process remains. |

## apply_patch implemented behavior

- Accepts `patchText`.
- Rejects empty patch text and empty begin/end patch.
- Parses add/update/delete/move hunks.
- Validates target paths before approval and before mutation.
- Records edit permission before applying changes.
- Writes changes only after approval.
- Emits file change cards/SSE metadata after approval.
- Records source-shaped filesystem edit receipts for non-delete approved patch targets.
- Records source-shaped watcher update receipts for add/change/unlink update sequences.
- Records LSP touch targets and diagnostics receipt metadata.
- Returns a human-readable success summary listing changed files.

## Not done / do not overclaim

- Live watcher event bus.
- Live LSP diagnostics service.
- Full BOM/formatter equivalence.
- Full ToolPart lifecycle/storage parity.
- Full compaction summary/replay/autocontinue behavior.
- AgentPart, RetryPart, StepStartPart, StepFinishPart, and SubtaskPart only when backed by a real Forge behavior path.

## Current highest-priority parity gaps

1. Full durable ToolPart lifecycle.
2. Real watcher/file edited event bus beyond metadata receipts.
3. Live LSP diagnostics beyond touched-file receipts.
4. BOM preservation and formatter hooks.
5. OpenCode prompt/system behavior.
6. Full context compaction process.
7. Durable session/message/part persistence beyond current snapshots.
