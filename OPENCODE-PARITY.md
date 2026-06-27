# OpenCode Parity Tracker

Updated: 2026-06-27

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Source-backed status

| OpenCode source | Forge status |
|---|---|
| `packages/schema/src/v1/session.ts` | ToolPart lifecycle receipt slice is now implemented for pending, running, completed, and error states. TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart remain proofed. |
| `packages/opencode/src/session/processor.ts` | Forge now records lifecycle receipts shaped from ensureToolCall/updateToolCall/completeToolCall/failToolCall. Exact mutable row storage and attachment handling remain incomplete. |
| `packages/opencode/src/tool/apply_patch.ts` | Patch parsing, approval gate, changed-file summaries, and post-edit receipts are implemented. Live watcher bus, live LSP diagnostics, BOM, and formatter hooks remain incomplete. |
| `packages/opencode/src/tool/edit.ts` | Used as an additional source for edit-event and LSP/formatter behavior. Deeper file_edit parity remains. |
| `packages/opencode/src/patch/index.ts` | Parser and derive/apply replacement behavior partially copied into Rust helpers. |
| `packages/opencode/src/session/compaction.ts` | First durable CompactionPart request marker copied; full compaction process remains. |

## Implemented behavior

- Tool lifecycle receipts include pending, running, and final states.
- Patch flow records post-edit receipts after confirmed edits.
- Session part cards are visible in WebUI proof.

## Not done / do not overclaim

- Exact OpenCode mutable part storage semantics.
- Live watcher event bus.
- Live LSP diagnostics service.
- Full BOM/formatter equivalence.
- Full compaction summary/replay/autocontinue behavior.
- AgentPart, RetryPart, StepStartPart, StepFinishPart, and SubtaskPart only when backed by a real Forge behavior path.

## Current highest-priority parity gaps

1. Real watcher/file edited event bus beyond metadata receipts.
2. Live LSP diagnostics beyond touched-file receipts.
3. BOM preservation and formatter hooks.
4. OpenCode prompt/system behavior.
5. Full context compaction process.
6. Durable session/message/part persistence beyond current snapshots.
