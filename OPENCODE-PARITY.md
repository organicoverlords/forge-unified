# OpenCode Parity Tracker

Updated: 2026-06-26

## Rule

Forge must not claim OpenCode parity from vibes. Every parity claim must cite an upstream OpenCode source path and the copied behavior.

## Studied upstream sources

| OpenCode source | Behavior to copy | Forge status |
|---|---|---|
| `packages/opencode/src/session/prompt/default.txt` | Concise, direct CLI-style answers; GitHub-flavored markdown; no unnecessary preamble/postamble; inspect code before editing; verify when completing work | Partially copied into screenshot proof prompt; orchestrator system prompt still needs a source-gated rewrite |
| `packages/opencode/src/session/prompt.ts` | Prompt/session flow, title generation, subtask and shell message/part handling | Studied only; not fully copied |
| `packages/opencode/src/session/system.ts` | Provider-specific prompt selection and environment context prompt | Studied only; not copied |
| `packages/opencode/src/tool/apply_patch.ts` | `patchText` schema, hunk parse, empty patch rejection, path validation, permission metadata, file updates, watcher events, LSP diagnostics, `A/D/M` success summary | Forge now parses OpenCode patch markers/hunks, rejects empty patch bodies, validates paths, records edit-permission/diff metadata, applies add/update/delete/move file mutations, and returns `A/D/M` success summary lines; watcher events, LSP diagnostics, real permission prompt, BOM, and formatter hooks remain incomplete |
| `packages/opencode/src/patch/index.ts` | Begin/end markers, add/delete/update/move hunk parsing, update chunks, EOF markers, multi-pass line matching, derive new contents from chunks | Parser and derive/apply replacement slice copied into Rust helpers; exact/rstrip/trim/Unicode matching implemented; BOM behavior is still not fully equivalent |
| `packages/opencode/src/tool/edit.ts` | Exact edit semantics, path handling, diff metadata, formatting/diagnostics hooks | Partially aligned through Forge `file_edit`; needs deeper comparison |
| `packages/llm/src/schema/events.ts` | LLM lifecycle event names | Partially copied in WebUI SSE proof |
| `packages/core/src/session/runner/publish-llm-event.ts` | Tool lifecycle validation and ordering | Partially copied in WebUI SSE proof |
| `packages/opencode/src/session/processor.ts` | Tool part states and assistant/tool result processing | Partially copied; WebUI tool cards still immature |

## Current highest-priority parity gaps

1. Real permission/edit approval flow for `apply_patch`.
2. Watcher/file edited events for file mutations.
3. LSP touch/diagnostics after patch mutations.
4. BOM preservation and formatter hooks.
5. Orchestrator system prompt copied from OpenCode prompt behavior instead of a hand-written approximation.
6. Tool part state model matching OpenCode pending/running/completed/error behavior.
7. Context compaction and prompt/session continuation behavior.
8. Durable session/message/part persistence.

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
- Publish file change events. — not yet.
- Trigger diagnostics/touch equivalent where available. — not yet.
- Preserve BOM and run formatter hooks where appropriate. — not yet.
- Return a human-readable success summary listing changed files. — implemented with `Success. Updated the following files:` and `A/D/M` lines.

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
