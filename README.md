# Forge Unified — Rust AI Superapp

Forge Unified is a compact Rust workspace for a chat-first AI coding-agent app. It currently provides a real engine, REST API, provider abstraction, tool executor, browser-proof path, NIM vision-review path, and graph visualization scaffold.

This repository is **not yet a finished replacement** for `localgpt-cockpit-rust`, `forgestack`, or `rust-ai-superapp`. It is best treated as a clean consolidation base.

## Current status

Verified by GitHub API inspection on 2026-06-26.

| Area | Status | Notes |
|------|--------|-------|
| Rust workspace | Real | `crates/engine`, `crates/webui`, `crates/app`, `crates/unifiedgraph` |
| CLI/server binary | Real | `forge` starts an Axum server |
| REST API | Real | Health, conversations, chat, cancel/pause/resume, snapshot, browser proof, vision review, benchmark, graph routes |
| Provider abstraction | Real | `Provider` trait with chat, stream, and health methods |
| Providers | Partial | NIM is explicit; other OpenAI-compatible providers route through generic `OpenAiProvider` |
| Provider fallback | Basic | Priority-order fallback exists, but routing receipts and health/quota policy are immature |
| Tool execution | Real | File ops, web fetch/search, shell, terminal, task, batch, browser proof, vision review, graph tools are wired |
| Browser proof | Real | Uses local Chrome/Chromium headless screenshot and optional DOM dump |
| NIM vision review | Real but provider-specific | Requires `NVIDIA_NIM_API_KEY`; defaults to Llama 3.2 11B vision |
| Conversation storage | Partial | Active conversations are in-memory; snapshots persist JSON files |
| WebSocket chat | Scaffold | `/ws` currently echoes text; it is not real streaming chat yet |
| Frontend chat UI | Missing | No bundled chat-first HTML/JS UI yet; graph visualization page exists |
| Cancel/pause/resume | API-shaped | Endpoints exist, but operational interruption inside model/tool loops needs proof/fixes |
| Benchmark adapter | Shallow | Capability summary only; not yet the full artifact-backed adapter used by Superapp |
| CI | Needs fix | Workflow currently targets `main` and `dev`, while this repo default branch is `master` |

## Architecture

```text
crates/
  engine/        core agent, orchestration, provider routing, tools, benchmark report, snapshots
  webui/         Axum HTTP server, REST routes, WebSocket scaffold, graph visualization route
  app/           CLI binary entrypoint
  unifiedgraph/  graph extraction/query support
```

## Quick start

```bash
cargo run --release -- --help
cargo run --release           # starts on http://0.0.0.0:3000
```

## Implemented code paths

- Multi-turn in-memory conversation objects
- Conversation create/list/get/delete REST endpoints
- Snapshot save/load support
- Provider trait with non-streaming and streaming chat methods
- Generic OpenAI-compatible chat/completion client
- NVIDIA NIM default config and explicit provider path
- Priority-order provider fallback
- Tool schema generation and tool-call conversion
- Parallel batch executor for independent tool requests
- File, web, shell, terminal, task, repo, patch proposal, browser, vision, and graph tool dispatch
- Browser proof via headless Chrome/Chromium screenshot and optional DOM capture
- NIM vision review for screenshots
- Graph visualization endpoint
- CI file-size gate and Rust build/test/lint workflow scaffold

## Main gaps before it can compete with the existing apps

1. Build a real chat-first frontend.
2. Replace the WebSocket echo scaffold with real chat/streaming events.
3. Add durable conversation persistence, not only snapshots.
4. Make cancel/pause/resume affect active model/tool loops.
5. Add visible model routing receipts, fallback events, and saved provider/model order.
6. Port mature commit-readiness and pre-push gates from LocalGPT/Superapp.
7. Replace the shallow benchmark report with artifact-backed Benchmark Adapter API v1.
8. Fix CI branches so checks run on the actual default branch.
9. Add proof smoke scripts for browser proof, vision review, model fallback, and normal UI usage.

## Relationship to the older repos

- `rust-ai-superapp` is still the most feature-rich and benchmark/proof-ready line.
- `localgpt-cockpit-rust` is still more usable as a chat/product app.
- `forgestack` still has more mature UI/proof/vision workflows.
- `forge-unified` is smaller and cleaner, so it is a strong candidate for a consolidation target once the missing product layers are ported.

For detailed status, proof requirements, and the current reality matrix, read [`FEATURE-AUDIT.md`](FEATURE-AUDIT.md).
