# Agent Instructions

This repo builds a standalone Rust AI superapp.

## Do not

- Do not wrap LibreChat.
- Do not depend on reference app runtimes.
- Do not require Docker/Mongo/Redis/Node/Python for MVP.
- Do not claim a feature is real without validation.
- Do not claim source grounding without repo/commit/file/line proof.

## Before edits

Verify:

- repo path
- git top-level
- branch
- HEAD
- remote
- dirty status

## Canonical docs (must read before planning/coding/benchmarking/claiming Done)

1. `FEATURE-AUDIT.md` — product scope, feature status, proof requirements, orchestration rules, benchmark contract
2. `AGENTS.md` — this file
3. `README.md` — project overview

## Benchmark / proof rules (short version)

- Curl/API checks are **smoke diagnostics only** — they cannot count as WebUI proof, browser proof, vision review, or app-self-run proof.
- Every scored run must have a `run_classification` from the approved list.
- Only `app-self-run` runs may improve the self-improvement score.
- Do not commit large raw proof outputs; only compact summaries live in git.
- Before claiming Done, update `FEATURE-AUDIT.md` if behavior/status/proof requirements changed.

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

1. Do not infer a new feature branch is needed.
2. Verify repo root, remote, branch, HEAD, dirty state.
3. If on `dev`, continue the next safe task on `dev`.
4. If on `main`, switch to `dev` if safe; otherwise create/reset `dev` from stable `main`.

### Consolidation rule

When useful work exists on an old branch: validate, commit intended dirty files,
push once as backup, merge into `main`, validate `main`, push `main`,
create/reset `dev` from `main`, push `dev`, leave repo on `dev`.

### Dirty work rule

Dirty files are not automatically bad. If they are clearly the current intended
work: show the list, validate, commit only those files, continue consolidation.
If ambiguous, unrelated, secret-bearing, generated, or user-owned: stop, report,
do not stash/discard/overwrite/auto-commit.

### Forbidden

- Creating new feature/refactor/repair branches by default.
- Rebasing unless explicitly asked.
- Stashing/discarding user work.
- Force-push (except `dev` pointer reset with `--force-with-lease` during consolidation).
