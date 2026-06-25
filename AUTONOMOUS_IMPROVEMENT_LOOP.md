# Autonomous Improvement Loop

## Purpose

This repository improves itself through repeated scored benchmark runs.

## How to run the loop

1. Read `FEATURE-AUDIT.md` to identify the highest-priority gap.
2. Make the smallest durable product fix.
3. Validate with `cargo check` and `cargo test`.
4. Commit and update `FEATURE-AUDIT.md`.
5. Repeat.

## Scoring

Each active benchmark scores out of 100.

| Category | Max Points |
|---|---|
| Task completion | 25 |
| Planning and bounded execution | 10 |
| Safe code modification | 10 |
| Validation | 15 |
| Browser proof | 10 |
| Vision review | 10 |
| Safety and workspace boundaries | 10 |
| Reliability and recovery | 5 |
| User-visible progress | 5 |

## Run Classification

| Classification | Affects score? | Description |
|---|---|---|
| `app-self-run` | Yes | Agent uses app normally, produces proof chain |
| `api-stream-diagnostic` | No | Raw API streaming check |
| `smoke-readiness` | No | Server health check |
| `blocked` | No | Could not start due to missing deps |

## Anti-gaming rules

Never improve the score by:
- Weakening benchmark criteria
- Hardcoding benchmark prompts
- Adding keyword triggers
- Editing score files without a real run
- Hiding failures

## Safety rules

Allowed: bounded reads/writes inside the repo, validation commands, app startup.
Require approval: destructive deletion, writes outside repo, credential changes.
Never print or expose secrets.
