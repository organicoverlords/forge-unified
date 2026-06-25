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
2. `AUTONOMOUS_IMPROVEMENT_LOOP.md` — how to run the self-improvement loop, classifications, scoring, anti-gaming rules
3. `self-healing-runs/scoreboard.json` — current benchmark state, scores, failing criteria, stuck items
4. `self-healing-runs/latest.json` — latest run summary
5. `self-healing-runs/history.jsonl` — append-only run history

## Benchmark / proof rules (short version)

- Curl/API checks are **smoke diagnostics only** — they cannot count as WebUI proof, browser proof, vision review, or app-self-run proof.
- Every scored run must have a `run_classification` from the approved list in `AUTONOMOUS_IMPROVEMENT_LOOP.md`.
- Only `app-self-run` runs may improve the self-improvement score.
- Threshold changes require old/new thresholds recorded, objective reason, both scores shown, no hidden regressions.
- Do not fix tool/research loops by increasing `max_rounds` alone; fix phase control, stall detection, tool ledger, validation ledger, provenance first.
- Do not commit large raw proof outputs; only compact summaries live in git.
- Before claiming Done, update `FEATURE-AUDIT.md` if behavior/status/proof requirements changed.
- **Before marking any run as `score_counted=true`, run:**
  ```bash
  bash scripts/validate-score-gate.sh
  ```
  The script inspects `self-healing-runs/latest.json` and the referenced artifact directory.
  If it exits nonzero, the `score_counted=true` claim is invalid — fix the proof chain first.

### WebUI Provenance Rule

`origin="webui"` is trusted only when proven by the application's normal browser UI path.
Do not trust a client-provided `origin` field by itself.

A run may be classified as `app-self-run` only when artifacts prove:

```text
browser opens the real app UI
→ user prompt is submitted through the visible WebUI
→ server creates or verifies the correlation ID
→ server records origin as WebUI-originated (derived from HTTP Origin header, not client body)
→ events.jsonl carries the same correlation ID
→ tool-ledger.jsonl carries the same correlation ID if tools ran
→ validation.json carries the same correlation ID if validation is claimed
→ browser-proof.json and screenshot carry the same correlation ID if browser proof is claimed
→ vision-review.json carries the same correlation ID if vision is claimed
→ summary.json/latest.json/history.jsonl carry the same correlation ID
```

If this chain is not proven, classify the run as one of:
`api-stream-diagnostic`, `smoke-readiness`, `stall-diagnostic`, `product-webui-run`,
`mixed`, `external-agent-assisted`, `not-proven`, or `blocked`.
Do not classify it as score-counted `app-self-run`.

Curl/API traffic may be useful diagnostics, but it must not count as WebUI provenance.

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

`dev` is the only normal active development branch. All implementation, refactor,
repair, benchmark, UI, provider, tool, self-build, and proof work must happen on `dev`.
Do not create new `feat/*`, `refactor/*`, `repair/*`, `benchmark/*`, dated, or
task-specific branches by default. Old feature branches are backup/history only
after consolidation.

### Continuation rule

When the user says "continue", "keep going", "next", "work on this", or similar:

1. Do not infer a new feature branch is needed.
2. Verify repo root, remote, branch, HEAD, dirty state.
3. If on `dev`, continue the next safe task on `dev`.
4. If on an old feature/refactor/repair branch, stop feature work and consolidate first.
5. If on `main`, switch to `dev` if safe; otherwise create/reset `dev` from stable `main`.

### Consolidation rule

When useful work exists on an old branch: validate, commit intended dirty files,
push once as backup, merge into `main`, validate `main`, push `main`,
create/reset `dev` from `main`, push `dev`, leave repo on `dev`.

Final state: `main` = stable checkpoint, `dev` = same HEAD, old branch = backup only.

### Dirty work rule

Dirty files are not automatically bad. If they are clearly the current intended
work: show the list, validate, commit only those files, continue consolidation.
If ambiguous, unrelated, secret-bearing, generated, or user-owned: stop, report,
do not stash/discard/overwrite/auto-commit.

### Force-push rule

Force-push is forbidden by default. The only exception is a one-time `dev` pointer
reset during consolidation using `--force-with-lease`, and only after `main` is
pushed and validated, local `dev` equals `main`, origin/dev unique commits inspected,
and backup branch created if needed. Never force-push `main`.

### Forbidden

- Creating new feature/refactor/repair branches by default.
- Splitting future work across multiple long-lived task branches.
- Continuing development on old branches after consolidation.
- Asking what branch name to use; use `dev`.
- Rebasing unless explicitly asked.
- Deleting old branches unless explicitly asked.
- Stashing/discarding user work.
- Claiming Done without repo root, branch, HEAD, dirty-state, validation, and push proof.

### Required final receipt

Every consolidation or continuation report must include: repo path, remote,
starting branch, final branch, source branch merged (if any), main HEAD before/after,
dev HEAD, dirty files handled, validation commands/results, push result,
final checked-out branch, confirmation no new feature branch was created.

One-line operating rule:

```text
Do not build more features until current work is consolidated into main and dev is checked out.
```
