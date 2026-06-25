# Feature Audit — Forge Unified

Audit date: 2026-06-25
Repo: `forge-unified`
Branch: `master`
Primary purpose: one canonical, tracked feature audit that agents and humans must read and update.

---

## 1. Product Intent

- **Product shape**: chat-first coding-agent app. The main UX is conversation.
- **Runtime shape**: standalone binary for MVP. No Docker/Mongo/Redis/Node/Python runtime dependencies.
- **Provider/model policy**: multi-provider support with configurable priority ordering. OpenAI-compatible, NVIDIA NIM, Groq, OpenRouter.
- **Memory policy**: no hidden memory writes. Persistence via conversation store and run records.
- **Out of scope for MVP**: multi-user auth, speech/video, no-code agent builder.

## 2. Status Definitions

| Status   | Meaning |
|----------|---------|
| DONE     | Implemented and compiles |
| PARTIAL  | Some implementation exists, but incomplete |
| MISSING  | No sufficient implementation found |
| BLOCKED  | Cannot proceed without missing prerequisite |

## 3. Core Feature Matrix

### 3.1 Chat and Conversation

| Feature | Pri | Status | Notes |
|---------|-----|--------|-------|
| Multi-turn conversation threads | P0 | DONE | Vec<Message> in-memory + persistence ready |
| Streaming token-by-token | P0 | PARTIAL | SSE streaming path exists, provider integration partial |
| Conversation create/list/get/delete | P0 | DONE | REST API endpoints |
| Cancel/stop generation | P0 | DONE | Cancel endpoint with token check |
| Pause/resume generation | P1 | DONE | Pause/resume endpoints |
| Conversation snapshots | P1 | DONE | Snapshot save/restore API |
| Conversation title/rename | P2 | MISSING | Not implemented |
| Message editing/regeneration | P2 | MISSING | Not implemented |
| Export/import conversation | P2 | MISSING | Not implemented |
| Markdown rendering | P2 | MISSING | No frontend yet |
| Temporary/ephemeral chats | P2 | MISSING | Not implemented |

### 3.2 Agent and Tool Execution

| Feature | Pri | Status | Notes |
|---------|-----|--------|-------|
| File read/write/edit/delete | P0 | DONE | Tool executor with workspace-root guard |
| File list/glob/search | P0 | DONE | Directory ops with glob support |
| Web fetch | P1 | DONE | HTTP fetch via reqwest |
| Web search | P2 | MISSING | Not implemented |
| Shell command execution | P1 | DONE | Blocking shell with output capture |
| Terminal run | P2 | DONE | PTY-style terminal execution |
| Task delegation | P2 | DONE | Task ops for sub-agent style delegation |
| Batch parallel execution | P0 | DONE | `buffer_unordered` with configurable concurrency |
| Tool timeout handling | P0 | DONE | Configurable per-provider timeout |
| Tool approval gates | P0 | DONE | Ask/Full/ReadOnly/Blocked modes |
| Propose patch | P2 | DONE | Patch proposal tool |
| Mode switching | P0 | DONE | Chat/Explore/Plan/Build modes |
| Repo info | P1 | DONE | Repository metadata tool |

### 3.3 Model and Provider

| Feature | Pri | Status | Notes |
|---------|-----|--------|-------|
| Multi-provider support | P0 | DONE | Provider trait with OpenAI, NIM, Groq, OpenRouter impls |
| Provider priority order | P0 | DONE | Priority-based fallback chain |
| Provider retry logic | P1 | DONE | Configurable max_retries per provider |
| MockProvider for dev | P0 | DONE | Zero-config development provider |
| Router with failover | P0 | DONE | Try providers in priority order on failure |
| Model selector/config | P1 | PARTIAL | ModelConfig exists, per-conversation model selection |
| Streaming support | P0 | PARTIAL | Provider trait supports streaming, provider impls partial |
| Key management | P1 | DONE | API keys from env vars |
| Temperature/top_p | P2 | MISSING | Not implemented |
| Local model support | P2 | MISSING | Not implemented |

### 3.4 Code and Git

| Feature | Pri | Status | Notes |
|---------|-----|--------|-------|
| Workspace-root guard | P0 | DONE | All file ops constrained to workspace |
| File size gate CI | P1 | DONE | GitHub Actions check: max 500 lines |
| Git integration | P2 | MISSING | Not implemented |

### 3.5 WebUI

| Feature | Pri | Status | Notes |
|---------|-----|--------|-------|
| Health endpoint | P0 | DONE | GET /api/health |
| REST API | P0 | DONE | Conversations CRUD, chat, benchmark |
| WebSocket endpoint | P0 | DONE | /ws endpoint with state sharing |
| CORS support | P0 | DONE | Any origin any method |
| Frontend HTML/JS assets | P1 | MISSING | Not yet bundled |
| Benchmark adapter API | P1 | PARTIAL | GET /api/benchmark exists |

### 3.6 CI/CD

| Feature | Pri | Status | Notes |
|---------|-----|--------|-------|
| Format check | P1 | DONE | cargo fmt --check |
| Clippy | P1 | DONE | cargo clippy --workspace |
| Build check | P0 | DONE | cargo check |
| Release build | P0 | DONE | cargo build --release |
| Test suite | P0 | DONE | cargo test |
| File size gate | P1 | DONE | Max 500 lines per .rs file |
| Security audit | P2 | DONE | cargo audit |
| License check | P2 | DONE | cargo deny check licenses |

### 3.7 Browser Proof and Vision

| Feature | Pri | Status | Notes |
|---------|-----|--------|-------|
| Browser proof tool | P0 | DONE | `ToolKind::BrowserProof` — headless Chrome screenshot, DOM dump, page title. API at `POST /api/browser-proof`. |
| Vision review tool | P0 | DONE | `ToolKind::VisionReview` — sends screenshot to NIM vision model (Llama 3.2 11B), returns analysis + pass/fail verdict. API at `POST /api/vision-review`. |
| Chrome auto-detection | P1 | DONE | Finds Chrome on Windows/macOS/Linux or via `CHROME_PATH` env var |
| Vision verdict extraction | P1 | DONE | Heuristic pass/fail extraction from analysis text |
| Console log capture | P2 | MISSING | Requires CDP (DevTools Protocol); future via chromiumoxide |

## 4. Current Gaps

| Priority | Feature | Blockers |
|----------|---------|----------|
| P1 | Frontend assets (HTML/JS) | Needs design |
| P1 | WebSocket chat client | Needs frontend |
| P1 | Streaming SSE from real providers | Provider integration testing |
| P1 | Provider failover E2E | End-to-end testing |
| P2 | Markdown rendering | Frontend work |
| P2 | Message editing/regeneration | API + frontend |
| P2 | Conversation export/import | API work |
