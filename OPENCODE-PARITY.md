# OpenCode Parity Tracker

Updated: 2026-06-27

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Studied upstream sources

| OpenCode source | Behavior to copy | Forge status |
|---|---|---|
| `packages/opencode/src/tool/apply_patch.ts` | `patchText` schema, hunk parse, empty patch rejection, path validation, edit approval before mutation, file updates, filesystem edit events, watcher update events, LSP touch/diagnostics, success summary | Forge parses/validates patches, returns pending edit approval before mutation, exposes approval API/UI, mutates add/update/delete/move only after approval, records OpenCode-shaped filesystem/watcher/LSP touch receipts, and returns changed-file summary lines; live watcher bus, live LSP diagnostics, BOM, and formatter hooks remain incomplete |
| `packages/opencode/src/tool/edit.ts` | Exact edit semantics, path handling, diff metadata, edit permission, filesystem edit events, watcher update events, formatting hooks, LSP touch/diagnostics | Partially aligned through Forge `file_edit` and apply_patch event receipts; deeper file_edit parity still needed |
| `packages/opencode/src/patch/index.ts` | Begin/end markers, add/delete/update/move hunk parsing, update chunks, EOF markers, multi-pass line matching, derive new contents from chunks | Parser and derive/apply replacement slice copied into Rust helpers; exact/rstrip/trim/Unicode matching implemented; BOM behavior is still not fully equivalent |
| `packages/opencode/src/session/processor.ts` | Tool part lifecycle; completed tools expose title, metadata, output, optional attachments, and timing | Partially copied: Forge emits/persists tool results, uses metadata title, preserves raw details in metadata, and presents compact output first; durable lifecycle parity remains incomplete |
| `packages/schema/src/v1/session.ts` | Session part schema: `TextPart`, `ReasoningPart`, `SnapshotPart`, `CompactionPart`, `FilePart`, `PatchPart`, `ToolPart` / `ToolState` | Partially copied: Forge persists/render-proofs these shapes as OpenCode-style metadata/cards; not full OpenCode storage semantics yet |
| `packages/opencode/src/session/compaction.ts` | Creates `CompactionPart` messages with auto and optional overflow; processing may update tail_start_id, generate summaries, replay overflow turns, and auto-continue | First slice copied: durable compaction request marker, optional local pruning, API route, WebUI card, browser proof. Full LLM summary/replay/autocontinue process is not copied yet |
| `packages/opencode/src/session/prompt/default.txt` | Concise coding-agent answers; inspect code before editing; verify completion | Partially copied in natural WebUI proof style; orchestrator prompt still needs a source-gated rewrite |
| `packages/opencode/src/session/prompt.ts` | Prompt/session flow, title generation, subtask and shell message/part handling | Studied only; not fully copied |
| `packages/opencode/src/session/system.ts` | Provider-specific prompt selection and environment context prompt | Studied only; not copied |
| `packages/llm/src/schema/events.ts` | LLM lifecycle event names | Partially copied in WebUI SSE proof |
| `packages/core/src/session/runner/publish-llm-event.ts` | Tool lifecycle validation and ordering | Partially copied in WebUI SSE proof |
| `packages/opencode/specs/effect/tools.md` | Tool surface and migration list for tool-like behavior | Used as a guardrail: repo inspection uses existing Forge `repo_info` and `file_list` tools rather than inventing a fake OpenCode tool |

## Current highest-priority parity gaps

1. Full durable tool part lifecycle matching OpenCode pending/running/completed/error behavior.
2. Real watcher/file edited event bus beyond metadata receipts.
3. Live LSP diagnostics beyond touched-file receipts.
4. BOM preservation and formatter hooks.
5. Orchestrator system prompt copied from OpenCode prompt behavior instead of a hand-written approximation.
6. Full context compaction process: summary generation, replay, overflow handling, and auto-continue.
7. Durable session/message/part persistence beyond current snapshots.
8. Agent/subtask session part behavior.
9. Retry/fallback receipts as OpenCode-style retry parts.

## apply_patch target behavior

Implemented:

- Accepts `patchText`.
- Rejects empty patch text and empty begin/end patch.
- Parses add/update/delete/move hunks.
- Supports add/update/delete/move mutation after approval.
- Validates target paths before approval and before mutation.
- Collects per-file diff metadata.
- Records edit permission before applying changes.
- Writes changes safely with parent-directory creation after approval.
- Emits file change cards/SSE metadata after approval.
- Records OpenCode-shaped filesystem edited receipts for non-delete approved patch targets.
- Records OpenCode-shaped watcher update receipts for add/change/unlink update sequences.
- Records LSP touch targets and diagnostics receipt metadata.
- Returns a human-readable success summary listing changed files.

Not done:

- Live watcher bus.
- Live LSP diagnostics service.
- Full BOM/formatter equivalence.

## WebUI / tool result parity status

Implemented and proofed:

- Natural prompt creates a pending edit approval first and does not write a file before approval.
- WebUI shows `OpenCode edit permission request`, `Approve edit`, and `Edit approval metadata`.
- Approval API applies the patch and then shows file-change cards, FilePart, and PatchPart.
- Approval results include post-edit receipts: `opencode_watcher_updates`, `opencode_filesystem_edits`, `lsp_touches`, and diagnostics touch metadata.
- Natural repo inspection runs real `repo_info` and `file_list` tool calls.
- Tool cards show compact visible output first.
- Raw JSON details are preserved under metadata instead of being the primary visible UI.
- Live WebUI proof at `6a34928` verifies post-edit event and LSP touch receipts are persisted and browser-DOM visible.

Not done:

- Browser UI still does not fully implement OpenCode's pending/running/completed/error lifecycle semantics.
- Attachments are not modeled like OpenCode tool parts.
- Tool timing is not preserved in the same complete shape.
- Compaction card does not yet mean full OpenCode summary/replay/autocontinue behavior.
- File watcher and LSP receipts are not a live event bus or a live diagnostics service yet.

## File size rule

All checked source files must stay at or below 500 lines. The hard gate is `scripts/ci/check-file-lines.sh` and runs in both CI and Build Proof. Split files before merging instead of creating large monoliths.

Latest source-size recoveries:

- `crates/unifiedgraph/src/main.rs` exceeded the 500-line gate in the PR merge checkout; graphify CLI command/argument definitions were moved to `crates/unifiedgraph/src/cli.rs`.
- `crates/engine/src/tool/task_ops.rs` briefly exceeded the gate while adding the `apply_patch` slice; patch behavior was split into `crates/engine/src/tool/patch_ops.rs` and the line gate passed again.
- `crates/engine/src/tool/patch_apply.rs` was added for mutation helpers so `patch_ops.rs` stays focused and both files remain under the hard gate.
- `crates/engine/src/tool/patch_events.rs` was added for post-edit event receipt helpers so `patch_ops.rs` remains under the hard gate.

## Documentation update rule

Whenever behavior/status changes, update these files in the same work slice:

- `CONTINUE_HERE.md`
- `PROJECT_STATE.md`
- `FEATURE-AUDIT.md`
- `OPENCODE-PARITY.md`
- `AGENTS.md` when agent workflow rules change
