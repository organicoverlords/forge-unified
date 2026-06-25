# Forge Unified — Current State

Updated: 2026-06-25

## Current verified baseline

- Repo: `C:\Users\Lauri\Desktop\forge-unified`
- Branch: `master`
- HEAD: `36c7590`
- Server port: `3000`

## What's new (2026-06-25)

- **Browser Proof tool**: Headless Chrome screenshot capture, DOM snapshot, page title extraction. API: `POST /api/browser-proof`. Detects Chrome/Chromium path automatically.
- **Vision Review tool**: Screenshot analysis via NVIDIA NIM vision models (Llama 3.2 11B Vision). API: `POST /api/vision-review`. Extracts pass/fail verdict from analysis text.
- Both tools available as ToolKind variants (`BrowserProof`, `VisionReview`) wired into the tool executor, safety checker, and agent API.
- **Tool definitions sent to LLM**: Orchestrator now passes full tool schemas (18 tools) to providers with `tool_choice: "auto"`. Models can now call any tool through chat, including browser_proof and vision_review.
- **NIM provider tool support**: NVIDIA NIM provider now sends tool definitions in requests and parses tool_calls from responses (was sending `tools: None`).
- **OpenAI provider tool_calls fix**: OpenAI provider now properly returns parsed tool calls in ChatResponse instead of discarding them.
- **Provider tool call normalization**: New `tool_kind_from_name()` and `tool_calls_from_deltas()` helpers in provider.rs.

## Features implemented

### Chat & Conversation
- Multi-turn conversation threads with persistence
- Streaming token-by-token (reqwest + tokio SSE)
- Conversation create / list / get / delete
- Chat completion via REST API
- Cancel / pause / resume generation
- Message history with metadata

### Agent & Tool Execution
- Tool calling: file read/write/edit/delete/list/glob/search, web fetch/search, shell, terminal, task, batch parallel, repo info, propose patch, switch mode
- Parallel tool execution via `futures::stream::buffer_unordered`
- Batch parallel tool execution
- Tool timeout handling (60s default)
- Tool approval gates (Ask/Full/ReadOnly/Blocked modes)
- Tool call ledgers with RunRecord persistence

### Model & Provider
- Multi-provider support: NVIDIA NIM, Groq, OpenRouter, OpenAI-compatible
- Provider priority ordering with configurable fallback
- Provider key management via env vars
- Retry logic with configurable max_retries
- MockProvider for zero-config development

### Architecture
```
crates/
  engine/    — Core domain types, tools, providers, router, orchestrator
  webui/     — Axum server with REST API + WebSocket
  app/       — CLI binary with clap arg parsing
```

### CI/CD
- GitHub Actions: check (fmt, clippy, build, release), test, file size gate (500 lines), smoke test, security audit, cargo-deny
- File size gate enforced per-Rust-file

## Test status

- `cargo check --workspace` — compiles with ~47 warnings
- Tests: not yet run

## Current gaps (highest priority)

| Area | Feature | Priority |
|------|---------|----------|
| Engine | Streaming SSE from providers | P1 |
| Engine | Provider failover with automatic routing | P1 |
| Engine | Context compaction | P2 |
| WebUI | Frontend HTML/JS assets | P1 |
| WebUI | WebSocket chat client | P1 |
| WebUI | Markdown rendering | P2 |
| Docs | Documentation alignment with actual code | P1 |

## What not to do

- Do not commit `.superapp/**` or provider secrets
- Do not overbuild orchestration before the basic loop works
- Do not build multi-user/auth before core single-user flow is solid
