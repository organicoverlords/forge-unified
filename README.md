# Forge Unified — Rust AI Superapp

A standalone Rust AI application with free LLM routing, tool execution, and a chat-first WebUI.

**No Docker/Mongo/Redis/Node/Python runtime required.** Single binary, zero API keys needed for MVP (MockProvider included).

## Architecture

```
crates/
  engine/    — Core engine: orchestration, tool execution, provider routing, conversation management
  webui/     — Axum HTTP/WS server with REST API and WebSocket chat
  app/       — CLI binary entrypoint
```

## Quick Start

```bash
cargo run --release -- --help
cargo run --release           # starts on http://0.0.0.0:3000
```

## Features

- Multi-provider LLM routing (OpenAI-compatible, NVIDIA NIM, Groq, OpenRouter)
- Streaming chat with SSE
- Tool execution (file ops, web fetch, shell, terminal, batch parallel)
- Conversation management (CRUD, snapshots, history)
- Benchmark adapter API
- WebSocket real-time chat
- Provider priority/fallback configuration
- File size gate CI (max 500 lines per file)

For product scope, feature status, proof requirements, orchestration rules, and benchmark contracts, read `FEATURE-AUDIT.md`.
