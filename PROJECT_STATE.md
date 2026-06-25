# Rust AI Superapp — Current State

Updated: 2026-06-24 11:10:00 UTC

## Current verified baseline

- Repo: `/root/work/rust-ai-superapp`
- Branch: `dev`
- HEAD: `617eaa3` — fix: use clean system prompt for chat-only mode, add live thinking trace
- Server port: `4891`
- Runtime state: `.superapp/**`
- Runtime secrets: `.superapp/secrets/providers.env` (not tracked by git)
- Server binary: `target/debug/superapp-server`

## Features implemented

### Chat & Conversation
- Multi-turn conversation threads with JSONL persistence
- Streaming token-by-token (reqwest + tokio SSE)
- Markdown rendering with code syntax highlighting
- Message editing, deletion, regenerate
- Conversation create / rename / delete / archive / export / import
- Conversation list with search/filter, sort by updated_at
- Conversation fork / branch from message point
- Ephemeral / temporary chats
- Token count per message
- Typing / thinking indicator, auto-scroll
- Cancel / stop generation
- Responsive layout, keyboard navigation

### Agent & Tool Execution
- Tool calling: file edit, bash, search (glob, grep, read_file, search_text, repo_scan)
- Animated tool call cards with expandable results
- Parallel tool execution (read-only tools run concurrently)
- Synthetic `parallel_tool_calls` batch tool (for models limited to 1 tool call/turn)
- **`tool_choice` parameter support** — force model to emit tool calls via `tool_choice: required` per-request; `openai_compatible_payload_with_messages` accepts override; SSE handler extracts from body and passes through tool loop; verified with NVIDIA NIM gpt-oss-120b (emits tool_calls with finish_reason: tool_calls)
- Tool approval gates with permission system
- File change summary endpoint
- Diff viewer with line-by-line highlighting
- Tool auto-retry on transient failures
- Tool timeout handling (60s)
- Destructive command detection
- Agent goals (domain type, CRUD, UI indicator)
- Todo list (user + model visible, 6 statuses, nested, reorder)
- Agent loops (stop conditions, failure classification, no-repeat guard, verification step, human checkpoint)
- Agent compaction (LLM-based context summarization)
- Subagent delegation (basic, with AgentTaskStore)
- Run ledger with ToolRunRecord persistence

### Model & Provider
- Multi-provider support: NVIDIA NIM, Groq, OpenRouter, GitHub Models, OpenAI-compatible
- Provider order / priority management with drag-and-drop
- Provider failover with automatic routing
- Provider key management with round-trip protection
- Provider availability detection
- Model selector dropdown
- Provider fallback notification

### Code & Git
- File create / edit / replace / delete
- Git status, diff, log display
- Git commit + push via bash tool
- File scope restrictions (path sanitization)
- Worktree management
- Source-map scanner (recursive directory walk)

### UI/UX
- Dark + Light theme with localStorage persistence
- Responsive / mobile layout
- Keyboard shortcuts (Enter=send, Shift+Enter=newline, Escape=cancel)
- Toast notification system
- Error surfacing with actionable messages
- Loading states, agent status indicators
- Sidebar with conversation list
- File attachments and drag-and-drop into chat
- Copy code block / message buttons

### System Prompt Architecture
- Opencode-style structured system prompt with `# Tone and style`, `# Tool usage`, `# Following conventions`, `# Code references` sections
- Dynamic environment block (`<env>`) injected per-request with working directory, git status, platform, date
- Model identity prepended: "You are powered by the model named {model}. The exact model ID is {provider}/{model}."
- Chat-only mode: clean conversational prompt without tool definitions when `tool_choice` is empty
- Escape hatch: `.superapp/system_prompt` file overrides the default tool prompt

### Live Orchestration Trace
- `emit_trace()` SSE events at 3 phases: `generating`, `running_tool`, `generating_final`
- Trace card updates live in WebUI via `replaceWith` instead of static "Thinking" card
- Shows phase, round, duration, correlation ID, origin, rate-limit count
- Rate-limit count (`🚦 N×429`) shown in meta bar and detail body

### Model Badge
- `message_done` SSE event includes `attempts` + `message_id`
- WebUI renders `(provider/model)` badge on assistant messages

### Loops & Orchestration (RALPH-Style)
- Explicit loop state with LoopRecord, LoopStore
- Stop condition (MaxIterations, TimeoutSecs, FirstFailure, etc.)
- Failure classification (ToolError, ValidationFailed, PermissionDenied, etc.)
- No-repeat failure guard with blocked_actions
- Verification step before marking done
- Human checkpoint (pause/resume/cancel/redirect)
- Stop whole workflow (cancel handler)
- Conflict detection for parallel file edits

### 100+ Tool Call Loop Stress Test
- `stress_test_handler.rs` — discovers real project files, emits 100+ `tool_start`/`tool_result` SSE events
- `scripts/smoke/browser-proof-100-tool-calls.js` — Playwright browser proof script
- Verifies: ≥100 tool_call events, all tool_result with ok:true, zero errors
- Runs against WebUI via `/api/messages/stream?tool=stress_test_tool_loop`

### Waiting-for-User State
- `PendingUserAction` domain type with id, type, prompt, options, pending/resolved/cancelled status
- `PendingUserActionStore` with JSON persistence in `.superapp/pending_actions/`
- `waiting_for_user` SSE event emitted for high-risk permission requests and patch loop approvals
- `GET /api/pending-actions` and `POST /api/pending-actions/:id/resolve` API endpoints
- Frontend: `makeWaitingCard()` renders waiting card with approve/deny buttons
- 8 new tests (4 core + 4 store)

### Goals
- Goal domain type with Active/Achieved/Cleared/Blocked/Failed statuses
- POST/GET /api/goals CRUD, persisted as JSON
- Sidebar goal indicator with colored dot
- Slash commands: /goal add / list / update

### Todos
- TodoItem with 6 statuses (Queued/Running/Done/Blocked/Skipped/Failed)
- User-visible todo card with collapse/expand
- Model-visible via manage_todo tool
- Owner, proof, blocker fields
- Reorder support

## Architecture

```
superapp-core    — Domain structs, enums, tools, diff viewer, change summary
superapp-store   — JSONL+artifact persistence (ChatStore, ToolRunStore, AgentTaskStore, etc.)
superapp-server  — SSE route handlers, tool loop, provider routing, streaming
superapp-web     — Static JS chat UI with ES modules (render.js, stream.js, state.js)
superapp-cli     — Doctor, validate, provider-status, prompt commands
superapp-agent   — Parallel tool runner, subagent logic
superapp-bench   — Live provider benchmark tool
```

## Test status

- **All server tests pass** (255+ cargo tests)
- **18/18 browser-proof.js** — UI element checks + SSE stream + API endpoints
- **16/16 e2e-webui-browser-proof.js** — full WebUI E2E with mode toggle, tool card checks
- `cargo build --workspace` — clean

## Current gaps (highest priority)

| Area | Feature | Priority |
|---|---|---|
| External | Visible prompt handoff, dispatch approval | P1 |
| External | Target repo proof, permission profile | P1 |
| Subagents | Subagent tool permissions | P1 |
| Loops | Repair plan | P2 |
| Loops | Loop iteration counter | P2 |
| Model | Temperature/top_p control | P2 |
| Model | Local model support | P2 |
| CI | Pre-push CI gate | P1 |
| Hooks | Commit hook, security hook | P1 |
| Cost | Usage budget warning, budget hard stop | P1 |

## Best next sequence

1. **External agent dispatch** — visible prompt handoff with approval, target repo proof, permission profile
2. **Subagent tool permissions** — each subagent has explicit allowed/denied tools
3. **NVIDIA NIM API key rotation** — current key hitting 429; generate a fresh key to unblock testing
4. **Repair plan for loops** — record strategy change before retry

## What not to do

- Do not grow `crates/superapp-server/src/main.rs` as a monolith (currently well-modularized at ~190 lines)
- Exception: `crates/superapp-server/src/benchmark_adapter.rs` (~793 lines) contains the full §7 benchmark adapter API v1 including session management, event streams, artifact tracking, readiness validation, control API, and 19 unit tests. Extract into sub-modules if extending beyond 850 lines.
- Do not commit `.superapp/**` or provider secrets
- Do not overbuild orchestration before the basic loop works
- Do not build multi-user/auth before core single-user flow is solid
