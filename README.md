# Forge Unified — Rust AI Superapp

Forge Unified is a compact Rust workspace for a chat-first AI coding-agent app. It currently provides a real engine, REST API, bundled MVP chat page, provider abstraction, tool executor, browser-proof path, NIM vision-review path, and graph visualization scaffold.

This repository is **not yet a finished replacement** for `localgpt-cockpit-rust`, `forgestack`, or `rust-ai-superapp`. It is best treated as a clean consolidation base that is now moving toward a usable MVP.

## Current status

Updated through GitHub API work on 2026-06-26.

| Area | Status | Notes |
|------|--------|-------|
| Rust workspace | Real | `crates/engine`, `crates/webui`, `crates/app`, `crates/unifiedgraph` |
| CLI/server binary | Real | `forge` starts an Axum server |
| Bundled root UI | Partial/real | `/` serves a single-page chat UI that can create conversations, send messages, show messages/tool results, save snapshots, and open graph view |
| REST API | Real | Health, conversations, chat, task controls, snapshot, browser proof, vision review, benchmark, graph routes |
| Provider abstraction | Real | `Provider` trait with chat, stream, and health methods |
| Providers | Partial | NIM is explicit; Groq/OpenRouter use generic OpenAI-compatible provider config when matching env vars exist |
| Provider fallback | Basic | Priority-order fallback exists, but routing receipts and health/quota policy are immature |
| Tool execution | Real | File ops, web fetch/search, shell, terminal, task, batch, browser proof, vision review, graph tools are wired |
| Browser proof | Real | Uses local Chrome/Chromium headless screenshot and optional DOM dump |
| NIM vision review | Real but provider-specific | Requires a configured NIM key; defaults to Llama 3.2 11B vision |
| Conversation storage | Partial | Active conversations are in-memory; snapshots persist JSON files |
| Socket chat | Scaffold | Socket route currently echoes text; it is not real streaming chat yet |
| Task controls | API-shaped | Endpoints exist, but operational interruption inside model/tool loops needs proof/fixes |
| Benchmark adapter | Shallow | Capability summary only; not yet the full artifact-backed adapter used by Superapp |
| CI | Improved | Workflow now targets `master`, `main`, and `dev`; smoke test calls the MVP UI/API script |

## Architecture

```text
crates/
  engine/        core agent, orchestration, provider routing, tools, benchmark report, snapshots
  webui/         Axum HTTP server, REST routes, bundled MVP chat page, socket scaffold, graph visualization route
  app/           CLI binary entrypoint
  unifiedgraph/  graph extraction/query support
scripts/
  smoke/         smoke scripts for local/CI proof
```

## Quick start

```bash
cargo run --release -- --help
cargo run --release           # starts on http://0.0.0.0:3000
```

Then open:

```text
http://127.0.0.1:3000/
```

To run the MVP smoke script locally:

```bash
bash scripts/smoke/mvp-chat-ui-smoke.sh
```

## Provider setup

Forge auto-enables common provider configs when the matching environment variable is present. If no provider credential is configured, the app still launches, but model calls will report provider errors until a provider is configured.

Priority order is currently NIM, then Groq, then OpenRouter.

## Implemented code paths

- Bundled `/` chat MVP page
- Multi-turn in-memory conversation objects
- Conversation create/list/get/delete REST endpoints
- Snapshot save/load support
- Provider trait with non-streaming and streaming chat methods
- Generic OpenAI-compatible chat/completion client
- NVIDIA NIM default config and explicit provider path
- Env-driven default provider configs for NIM, Groq, and OpenRouter
- Runtime model/provider selection from first enabled provider
- Priority-order provider fallback
- Tool schema generation and tool-call conversion
- Parallel batch executor for independent tool requests
- File, web, shell, terminal, task, repo, patch proposal, browser, vision, and graph tool dispatch
- Browser proof via headless Chrome/Chromium screenshot and optional DOM capture
- NIM vision review for screenshots
- Graph visualization endpoint
- CI file-size gate and Rust build/test/lint workflow scaffold
- MVP smoke script for root UI, health, conversation creation, conversation readback, and snapshot save

## Main gaps before it can compete with the existing apps

1. Replace blocking chat POST UX with true streaming events.
2. Add durable conversation persistence, not only in-memory conversations plus manual snapshots.
3. Make task controls affect active model/tool loops.
4. Add visible model routing receipts, fallback events, and saved provider/model order UI.
5. Port mature commit-readiness and pre-push gates from LocalGPT/Superapp.
6. Replace the shallow benchmark report with artifact-backed Benchmark Adapter API v1.
7. Add proof smoke scripts for browser proof, vision review, model fallback, and normal UI usage.
8. Add provider settings UI and key test buttons.
9. Add first-class tool cards with live running/completed/failed state.

## Relationship to the older repos

- `rust-ai-superapp` is still the most feature-rich and benchmark/proof-ready line.
- `localgpt-cockpit-rust` is still more usable as a mature chat/product app.
- `forgestack` still has more mature UI/proof/vision workflows.
- `forge-unified` is smaller and cleaner, so it is a strong candidate for a consolidation target once the missing product layers are ported.

For detailed status, proof requirements, and the current reality matrix, read [`FEATURE-AUDIT.md`](FEATURE-AUDIT.md).
