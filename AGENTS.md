# Agent Instructions

This repo builds a standalone Rust AI superapp.

## Do not

- Do not wrap LibreChat.
- Do not depend on reference app runtimes.
- Do not require Docker/Mongo/Redis/Node/Python for MVP.
- Do not claim a feature is real without validation.
- Do not claim source grounding without repo/commit/file/line proof.
- Do not claim OpenCode parity without reading the matching upstream OpenCode source first.

## Before edits

Verify:

- repo path
- git top-level
- branch
- HEAD
- remote
- dirty status
- latest CI / Build Proof / Live WebUI Feature Sprint result for the current HEAD

## Canonical docs (must read before planning/coding/benchmarking/claiming Done)

1. `CONTINUE_HERE.md` — current continuation state and next task
2. `PROJECT_STATE.md` — current product and proof state
3. `OPENCODE-PARITY.md` — upstream OpenCode source map and parity gaps
4. `FEATURE-AUDIT.md` — product scope, feature status, proof requirements, orchestration rules, benchmark contract
5. `AGENTS.md` — this file
6. `README.md` — project overview

## OpenCode source-first workflow

When work is intended to match OpenCode:

1. Read the exact upstream OpenCode source file first.
2. Record the source path in `OPENCODE-PARITY.md`.
3. Copy only the smallest behavior slice that is understood.
4. Mark incomplete parity honestly.
5. Validate before claiming Done.

Current high-priority upstream files:

- `packages/opencode/src/session/prompt/default.txt`
- `packages/opencode/src/session/prompt.ts`
- `packages/opencode/src/session/system.ts`
- `packages/opencode/src/tool/apply_patch.ts`
- `packages/opencode/src/tool/edit.ts`
- `packages/opencode/src/session/processor.ts`
- `packages/llm/src/schema/events.ts`
- `packages/core/src/session/runner/publish-llm-event.ts`

## Benchmark / proof rules (short version)

- Curl/API checks are **smoke diagnostics only** — they cannot count as WebUI proof, browser proof, vision review, or app-self-run proof.
- WebUI screenshot proof must show a completed human-readable prompt response, not only an empty app shell.
- Every scored run must have a `run_classification` from the approved list.
- Only `app-self-run` runs may improve the self-improvement score.
- Do not commit large raw proof outputs; only compact summaries live in git.
- Before claiming Done, update `PROJECT_STATE.md`, `FEATURE-AUDIT.md`, and `OPENCODE-PARITY.md` if behavior/status/proof requirements changed.

## Branch and Continuation Policy

This repo uses a simple two-branch model:

```text
main = stable checkpoint
dev  = only active development branch
```

### Main rule

`main` is not a daily work branch. Allowed: stable checkpoint merge, validation, push.
Forbidden: new feature/refactor/repair/benchmark work, half-built dirty work.

### Dev rule

`dev` is the only normal active development branch.

### Continuation rule

When the user says "continue", "keep going", "next", "work on this", or similar:

1. Read `CONTINUE_HERE.md` first.
2. Do not infer a new feature branch is needed.
3. Verify repo root, remote, branch, HEAD, dirty state, and proof status.
4. Continue the next safe source-grounded task.
5. Update continuation docs before claiming Done.

### Consolidation rule

When useful work exists on an old branch: validate, commit intended dirty files,
push once as backup, merge into `main`, validate `main`, push `main`,
create/reset `dev` from `main`, push `dev`, leave repo on `dev`.

### Dirty work rule

Dirty files are not automatically bad. If they are clearly the current intended
work: show the list, validate, commit only those files, continue consolidation.
If ambiguous, unrelated, generated, or user-owned: stop, report,
do not stash/discard/overwrite/auto-commit.

### Forbidden

- Creating new feature/refactor/repair branches by default.
- Rebasing unless explicitly asked.
- Stashing/discarding user work.
- Force-push except when explicitly approved.
