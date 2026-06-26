# Feature Audit — Forge Unified

Audit date: 2026-06-26
Repo: `organicoverlords/forge-unified`
Branch updated by API: `docs/api-reality-check-20260626`
Source branch inspected: `master`
Primary purpose: honest, tracked product scope and feature status for agents and humans.

This audit intentionally separates **real code**, **scaffold**, **partial implementation**, and **missing product work**. Do not mark a feature DONE unless code exists and the expected product behavior is proven or directly inspectable.

---

## 1. Product intent

- **Product shape**: chat-first coding-agent app.
- **Runtime shape**: standalone Rust binary for MVP.
- **Provider/model policy**: multi-provider support with visible, honest routing and fallback. Current code has explicit NVIDIA NIM support plus generic OpenAI-compatible support.
- **Memory policy**: no hidden memory writes. Persistence must be visible as conversation files, snapshots, run records, or proof artifacts.
- **Current role**: clean consolidation base, not yet a mature replacement for `localgpt-cockpit-rust`, `forgestack`, or `rust-ai-superapp`.
- **Out of scope for MVP**: multi-user auth, speech/video, no-code agent builder, marketplace plugins.

---

## 2. Status definitions

| Status | Meaning |
|--------|---------|
| DONE | Implemented and directly inspectable/proven |
| PARTIAL | Some real implementation exists, but important behavior, integration, proof, or UI is missing |
| SCAFFOLD | Route/type/module exists, but product behavior is mostly placeholder |
| MISSING | No sufficient implementation found |
| UNPROVEN | Claimed or likely present, but not proven by code/API inspection |
| REJECTED | Prior claim contradicted by code/manual proof |

---

## 3. Reality summary

| Area | Status | Reality |
|------|--------|---------|
| Rust workspace | DONE | Workspace has `crates/engine`, `crates/webui`, `crates/app`, `crates/unifiedgraph`. |
| CLI/server binary | DONE | `forge` starts an Axum server using loaded config and `Agent`. |
| REST API | DONE/PARTIAL | Routes exist for health, conversations, chat, cancel/pause/resume, snapshot, browser proof, vision review, benchmark, graph. |
| Chat frontend | MISSING | No bundled chat-first HTML/JS frontend found. |
| WebSocket chat | SCAFFOLD | `/ws` handler currently echoes text; not real streaming chat. |
| Provider abstraction | DONE | Provider trait exposes `chat`, `chat_stream`, and `health_check`. |
| Dedicated providers | PARTIAL | `nvidia_nim` and generic `openai` modules exist. Groq/OpenRouter are not dedicated modules; they must use generic OpenAI-compatible config. |
| Provider fallback | PARTIAL | Priority fallback exists, but visible routing receipts, saved order UX, quota/degraded state, and strong E2E proof are missing. |
| Tool executor | DONE/PARTIAL | Many tools dispatch to real implementations. Safety, UX, timeout/cancel integration, and proof depth remain incomplete. |
| Browser proof | DONE/PARTIAL | Headless Chrome screenshot + optional DOM dump exists. Console/network capture missing. |
| Vision review | DONE/PARTIAL | NIM vision request exists; provider-specific, key-dependent, and not deeply integrated into benchmark gates. |
| Conversation persistence | PARTIAL | Active conversations are in-memory; snapshots persist JSON. Normal durable conversation store is missing. |
| Cancel/pause/resume | SCAFFOLD/PARTIAL | API methods update engine state; active run-loop/tool-loop interruption is not proven. |
| Benchmark adapter | PARTIAL | Shallow capability score exists; full artifact-backed Benchmark Adapter API v1 is missing. |
| CI | PARTIAL | Workflow exists, but branch filter targets `main`/`dev` while default branch is `master`. |

---

## 4. Core feature matrix

### 4.1 Chat and conversation

| Feature | Pri | Status | Evidence / current state | Missing work |
|---------|-----|--------|--------------------------|--------------|
| Multi-turn conversation objects | P0 | DONE | `Conversation` and `Message` types exist; manager stores messages. | Durable normal storage. |
| Conversation create/list/get/delete API | P0 | DONE | REST routes and manager methods exist. | Browser proof against UI once frontend exists. |
| Streaming token-by-token provider path | P0 | PARTIAL | Provider trait and OpenAI-compatible SSE parser exist. | Real WebUI streaming path and E2E provider proof. |
| WebSocket chat | P0 | SCAFFOLD | `/ws` route exists. Handler echoes `echo: <text>`. | Replace with real chat/session/event protocol. |
| Chat frontend | P0 | MISSING | No normal chat UI assets found. | Build bundled HTML/JS/CSS chat interface. |
| Markdown rendering | P1 | MISSING | No chat frontend yet. | Renderer, code blocks, copy buttons. |
| Conversation title/rename | P2 | MISSING | Create takes title, but rename endpoint not found. | Rename API + UI. |
| Message editing/regeneration | P2 | MISSING | No implementation found. | API + UI + state semantics. |
| Export/import conversation | P2 | MISSING | Snapshot save/load exists, export/import API absent. | Explicit export/import route and UI. |
| Snapshots | P1 | DONE | Snapshot manager writes/loads conversation JSON. | Route for restore/list/delete if needed. |
| Temporary/ephemeral chats | P2 | MISSING | Not found. | Explicit non-persistent mode. |
| Cancel generation | P0 | PARTIAL | Route and state mutation exist. | Run loop must check cancellation during model/tool work. |
| Pause/resume generation | P1 | SCAFFOLD/PARTIAL | Routes and state mutation exist. | Real pause/resume behavior inside execution loops. |

### 4.2 Agent and tool execution

| Feature | Pri | Status | Evidence / current state | Missing work |
|---------|-----|--------|--------------------------|--------------|
| Agent loop | P0 | DONE/PARTIAL | Orchestrator loops model → tool calls → tool results. | More robust state, events, cancellation, repair, receipts. |
| File read/write/edit/delete | P0 | DONE | Tool kinds and executor dispatch exist. | Proof and safety expansion. |
| File list/glob/search | P0 | DONE | Tool definitions and executor dispatch exist. | Proof and output compaction. |
| Web fetch | P1 | DONE | `execute_web_fetch` uses reqwest. | Network policy and output limits. |
| Web search | P2 | PARTIAL | DuckDuckGo HTML scraping implementation exists. | Robust search API or honest capability gap. |
| Shell command execution | P1 | DONE/PARTIAL | Shell tool exists. | Strong destructive-command policy and sandboxing. |
| Terminal run | P2 | DONE/PARTIAL | Terminal tool exists. | True PTY behavior/proof if required. |
| Task delegation | P2 | PARTIAL | Task tool exists. | Real subagent lifecycle/UI/scope. |
| Batch parallel execution | P0 | DONE/PARTIAL | `buffer_unordered(max_parallel)` used for batch execution. | Dependency-aware strategy, cancellation, compaction, UI cards. |
| Tool timeout handling | P0 | PARTIAL | Executor has timeout config, some tools use request/client timeouts. | Enforce consistently for all tools. |
| Tool approval gates | P0 | PARTIAL | Approval mode/safety checker exists. | User-visible approval workflow and policy proof. |
| Propose patch | P2 | PARTIAL | Tool kind/definition/dispatch exists. | Real patch application/review flow. |
| Mode switching | P1 | PARTIAL | `AgentMode` and switch-mode tool exist. | End-to-end plan/build/read-only behavior. |
| Browser proof tool | P0 | DONE/PARTIAL | Chrome screenshot and DOM dump exist. | Console/network capture, artifact paths, benchmark integration. |
| Vision review tool | P0 | DONE/PARTIAL | NIM vision request exists. | Provider fallback, verdict schema, benchmark gates. |
| Graph build/query | P1 | DONE/PARTIAL | Unified graph tools and graph route exist. | More proof and UX integration. |

### 4.3 Model and provider

| Feature | Pri | Status | Evidence / current state | Missing work |
|---------|-----|--------|--------------------------|--------------|
| Provider trait | P0 | DONE | `Provider` trait has chat, stream, health. | — |
| NVIDIA NIM provider | P0 | DONE/PARTIAL | Explicit `nvidia_nim` module and default config exist. | Current model list, live proof, flakiness handling. |
| OpenAI-compatible provider | P0 | DONE/PARTIAL | Generic OpenAI-compatible client exists. | Provider-specific quirks and tests. |
| Groq/OpenRouter support | P1 | PARTIAL | No dedicated modules found; can likely use generic OpenAI-compatible config. | Config examples, UI, proof. |
| Provider priority order | P0 | PARTIAL | Config sorts enabled providers by priority. | Visible order UI, persisted edits, receipts. |
| Provider retry logic | P1 | PARTIAL | Provider configs include `max_retries`, router tries providers. | Explicit retry/backoff/degraded handling. |
| Router with failover | P0 | PARTIAL | Basic fallback chain exists. | Visible fallback events and no-hidden-swap proof. |
| Model selector/config | P1 | PARTIAL | Model config structs exist. | UI/API for selection and per-conversation persistence. |
| Streaming support | P0 | PARTIAL | Provider streaming parser exists. | Real WebUI stream and tool-call streaming correctness. |
| Key management | P1 | PARTIAL | API keys read from env vars. | UI/secrets file, masking, rotation/test. |
| Temperature/top_p | P2 | PARTIAL | Request has temperature; top_p absent. | UI/API/storage and provider payload proof. |
| Local model support | P2 | MISSING | No Ollama/llama.cpp/LM Studio provider found. | Add local provider path if desired. |
| Model capability matrix | P0 | PARTIAL | `model_caps` module exists. | Formal matrix comparable to LocalGPT/Superapp. |

### 4.4 Code and git

| Feature | Pri | Status | Evidence / current state | Missing work |
|---------|-----|--------|--------------------------|--------------|
| Workspace-root guard | P0 | PARTIAL | File/tool safety code exists. | Prove all write/exec tools respect scope. |
| Git status/diff/log | P1 | MISSING/PARTIAL | Shell can run git, but no dedicated git API found. | Dedicated git service/tool if needed. |
| Git commit/push | P1 | MISSING/PARTIAL | Shell can run git. | Commit readiness and push gate. |
| Worktree support | P1 | MISSING | No worktree feature found in inspected files. | Port from LocalGPT/ForgeStack if required. |
| Diff viewer | P1 | MISSING | No chat frontend. | UI plus patch summary. |
| Commit readiness analysis | P1 | MISSING | Not found. | Port from LocalGPT/Superapp. |
| Pre-push CI gate | P1 | MISSING | GitHub Actions exists; no local readiness hook found. | Local hook + readiness endpoint. |

### 4.5 WebUI and UX

| Feature | Pri | Status | Evidence / current state | Missing work |
|---------|-----|--------|--------------------------|--------------|
| Health endpoint | P0 | DONE | `GET /api/health`. | Include git/head/runtime identity if desired. |
| REST API | P0 | DONE/PARTIAL | Main API routes exist. | Complete missing product semantics. |
| WebSocket endpoint | P0 | SCAFFOLD | Echo handler only. | Real event protocol. |
| CORS support | P1 | DONE | Any origin/method/header configured. | Harden later. |
| Frontend HTML/JS assets | P0 | MISSING | Main chat frontend absent. | Build UI. |
| Graph visualization | P1 | DONE/PARTIAL | Embedded graph page exists. | Connect to normal app navigation. |
| Model selector dropdown | P0 | MISSING | No chat UI. | Port from Superapp/LocalGPT. |
| Tool cards | P0 | MISSING | No chat UI. | Running/completed/failed cards. |
| Debug/telemetry panel | P1 | MISSING | Not found. | Event log, model attempts, timings. |
| Theme/responsive layout | P2 | MISSING | No chat UI. | Dark/light, mobile. |

### 4.6 Benchmarking and proof

| Feature | Pri | Status | Evidence / current state | Missing work |
|---------|-----|--------|--------------------------|--------------|
| Benchmark capability report | P1 | PARTIAL | `/api/benchmark` returns score/capabilities. | Make honest/artifact-backed. |
| Benchmark Adapter API v1 | P0 | MISSING | Full run/readiness/events/artifacts/result API not found. | Port from Superapp. |
| Browser artifact proof | P0 | PARTIAL | Browser proof tool returns screenshot base64. | Store artifacts, expose paths, integrate gates. |
| Vision verdict proof | P0 | PARTIAL | NIM analysis/verdict heuristic exists. | Strict verdict JSON/gap codes. |
| Natural prompt corpus | P0 | MISSING | Not found. | Add prompts and runner. |
| Daily-use benchmark | P0 | MISSING | Not found. | Launch app, use normal UI, screenshots, console capture. |
| Cross-app comparison readiness | P0 | MISSING | Not found. | Standard schema compatible with existing app benchmark. |

### 4.7 CI/CD

| Feature | Pri | Status | Evidence / current state | Missing work |
|---------|-----|--------|--------------------------|--------------|
| Format check | P1 | DONE | Workflow runs `cargo fmt --all -- --check`. | Ensure branch trigger runs. |
| Clippy | P1 | DONE | Workflow runs clippy with warnings denied. | Ensure branch trigger runs. |
| Build check | P0 | DONE | Workflow runs `cargo check` and release build. | Ensure branch trigger runs. |
| Test suite | P0 | DONE/PARTIAL | Workflow runs cargo tests. No tests found by API search. | Add meaningful tests. |
| File size gate | P1 | DONE | Max 500-line Rust file check exists. | Ensure branch trigger runs. |
| Security audit | P2 | DONE/PARTIAL | Workflow installs/runs cargo-audit. | Runtime cost and lockfile proof. |
| License check | P2 | DONE/PARTIAL | Workflow installs/runs cargo-deny. | Keep deny config current. |
| Branch filters | P0 | REJECTED | Workflow targets `main` and `dev`; repo default is `master`. | Add `master` or change default branch. |

---

## 5. Comparison snapshot

| Repo | Current role | Compared with Forge Unified |
|------|--------------|-----------------------------|
| `rust-ai-superapp` | Most feature-rich and benchmark/proof-ready line | Far ahead in routing telemetry, benchmark adapter, prompt CI, debug panel, vision proof, hooks, natural prompt corpus. |
| `localgpt-cockpit-rust` | More usable chat/product app | Far ahead in chat UI, conversation UX, provider settings, model capability matrix, commit readiness. |
| `forgestack` | More mature UI/proof/vision workflow | Ahead in tool cards, model order UX, subagent visibility, baseline QA, vision gate. |
| `forge-unified` | Clean consolidation base | Smaller, clearer, easier to reason about; not yet product-complete. |

Approximate status from API/code inspection:

```text
forge-unified:          4/10 as product, 7/10 as clean foundation
localgpt-cockpit-rust:  7.5/10 as product
forgestack:             7/10 as product
rust-ai-superapp:       8.5/10 as product
```

---

## 6. Highest-leverage next work

1. **Fix CI branch trigger**: include `master` or move default branch to `dev/main` intentionally.
2. **Add a minimal real chat frontend**: conversation list, composer, streaming area, tool/result cards placeholder.
3. **Replace WebSocket echo** with a real event protocol or use SSE consistently.
4. **Add durable conversation persistence**: JSONL or per-conversation JSON store loaded on startup.
5. **Make cancel/pause/resume honest**: either implement real checks in run/tool loops or report capability gaps.
6. **Port visible router receipts** from Superapp: model card, fallback/swap events, provider order source.
7. **Port commit readiness** from LocalGPT/Superapp: dirty ownership, secrets scan, build/test/proof gates.
8. **Replace benchmark adapter** with artifact-backed Benchmark Adapter API v1.
9. **Add smoke proof scripts** for health, conversation chat, browser proof, vision review, model fallback, and graph route.

---

## 7. Claim rules for agents

- Do not claim “WebSocket chat DONE” until `/ws` performs real chat/event streaming instead of echo.
- Do not claim “frontend DONE” until a normal chat-first UI is bundled and browser-smoked.
- Do not claim “conversation persistence DONE” for normal chats until conversations survive restart without manually saving snapshots.
- Do not claim “pause/resume DONE” until long-running model/tool work actually pauses and resumes.
- Do not claim “Benchmark Adapter API v1 DONE” until readiness/run/events/artifacts/result/cancel endpoints exist and are smoke-tested.
- Do not claim “Groq/OpenRouter dedicated support DONE” unless provider-specific config/tests or explicit generic OpenAI-compatible examples are added.
- Do not claim CI is protecting the default branch until the workflow branch filter covers the repo default branch.
