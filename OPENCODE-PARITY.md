# OpenCode Parity Tracker

Updated: 2026-06-27

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Source-backed status

| OpenCode source | Forge status |
|---|---|
| `packages/schema/src/v1/session.ts` | ToolPart lifecycle receipt slice is implemented for pending, running, completed, and error states. TextPart, ReasoningPart, SnapshotPart, CompactionPart, FilePart, ToolPart, and PatchPart remain proofed. |
| `packages/opencode/src/session/processor.ts` | Forge records lifecycle receipts shaped from ensureToolCall/updateToolCall/completeToolCall/failToolCall. Exact mutable row storage and attachment handling remain incomplete. |
| `packages/opencode/src/tool/apply_patch.ts` | Patch parsing, approval gate, changed-file summaries, post-edit receipts, watcher update receipts, and LSP diagnostic event envelopes are implemented. Live LSP collection, BOM, and formatter hooks remain incomplete. |
| `packages/opencode/src/tool/edit.ts` | Used as an additional source for edit-event and LSP/formatter behavior. Deeper file_edit parity remains. |
| `packages/opencode/src/patch/index.ts` | Parser and derive/apply replacement behavior partially copied into Rust helpers. |
| `packages/core/src/session/compaction.ts` | Forge now selects an old head versus recent tail, serializes old context, emits a structured Markdown `compaction_summary`, stores `compaction_recent`, and keeps a recent tail. LLM-streamed summary generation and exact event lifecycle remain incomplete. |

## Implemented behavior

- Tool lifecycle receipts include pending, running, and final states.
- Patch flow records post-edit receipts after confirmed edits.
- Session part cards are visible in WebUI proof.
- The event rail displays filesystem, watcher, and LSP diagnostic event-envelope activity.
- Compaction no longer stores only a marker: it now produces an OpenCode-shaped structured summary and preserves recent tail context.

## Not done / do not overclaim

- Exact OpenCode mutable part storage semantics.
- Live language-server diagnostics service.
- Full BOM/formatter equivalence.
- LLM-streamed compaction summaries with provider/event lifecycle parity.
- AgentPart, RetryPart, StepStartPart, StepFinishPart, and SubtaskPart only when backed by a real Forge behavior path.

## Current highest-priority parity gaps

1. Live LSP diagnostics beyond touched-file receipts.
2. BOM preservation and formatter hooks.
3. OpenCode prompt/system behavior.
4. LLM-backed compaction summary generation and compaction start/end events.
5. Durable session/message/part persistence beyond current snapshots.
