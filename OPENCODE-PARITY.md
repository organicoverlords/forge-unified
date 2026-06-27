# OpenCode Parity Tracker

Updated: 2026-06-27

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Studied upstream sources

| OpenCode source | Behavior to copy | Forge status |
|---|---|---|
| `packages/opencode/src/session/prompt/default.txt` | Concise, direct coding-agent answers; inspect code before editing; verify when completing work | Partially copied in natural WebUI proof style; orchestrator system prompt still needs a source-gated rewrite |
| `packages/opencode/src/session/prompt.ts` | Prompt/session flow, title generation, subtask and shell message/part handling | Studied only; not fully copied |
| `packages/opencode/src/session/system.ts` | Provider-specific prompt selection and environment context prompt | Studied only; not copied |
| `packages/opencode/src/tool/apply_patch.ts` | `patchText` schema, hunk parse, empty patch rejection, path validation, permission metadata, file updates, watcher events, LSP diagnostics, `A/D/M` success summary | Forge parses OpenCode patch markers/hunks, rejects empty patch bodies, validates paths, records edit-permission/diff metadata, applies add/update/delete/move mutations, returns `A/D/M` summary lines, and renders file-change cards; watcher events, LSP diagnostics, real permission prompt, BOM, and formatter hooks remain incomplete |
| `packages/opencode/src/patch/index.ts` | Begin/end markers, add/delete/update/move hunk parsing, update chunks, EOF markers, multi-pass line matching, derive new contents from chunks | Parser and derive/apply replacement slice copied into Rust helpers; exact/rstrip/trim/Unicode matching implemented; BOM behavior is still not fully equivalent |
| `packages/opencode/src/tool/edit.ts` | Exact edit semantics, path handling, diff metadata, formatting/diagnostics hooks | Partially aligned through Forge `file_edit`; needs deeper comparison |
| `packages/llm/src/schema/events.ts` | LLM lifecycle event names | Partially copied in WebUI SSE proof |
| `packages/core/src/session/runner/publish-llm-event.ts` | Tool lifecycle validation and ordering | Partially copied in WebUI SSE proof |
| `packages/opencode/src/session/processor.ts` | Tool part lifecycle; completed tools expose `title`, `metadata`, `output`, optional attachments, and timing | Partially copied: Forge emits/persists tool results, uses `metadata.title`, preserves raw details in metadata, and presents compact output first; durable lifecycle parity remains incomplete |
| `packages/schema/src/v1/session.ts` | `TextPart`, `ReasoningPart`, `SnapshotPart`, `FilePart`, `PatchPart`, `ToolPart` / `ToolState` schema | Partially copied: Forge persists/render-proofs these shapes as OpenCode-style metadata/cards; not full OpenCode storage semantics yet |
| `packages/opencode/specs/effect/tools.md` | Tool surface and migration list for tool-like behavior | Used as a guardrail: repo inspection uses existing Forge `repo_info` and `file_list` tools rather than inventing a fake OpenCode tool |

## Current highest-priority parity gaps

1. Real permission/edit approval flow for `apply_patch`.
2. Watcher/file edited event bus for file mutations.
3. LSP touch/diagnostics after patch mutations.
4. BOM preservation and formatter hooks.
5. Orchestrator system prompt copied from OpenCode prompt behavior instead of a hand-written approximation.
6. Full durable tool part lifecycle matching OpenCode pending/running/completed/error behavior.
7. Context compaction and prompt/session continuation behavior.
8. Durable session/message/part persistence beyond current snapshots.
9. Agent/subtask session part behavior.
10. Retry/fallback receipts as OpenCode-style retry parts.

## Session part target behavior

Upstream source:

- `packages/schema/src/v1/session.ts`

Implemented / proofed:

- `TextPart`: public user/assistant text with collapsed metadata.
- `SnapshotPart`: explicit snapshot save messages and WebUI card.
- `FilePart`: changed-file metadata for successful `apply_patch`, including `mime`, `filename`, `workspace://...` URL, and `FilePartSource`-style file source.
- `ToolPart`: running/completed/error cards from tool calls/results, preserving metadata and compact visible output.
- `PatchPart`: patch hash and changed file list for successful `apply_patch`.
- `ReasoningPart`: safe public progress summary on assistant messages, with `private_chain_of_thought=false` and `visibility=public_progress_summary`.

Not done / do not overclaim:

- ReasoningPart is not hidden chain-of-thought capture and must not become that.
- ToolPart is not yet full OpenCode lifecycle/storage parity.
- AgentPart, CompactionPart, RetryPart, StepStartPart, StepFinishPart, and SubtaskPart remain to be implemented only when backed by a real Forge behavior path.

## `apply_patch` target behavior

Before claiming full `apply_patch` parity, Forge must prove these behaviors:

- Accept `patchText`. — implemented.
- Reject empty patch text. — implemented.
- Reject empty begin/end patch. — implemented.
- Parse hunks rather than shelling out blindly. — implemented for add/update/delete/move.
- Support add, update, delete, and move. — mutation implemented.
- Validate target paths stay inside the allowed workspace. — implemented before mutation.
- Collect per-file diff metadata. — implemented with a simple per-file diff summary.
- Ask/record edit permission metadata before applying changes. — metadata recorded; real interactive approval is not wired yet.
- Write changes safely. — implemented with parent-directory creation for writes.
- Publish file change events. — implemented as tool metadata/SSE/WebUI file cards; not yet a real watcher event bus.
- Trigger diagnostics/touch equivalent where available. — not yet.
- Preserve BOM and run formatter hooks where appropriate. — partially implemented at file write level; not fully equivalent to OpenCode's format/BOM sync flow.
- Return a human-readable success summary listing changed files. — implemented with `Success. Updated the following files:` and `A/D/M` lines.

## WebUI / tool result parity status

Implemented and proofed:

- Natural prompt creates a file through real `apply_patch` and shows an `ADDED` file card.
- Natural repo inspection runs real `repo_info` and `file_list` tool calls.
- Tool cards show compact visible output first.
- Raw JSON details are preserved under metadata (`raw_output`) instead of being the primary visible UI.
- Live WebUI proof at `c3f15e4` requires text/snapshot/file/tool/patch parts in the stream, persisted conversation JSON, and browser DOM.
- Live WebUI proof after `d880f839` also requires visible/persisted OpenCode `ReasoningPart` proof.

Not done:

- Browser UI still does not fully implement OpenCode's pending/running/completed/error lifecycle semantics.
- Attachments are not modeled like OpenCode tool parts.
- Tool timing is not preserved in the same complete shape.
- Real approval controls for edit permission are not implemented.

## File size rule

All checked source files must stay at or below 500 lines. The hard gate is `scripts/ci/check-file-lines.sh` and runs in both CI and Build Proof. Split files before merging instead of creating large monoliths.

Latest source-size recoveries:

- `crates/unifiedgraph/src/main.rs` exceeded the 500-line gate in the PR merge checkout; graphify CLI command/argument definitions were moved to `crates/unifiedgraph/src/cli.rs`.
- `crates/engine/src/tool/task_ops.rs` briefly exceeded the gate while adding the `apply_patch` slice; patch behavior was split into `crates/engine/src/tool/patch_ops.rs` and the line gate passed again.
- `crates/engine/src/tool/patch_apply.rs` was added for mutation helpers so `patch_ops.rs` stays focused and both files remain under the hard gate.

## Documentation update rule

Whenever behavior/status changes, update these files in the same work slice:

- `CONTINUE_HERE.md`
- `PROJECT_STATE.md`
- `FEATURE-AUDIT.md`
- `OPENCODE-PARITY.md`
- `AGENTS.md` when agent workflow rules change
