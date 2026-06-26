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
| `packages/opencode/src/tool/apply_patch.ts` | `patchText` schema, hunk parse, validation, permission metadata, file updates, watcher events, LSP diagnostics, success summary | Forge has a safe first-stage `apply_patch` surface only; full parity not done |
| `packages/opencode/src/tool/edit.ts` | Exact edit semantics, path handling, diff metadata, formatting/diagnostics hooks | Partially aligned through Forge `file_edit`; needs deeper comparison |
| `packages/llm/src/schema/events.ts` | LLM lifecycle event names | Partially copied in WebUI SSE proof |
| `packages/core/src/session/runner/publish-llm-event.ts` | Tool lifecycle validation and ordering | Partially copied in WebUI SSE proof |
| `packages/opencode/src/session/processor.ts` | Tool part states and assistant/tool result processing | Partially copied; WebUI tool cards still immature |

## Current highest-priority parity gaps

1. Full `apply_patch` implementation.
2. Orchestrator system prompt copied from OpenCode prompt behavior instead of a hand-written approximation.
3. Tool part state model matching OpenCode pending/running/completed/error behavior.
4. Context compaction and prompt/session continuation behavior.
5. Permission/edit approval flow with diff metadata.
6. Durable session/message/part persistence.

## `apply_patch` target behavior

Before claiming full `apply_patch` parity, Forge must prove these behaviors:

- Accept `patchText`.
- Reject empty patch text.
- Reject empty begin/end patch.
- Parse hunks rather than shelling out blindly.
- Support add, update, delete, and move.
- Validate target paths stay inside the allowed workspace.
- Collect per-file diff metadata.
- Ask/record edit permission metadata before applying changes.
- Write changes safely.
- Publish file change events.
- Trigger diagnostics/touch equivalent where available.
- Return a human-readable success summary listing changed files.

## Documentation update rule

Whenever behavior/status changes, update these files in the same work slice:

- `CONTINUE_HERE.md`
- `PROJECT_STATE.md`
- `FEATURE-AUDIT.md`
- `OPENCODE-PARITY.md`
- `AGENTS.md` when agent workflow rules change
