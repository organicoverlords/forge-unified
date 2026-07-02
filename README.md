# Forge Unified — Rust AI Superapp

Forge Unified is a compact Rust workspace for a chat-first AI coding-agent app. It currently provides a real engine, REST API, bundled MVP chat page, provider abstraction, tool executor, browser-proof path, NIM vision-review path, and graph visualization scaffold.

This repository is **not yet a finished replacement** for `localgpt-cockpit-rust`, `forgestack`, or `rust-ai-superapp`. It is best treated as a clean consolidation base that is moving toward OpenCode-equivalent behavior.

## Start here

For current work, read these first:

1. [`CONTINUE_HERE.md`](CONTINUE_HERE.md)
2. [`PROJECT_STATE.md`](PROJECT_STATE.md)
3. [`OPENCODE-PARITY.md`](OPENCODE-PARITY.md)
4. [`FEATURE-AUDIT.md`](FEATURE-AUDIT.md)
5. [`AGENTS.md`](AGENTS.md)

## Current status

Updated through GitHub API work on 2026-06-26.

| Area | Status | Notes |
|------|--------|-------|
| Rust workspace | Real | `crates/engine`, `crates/webui`, `crates/app`, `crates/unifiedgraph` |
| CLI/server binary | Real | `forge` starts an Axum server |
| Bundled root UI | Partial/real | `/` serves a single-page chat UI that can create conversations, send prompts, show messages/tool results, and open graph view |
| REST API | Real | Health, conversations, chat, task controls, snapshot, browser proof, vision review, benchmark, graph routes |
| Provider abstraction | Real | `Provider` trait with chat, stream, and health methods |
| Providers | Partial | NIM is explicit; Groq/OpenRouter use generic OpenAI-compatible provider config when matching env vars exist |
| Provider fallback | Basic | Priority-order fallback exists, but routing receipts and health/quota policy are immature |
| Tool execution | Real/partial | File ops, web fetch/search, command tools, task, batch, browser proof, vision review, graph tools, and safe first-stage `apply_patch` surface are wired |
| Browser proof | Real | Produces screenshot and DOM proof artifacts |
| Completed-prompt screenshot proof | Real | Live WebUI proof now requires a completed human-readable response in the screenshot DOM |
| OpenCode parity | In progress | Source tracking lives in `OPENCODE-PARITY.md` |
| Conversation storage | Partial | Active conversations are in-memory; snapshots persist JSON files |
| Task controls | API-shaped | Endpoints exist, but operational interruption inside active model/tool loops needs proof/fixes |
| Benchmark adapter | Shallow | Capability summary only; not yet the full artifact-backed adapter used by Superapp |
| CI | Real | CI, Build Proof, and Live WebUI Feature Sprint are active PR gates |

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

To run the current browser/WebUI proof locally:

```bash
bash scripts/smoke/live-webui-feature-sprint.sh
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
- File, web, command, terminal, task, repo, patch proposal, safe apply_patch surface, browser, vision, and graph tool dispatch
- Browser proof via headless Chrome/Chromium screenshot and optional DOM capture
- NIM vision review for screenshots
- Graph visualization endpoint
- CI file-size gate and Rust build/test/lint workflow scaffold
- Live WebUI feature-sprint smoke with completed-prompt screenshot proof

## Main gaps before it can compete with the existing apps

1. Finish full OpenCode `apply_patch` behavior.
2. Copy OpenCode system/prompt behavior into the orchestrator instead of maintaining custom approximations.
3. Add durable conversation persistence, not only in-memory conversations plus manual snapshots.
4. Make task controls affect active model/tool loops.
5. Add visible model routing receipts, fallback events, and saved provider/model order UI.
6. Port mature commit-readiness and pre-push gates from LocalGPT/Superapp.
7. Replace the shallow benchmark report with artifact-backed Benchmark Adapter API v1.
8. Add first-class tool cards with live running/completed/failed state.
9. Add context compaction parity.

## Relationship to the older repos

- `rust-ai-superapp` is still the most feature-rich and benchmark/proof-ready line.
- `localgpt-cockpit-rust` is still more usable as a mature chat/product app.
- `forgestack` still has more mature UI/proof/vision workflows.
- `forge-unified` is smaller and cleaner, so it is a strong consolidation target once missing product layers are ported.
