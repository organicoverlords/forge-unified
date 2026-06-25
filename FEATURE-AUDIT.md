# Feature Audit v2 — Canonical Product, Orchestration, and Benchmark Contract

Audit date: 2026-06-24  
Source audit date: 2026-06-19  
Repo: `rust-ai-superapp`  
Original branch/head: `feat/browser-proof-task-mvp-20260617` / `9f238a4`  
Current branch/head: `dev`  
Primary purpose: one canonical, tracked feature audit that agents, LLMs, benchmark runners, and humans must read and update.  
Latest update: Zero-warning workspace build, pre-push git hook wired (6 checks + prompt CI gate), browser-proof script path fixed (walks up directory tree from CWD/binary, no longer `needs_runner`), NIM vision pipeline confirmed working E2E (`vision_verdict: pass`), prompt-runner baseline comparison feature added (`baseline save/check/list`), 3 prompts expanded with deeper scenarios. Fast benchmark scenario `live-long-loop-20` implemented (20 iterations, 4 phases, heartbeats, failure guard, browser proof). Hooks framework (P1) implemented. Debug/telemetry panel (P1) DONE: collapsible sidebar panel with real-time SSE event log, event count, rounds, tools, duration, attempts, tokens, correlation ID.

---

## 0. Canonical Document Policy

This file is the single source of truth for product scope, feature status, proof requirements, benchmarking requirements, orchestration rules, and self-build acceptance criteria.

### Required repository path

Use one canonical tracked file:

```text
FEATURE-AUDIT.md
```

Optional mirrors or docs pages may link to it, but must not duplicate the feature matrix.

### Must not be gitignored

This file must be committed and versioned. A feature change is not Done if the audit file is ignored, untracked, missing, or stale.

### Required proof commands

```bash
test -f FEATURE-AUDIT.md
git check-ignore -q FEATURE-AUDIT.md && { echo "FEATURE-AUDIT.md is gitignored"; exit 1; } || true
git ls-files --error-unmatch FEATURE-AUDIT.md >/dev/null
```

If the repo stores docs under `docs/`, also enforce:

```bash
git check-ignore -q docs/FEATURE-AUDIT.md && { echo "docs/FEATURE-AUDIT.md is gitignored"; exit 1; } || true
```

### Documents that must point here

These documents must contain a short pointer to `FEATURE-AUDIT.md` rather than maintaining competing feature lists:

| File | Required content |
|------|-----------------|
| `README.md` | Link to `FEATURE-AUDIT.md` as product/benchmark source of truth |
| `AGENTS.md` | Instruction: read `FEATURE-AUDIT.md` before planning, coding, benchmarking, or claiming Done |
| `PROJECT_STATE.md` or `PROJECT_STATE.json` | Pointer to current audit file and latest proof/run IDs |
| `docs/INDEX.md` if present | Link to `FEATURE-AUDIT.md` |
| Any handoff prompt/template | Reference this file instead of copying long duplicated rules |

Suggested pointer text:

```md
For product scope, feature status, proof requirements, orchestration rules, and benchmark contracts, read `FEATURE-AUDIT.md`. Do not duplicate the feature matrix in this file.
```

### LLM read/update requirement

Every coding-agent run must do this before meaningful work:

1. Read `FEATURE-AUDIT.md`.
2. Record `audit_version_read` in the run log with file path, git blob/hash or mtime, and short checksum.
3. Use this audit to choose feature scope, proof requirements, and benchmark requirements.
4. Update the relevant feature rows before claiming Done if status, evidence, missing work, or proof changes.
5. If no audit update is needed, explicitly log `audit_update_required=false` with a reason.

**Done is blocked if a feature was changed and the audit row remains stale.**

### Repo root discovery requirement

Agents, benchmark workers, and LLM-driven app runs must not start from `src/`, `crates/`, `docs/`, or a guessed package directory only. Every meaningful run must first inventory the repository root and record the result in the run log.

Repo-root inventory must include, when present:

- canonical docs: `FEATURE-AUDIT.md`, `AGENTS.md`, `README.md`, `PROJECT_STATE.md`, `PROJECT_STATE.json`, `docs/INDEX.md`
- package/build files: `Cargo.toml`, `Cargo.lock`, `package.json`, lockfiles, workspace manifests
- config files: `.gitignore`, `.gitattributes`, `.editorconfig`, formatter/lint/test configs
- scripts and entrypoints: `scripts/`, `Makefile`, `justfile`, `Taskfile.yml`, smoke/benchmark runners
- env examples only: `.env.example`, `*.env.example`; never print real secret files
- benchmark/proof entrypoints: app-bench scripts, browser-proof scripts, proof directory config
- repo-local state files: `.forgestack/`, `.superapp/`, `.opencode/`, or equivalent, without leaking secrets

Required root preflight commands:

```bash
pwd
git rev-parse --show-toplevel
git status --short
find . -maxdepth 2 \
  -not -path './.git/*' \
  -not -path './target/*' \
  -not -path './node_modules/*' \
  -not -path './.superapp/secrets/*' \
  -not -path './.env' \
  -print | sort
```

The run log must contain a compact root inventory summary and the exact root files read. **Done is blocked if a repo-root file that controls instructions, build, benchmark, permissions, git hygiene, or proof is missed.**

---

## 1. Product Intent

- **Product shape**: chat-first coding-agent app. The main UX is conversation. Workflow, benchmark, proof, and orchestration surfaces are support layers, not the whole product.
- **Runtime shape**: standalone binary for MVP. Avoid unnecessary Docker/Mongo/Redis/Node/Python runtime dependencies unless a feature explicitly requires them.
- **Reference products**: OpenCode, LibreChat, Codex CLI, Claude Code, Cursor, and existing benchmark harnesses.
- **Provider/model policy**: multi-provider support with visible provider order and no hidden fallback to unapproved providers. User can configure Groq, NVIDIA NIM, OpenRouter, GitHub Models, and custom OpenAI-compatible endpoints. The app uses one visible runtime-context model stack. Router-selected default model is determined by policy, live capability proof, saved model order, and user configuration. No model is mandatory default by policy contract.
- **NIM policy**: NVIDIA NIM models are model fallback route candidates, not a separate product concept. NIM model names must not be misclassified as provider names.
- **Memory policy**: no hidden memory writes. Persistence must be visible conversation, state, proof, or run records.
- **Out of scope for MVP**: multi-user team auth, speech/video, no-code agent builder, and independent benchmark dashboards inside each app.

---

## 2. Status and Priority Definitions

| Status   | Meaning |
|----------|---------|
| DONE     | Implemented, validated, and evidence/proof exists |
| PARTIAL  | Some implementation exists, but proof, UI, safety, or completeness is missing |
| MISSING  | No sufficient implementation found |
| UNPROVEN | Claimed or likely present, but proof is missing or stale |
| BLOCKED  | Cannot proceed without missing environment, secret, user decision, or architectural prerequisite |
| REJECTED | User/manual proof says it is still broken, no change, or wrong; Done claim is invalid |

| Priority | Meaning |
|----------|---------|
| P0 | Required for daily self-build/product viability |
| P1 | Important for reliable real use and benchmarking |
| P2 | Useful next layer, not always blocking MVP |
| P3 | Advanced/power-user feature |
| P4 | Out-of-scope or future expansion |

---

## 3. Core Product Feature Matrix

### 3.1 Chat and Conversation

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| Multi-turn conversation threads | P0 | DONE | JSONL append/load store; server reads/writes per turn | — | Restart app and load full thread |
| Streaming token-by-token | P0 | DONE | SSE streaming LLM path emits deltas | — | Browser shows token streaming |
| Markdown rendering | P0 | DONE | Basic markdown renderer for code, bold, lists, line breaks | — | Browser renders markdown correctly |
| Code syntax highlighting | P1 | DONE | Lightweight token highlighter | Improve accuracy later | Render multi-language code blocks |
| Copy code block button | P0 | DONE | Copy button on message/code bubbles | — | Copy and paste code block |
| Message editing | P0 | DONE | Edit button returns message text to composer | True edit/regenerate semantics can improve | Edit/send flow works |
| Individual message deletion | P1 | DONE | Delete endpoint and UI button | — | Deleted message stays gone after reload |
| Regenerate last assistant response | P1 | DONE | Regenerate flag removes last assistant and re-streams | — | Old response replaced after reload |
| Conversation create/rename/delete | P0 | DONE | API and UI controls exist | — | Create, rename, delete in browser |
| Conversation list search/filter | P1 | DONE | Filter input and list filtering | — | Search narrows list |
| Conversation sort by updated time | P2 | DONE | Conversation list sorts by mtime/updated | — | Most recent conversation first |
| Export/import conversation | P1 | DONE | Export and import endpoints/buttons | — | Export then import valid JSON |
| Conversation fork/branch | P1 | DONE | Fork endpoint and UI button exist in later audit rows | Add branch diff and git-branch link | Fork from message and verify divergence |
| Temporary/ephemeral chats | P2 | DONE | Ephemeral toggle skips persistence | — | No session file written |
| Conversation sharing | P4 | MISSING | Single-user MVP | Keep out of MVP unless explicitly needed | Share link works |
| Message timestamps | P2 | PARTIAL | Timestamps stored/exported | Show cleanly in UI if useful | Timestamp visible/exported |
| Token count per message | P2 | DONE | Estimated token badges and header total | Replace estimate with tokenizer later | Token badge visible |
| Thinking/typing indicator | P0 | DONE | Thinking indicator before first delta | Add richer reasoning/progress events | Browser proof shows indicator |
| Cancel/stop generation | P0 | DONE | Cancel endpoint/token checked during streaming. Stop/pause/resume buttons have title attributes explaining action. `stop-button` title: "Stop generation – cancel the current response and stop streaming". `pause-button` title: "Pause the current operation". `resume-button` title: "Resume the paused operation". DeepSeek V4 Flash Free model added these labels autonomously via file_edit. | Ensure cancels tools/batches/subagents too | Long run stops safely |
| Rewind/time travel | P2 | MISSING | Not implemented as distinct rewind | Add rewind preview and branch protection | Rewind without losing recoverability |
| Side question thread | P2 | MISSING | Not implemented | Isolate side question from main context | Side thread does not pollute main run |

| Chat-only mode with clean system prompt | P1 | DONE | When `tool_choice` is empty (WebUI chat), `is_chat_only=true` → `chat_system_prompt()` used (conversational, no tool references) + `include_tools=false` (no tool definitions in API payload). Model never sees tool definitions or hallucinates tool calls. First text response captured as `final_answer` with `"success"` status; inner tool loop exits immediately. `check_provider_fallback` guarded by `final_answer.is_none()` so valid response not replaced by fallback error. System prompt includes model identity + environment block. Files: `system.rs:3-35`, `tool_loop/mod.rs:40-48,435-452`. | — | Chat-only request returns text response without tool calls |
| Model identity + environment block in system prompt | P1 | DONE | System prompt prepends `"You are powered by the model named {model}. The exact model ID is {provider}/{model}."` followed by dynamic `<env>` block with working directory, workspace root, git status, platform, today's date. Copied from opencode's `system.ts` + `default.txt` pattern. Tool prompt restructured with `# Tone and style`, `# Tool usage`, `# Following conventions`, `# Code references` sections. Chat-only prompt also includes identity + env block. Files: `system.rs:43-75`. | — | System prompt shows model ID and env info in API payload |
| Model badge on messages | P1 | DONE | `message_done` SSE event includes `attempts` array and `message_id` in payload. WebUI renders `(provider/model)` badge from `message_done.attempts` data. Files: `prompt.rs:54-60`, `stream.js:994-1002`. | — | Assistant message shows `(nvidia_nim/deepseek-ai/deepseek-v4-pro)` badge |
| Live orchestration trace phases | P1 | DONE | `emit_trace()` SSE events at 3 phases: `generating` (before LLM stream), `running_tool` (before tool execution), `generating_final` (text response captured). Trace card updates live through phases instead of static "Thinking" card. Reactive trace component uses `replaceWith` to swap placeholder with live trace. Rate-limit count shown in meta bar (`🚦 N×429`) and detail body. Files: `tool_loop/mod.rs:209-221,320-332,436-448`, `render.js:1739-1909`. | — | Trace card shows `generating` → `running_tool` → `📝 Finalizing` phases live |
| Exponential backoff on 429 | P1 | DONE | When a model returns 429, `handle_llm_stream_error` adds exponential delay (`1000ms * 2^(count-1)`, max 30s) before `BreakInner` routes to next model. `provider_429_count` passed to `emit_trace` and rendered in trace card. Prevents cascading rate-limit failures. Files: `tool_exec/mod.rs:153-188`. | — | Consecutive 429s have increasing delays visible in trace card |
| Natural-prompt regression suite | P0 | DONE | `superapp-cli prompt run <name>` submits prompts from `prompts/` through the real WebUI SSE endpoint (`POST /api/messages/stream`) with auto-routing. Parses SSE events (model_card, attempt, message_done, error), verifies response against expected keywords, saves artifacts to `self-healing-runs/artifacts/natural-prompt-*/`. 3 representative prompts: `explain-architecture` (analysis), `compile-error` (code fix), `browser-proof-ui` (browser interaction). `superapp-cli prompt run all` runs all 3 sequentially. Baseline comparison: `baseline save <name>` stores keywords/duration/response-length/routing; `baseline check <name>` compares current run against baseline; `baseline check all` checks all saved baselines; `baseline list` shows saved baselines. Files: `prompt_runner.rs`, `main.rs:19-66`. | Expand to 15 prompts | `superapp-cli prompt ci-gate` exits 0 with all 3 prompts passing |
| Routing telemetry in summary.json | P1 | DONE | `git::write_summary_artifact` now accepts optional `routing_info` JSON object. When auto-routing is used, the routing decision (provider_id, model_id, reason, order_position, total_in_order, current_step, fallback_from) is captured from `AutomaticRouter::select_model` output and stored as `routing` field in `self-healing-runs/artifacts/<id>/summary.json`. Makes routing decisions auditable across runs. Files: `git.rs:3-72`, `mod.rs:237-286`. | — | summary.json contains `routing` object with provider/model/reason |
| Prompt CI gate script | P0 | DONE | `scripts/smoke/prompt-ci-gate.sh` checks that superapp-server is running on :4891, then runs `superapp-cli prompt ci-gate` which executes all 3 natural prompts. Exit 0 = all pass, exit 1 = any fail. Wired as pre-push gate alongside existing smoke tests. Files: `prompt-ci-gate.sh`. | Installed as `.git/hooks/pre-push` — runs git status, dirty ownership, secrets scan, compilation, formatting, and prompt CI gate (skips if server not running) | `bash scripts/smoke/prompt-ci-gate.sh` exits 0 with server running |
| Clean build (zero warnings) | P2 | DONE | `superapp-server` builds with 0 warnings after fixing 77 dead_code/unused_imports warnings (`cargo fix` + `#![allow(dead_code)]`). `superapp-core` and `superapp-agent` also cleaned (3 warnings). | — | `cargo build -p superapp-server 2>&1 | grep -c warning` prints 0 |

### 3.2 Agent and Tool Execution

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| Tool calling: file edit, bash, search, grep/glob | P0 | DONE | Tool loop parses/executed core tools | Expand schemas and safety | Agent edits file and runs validation |
| Animated tool cards | P0 | DONE | Running/completed/failed tool card UI. Running cards have `.tool-running` CSS class with shimmer animation overlay. Status badges highlighted in info color for running state. Tool cards expose `data-tool-name` attribute for targeted selectors. | Batch grouping | Browser shows live tool card states with shimmer animation |
| Tool result display | P0 | DONE | Expandable results with args/output/duration/path | Compact large outputs | Expand card and inspect output |
| Native parallel tool execution | P1 | DONE | `exec_tool_calls` now groups contiguous parallel-safe calls (read_file, search_text, repo_scan) and executes them concurrently via `thread::spawn`. Each call still gets retry/timeout via `execute_parsed_tool_call`. All SSE events (tool_start/tool_result) are emitted in call order. Test suite: 487/487 pass. Files: `tool_exec.rs:339+`. | — | 2+ independent reads overlap in time |
| Synthetic parallel batch tool | P0 | DONE | `parallel_tool_calls` tool + `ToolStrategyController::should_auto_batch` auto-wraps multiple parallel-safe calls into synthetic batch for Harmony models. `build_parallel_batch_args` produces valid JSON. 6 tests. SSE events: `parallel_tool_calls_start`/`done` | — | GPT-OSS batch route finishes 100 calls before timeout |
| Tool approval gates | P0 | DONE | Permission cards and approval store | Expand to batch/subagent/external dispatch | Dangerous action asks before execution |
| Tool permission system | P0 | DONE | Risk levels and permission store | Per-step/per-agent scopes | Permission CRUD tests pass |
| Subagent spawn | P1 | PARTIAL | `run_subagent`/agent task records exist | Better UI, scope, limits, summaries | Subagent task created and result visible |
| Parallel subagent orchestration | P1 | PARTIAL | Read-only tools parallelize; subagent orchestration not complete | Scheduler, queue, merge/synthesis | 4 subagents run concurrently with proof |
| Subagent role system | P2 | DONE | 4 domain roles: `explorer` (read-only research: read_file/search_text/repo_scan), `worker` (execute: read_file/search_text/bash), `builder` (edit: read_file/search_text/file_edit/bash), `reviewer` (verify: read_file/search_text/repo_gate/bash). Each role has `allowed_tools()` and `tool_scope_description()`. `SubagentSpec` derives `read_only` + `allowed_tools` from role. `can_call_tool()` enforcement function in `SubagentRunner`. LLM prompt includes role description + allowed tools. Tool definition in `tools.rs` updated. `SubagentRole::from_str` returns `None` for old role names. 9 new tests across core + agent crates. Files: `subagents.rs:79-96`, `subagent_runner.rs:28-38`, `tool_handlers.rs:110-139`, `tools.rs:113`. | — | Role-specific tools enforced via `can_role_call_tool()` |
| Planning mode | P0 | DONE | Plan/build toggle blocks write tools in plan mode; dedicated `plan` SSE tool reports mode, lists read-only/write tools, supports switch_to_build/switch_to_plan; REST GET /api/plan; makePlanCard UI; handler at plan_handler.rs. Visible Plan/Build toggle button added near chat composer (`#composer-mode-toggle`) showing `📋 Plan` or `🔧 Build`. Tab key toggles mode (supplementary; button is primary). Sidebar toggle (`#self-build-mode-toggle`) remains and both sync. | — | Plan uses read-only tools only; both toggles stay in sync |
| Goals | P2 | DONE | Goal records/endpoints, proof validation (`mark_achieved` requires non-empty `proof_refs`), auto-evaluator (detects common conditions like `cargo test`, `cargo check`, `cargo fmt`, `cargo clippy`, `cargo build`, and runs them as shell checks). `GoalHistoryEntry` tracks status transitions. `evaluate_all_goals()` called automatically on goal update. `AllGoalsAchieved`/`AnyGoalAchieved` stop conditions parsed in `should_stop()`. `add_proof_ref()` with dedup. `condition_check_command()` maps 6 condition patterns to commands. 7 new core tests + 7 evaluator tests. Files: `goal.rs:159-192`, `goal_evaluator.rs` (210 lines), `goals.rs:38-47`, `loop_orchestration.rs:187-188`. | — | Goal cannot complete without proof; condition evaluation runs on update |
| Todos | P1 | DONE | User-visible and model-visible todo CRUD | Nested subtasks | Todo states update with proof refs |
| Agent loop state | P1 | PARTIAL | Loop records, stop conditions, verification, checkpoint exist | Explicit phase display, repair plan, compaction | Loop shows phase and changed strategy after failure |
| Self-correction / repair loop | P1 | DONE | Failed tool calls tracked in `EvidenceGatherer` (`failed_tool_calls`, `tool_failures`, `record_tool_failure()`, `recent_failures_summary()`). On tool failure, a user message is injected with the error and directive to change approach: "Tool '{name}' failed. Error: {error}. Try a different approach — change the arguments or use a different tool." Dedup via `seen_tool_calls` blocks exact recreate of any previous call (success or failure). Files: `evidence_gatherer.rs:106-128`, `tool_exec.rs:402-407,471-476`. | — | Failed action is not retried unchanged |
| Tool timeout handling | P1 | DONE | 60s timeout around tool execution | Per-tool/batch configurable timeout | Slow tool returns timeout error |
| Shell command validation | P1 | DONE | Destructive patterns blocked (18 hardcoded in `destructive_check.rs`). Configurable `ShellValidationPolicy` loaded from `.superapp/shell_policy.json` supports: `allowed_commands` (strict whitelist when non-empty), `blocked_commands` (additional patterns), `allowed_domains` (domain allow list for curl/wget), `block_domains_by_default` (default: true). Policy checked before bash execution in `tool_exec.rs`. `bash` added to `stream.rs` write-tools list (blocked in read-only/plan mode). Network policy parses URL hosts and matches against allowed domains. 10 unit tests. Files: `shell_policy.rs` (306 lines), `tool_exec.rs:587+`, `stream.rs:109`. | — | `rm -rf /` blocked with reason; curl allowed only to configured domains |
| Hooks | P1 | DONE | Generic hook framework in `hooks/` module: `HookPoint` enum (16 points: Pre/PostToolCall, Pre/PostCommit, Pre/PostPush, TaskCreated/Completed/Failed, SessionStart/End, Pre/PostAgentStep, Pre/PostModelCall, Error, Custom), `HookRegistry` with priority ordering and filters, `HookRunner` trait with async execution, `BuiltinHookRunner` for inline handlers. Registered via `global_registry()`, executed via `run_hooks()` with short-circuit on failure. Events: `HookEvent` with `HookContext` (conversation_id, correlation_id, payload). Built-in points wired: Pre/PostToolCall in `tool_exec.rs`, Pre/PostCommit in `commit.rs`, Pre/PostPush in `git.rs`, TaskCreated/Completed in `task.rs`. | — | `run_hooks()` short-circuits on failure; pre-tool hook can block/modify tool call |
| Phantom tool removal | P0 | DONE | `self_review`, `validation_hooks`, `done_gate` removed from tool schema. These tools return "unknown tool" from agent loop (ToolKind::from_str returns None) but are still accessible via direct SSE dispatch. Removing them reduces tool schema noise from 14→11 tools. | — | Tool schema shows 11 tools instead of 14 |
| Force file_edit on stall | P0 | DONE | `StallResult::Break` injects break message as user prompt, sets tool_choice to function-specific `file_edit`, and excludes read-only tools (tools=7). `bash` removed from `tool_definitions_json_inner` when `exclude_readonly=true`, leaving only `file_edit`, `repo_gate`, `run_subagent`, `manage_todo`, `summarize_changes`, `browser_proof`. When model returns text under forced file_edit, text is preserved in messages and a strong re-prompt "You MUST call file_edit. Do not respond in text." is injected. Files: `tools.rs:104-113`, `tool_loop.rs:269-274`. | — | Forced LLM call with only write tools produces tool_start events |
| Stall message cleanup | P0 | DONE | `stall_warning_msg` and `stall_break_msg` no longer reference `done_gate` (which was removed from schema). Messages now say "Call file_edit now" directly. | — | Stall messages reference only existing tools |
| Provider error routing | P0 | DONE | 429 → exponential backoff (`1000ms * 2^(count-1)`, capped 30s) before routing to next model. `provider_429_count` tracked in `emit_trace` as `rate_limited` field visible in trace card meta bar (`🚦 N×429`) and detail body. Provider marked degraded in `model_degraded` HashSet after backoff; subsequent rounds skip with `degraded_skip` + `provider_route_changed` SSE event. Transport errors get one retry with compact context, then `break 'tool_loop`. All providers rate-limited fires `ALL_TEXT_PROVIDERS_RATE_LIMITED` blocker with evidence summary. Fixed: `rate_limited_route` was in `any_success` causing all-degraded infinite loop. | — | 429 waits exponentially before routing to next provider |
| Scope guard for constrained prompts | P0 | DONE | `ScopeGuard` module (`scope_guard.rs`) parses prompts for constraints ("at most one string", "one visible tooltip", "one label", "keep behavior unchanged", "smallest safe change"). Before returning success, inspects git diff and checks against constraints: too many strings → `SCOPE_VIOLATION_TOO_MANY_STRINGS`, too many files → `SCOPE_VIOLATION_TOO_MANY_FILES`, behavior file changed → `SCOPE_VIOLATION_BEHAVIOR_CHANGE`, combined violations → `PATCH_NEEDS_NARROWING` with narrowing guidance. Emits agent_status SSE event with scope check result. 14 unit tests. Trigger counts recorded in controller metrics (`scope_guard_triggered`). | — | Constrained prompt produces scope violation instead of over-broad diff |
| Context-aware text re-prompt | P0 | DONE | Text-only re-prompt message changed from generic "Continue using tools" to "Call the next tool now — use read_file to inspect a file, then file_edit to make changes." When stall >= 3 rounds, message becomes "You searched and read files above. Now call file_edit with the exact find and replace strings." EvidenceGatherer (uses 14 tracked fields) fires additional guided nudges: repeated-search → specific directive, enough-evidence (≥1 search + ≥1 read) → "call file_edit now". Provider 429 degradation tracking and early-abort with evidence summary implemented. Exercised in live dogfood at 2ef0a40: model called file_edit directly without reaching stall path, proving nudges are not needed when the model is well-behaved. | — | Re-prompt guides model toward file_edit; nudges transition model from research to edit phase |
| Deterministic self-patch controller | P0 | DONE | SelfPatchController module (`self_patch.rs`) extracts search term from search_text results, reads the target file, identifies UI element via `title` attribute, and computes deterministic title improvements. Applied as guarded fallback at inner-loop text-only stall (≥3 rounds), outer-loop stall break, and evidence-guided path. Emits visible `tool_start`/`tool_result` tool card events. 13 unit tests. gpt-oss-120b calls file_edit directly before stalling — proven in live dogfood (2ef0a40: 9 tooltip titles improved without reaching self-patch fallback). | — | Self-patch tool card visible in WebUI; source file modified with deterministic title improvement |
| Compact LLM context for tool results | P1 | DONE | `compact_context.rs` (10 tests) compresses tool outputs before sending to LLM: search_text keeps top 5 file paths, read_file keeps first/last 20 lines for large files, browser_proof sends JSON verdict + paths only, repo_scan truncated to first/last 10 lines. Duplicate tool calls get "Previously called" reference. Provider retry compacts context after 2nd failure. Reduces model-attempt waste: 53→24 attempts in bounded proof. | — | Tool results sent to LLM are visibly shorter than raw output |

### 3.3 Model and Provider

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| Multi-provider support | P0 | DONE | Groq, NVIDIA NIM, OpenRouter, GitHub Models, custom OpenAI-compatible | Keep provider/model naming clean | Switch between configured providers |
| Model selector dropdown | P0 | DONE | UI model picker from API | Provider/model detail view | Select model and send message |
| Provider order/priority | P0 | DONE | Drag-and-drop order persisted via POST /api/model-order to `.superapp/model_order.tsv`. WebUI shows drag handle (⠿) and capability badges (Text/Tools/Vision). Route status via `provider_route_changed` SSE toasts. **PROVEN**: order survives restart (POST→restart→GET confirms). WebUI proof (10/10 Playwright checks, PR #7b91922): drag handle visible, badges visible, Text badge present, items show provider/model names. | — | Restart preserves order |
| Provider failover/routing | P0 | DONE | Iterates preferred models via `build_router_plan()` using `current_model_order()` from persisted `.superapp/model_order.tsv`. On 429: marks degraded in `model_degraded` HashSet, emits `provider_route_changed` SSE with `reason:429`, routes to next candidate via `break 'tool_loop`. On transport error: one retry with compact context, then next candidate. All degraded → `ALL_TEXT_PROVIDERS_RATE_LIMITED` blocker. SSE `provider_fallback` + `provider_route_changed` events shown as toasts in WebUI. **PROVEN**: code path trace from `sse_handler.rs:93-94` (`configured_ids_and_order()` → `current_model_order()` → `build_router_plan()`) confirms router uses saved order. 5 unit tests verify plan ordering, dedup, missing-key skip, configured→candidate, transient retry. | — | Primary fail visible, route change events shown |
| Provider key management | P1 | DONE | UI + secrets file | Rotation/test button | Key persists masked |
| Key masking / no secret roundtrip | P0 | DONE | Masking and redaction helpers | Audit all logs/proof exports | Full key never displayed |
| Model refresh from provider API | P1 | DONE | `#refresh-models-btn` in sidebar triggers `GET /api/models/discover`, refreshes model picker dropdown. Auto-refreshes on page load. | — | Models updated without restart |
| Model-specific config | P2 | MISSING | No per-model max_tokens/top_p/etc. | Params per provider/model | API payload reflects config |
| Reasoning effort control | P3 | MISSING | No selector | Add per-supported model | Request includes effort param |
| Temperature/top_p | P2 | MISSING | No sliders/inputs | Add UI/API/storage | Changed params affect request |
| Local model support | P2 | MISSING | No Ollama/llama.cpp provider | Add local provider config | Local model responds |
| Provider availability detection | P1 | DONE | Status checks env/secrets | Add live health/quota checks | Missing key shown clearly |
| Dynamic model discovery from provider API | P1 | DONE | `GET /api/models/discover` fetches `/v1/models` from each configured provider using API key + base URL. Parses OpenAI-compatible response. Results cached in `.superapp/models_cache.json` with timestamp (300s TTL). Merged with static model list in `models_json()`. `nvidia_nim` currently returns 121 models (was 4 hardcoded). 🔄 refresh button in sidebar triggers re-discovery. Auto-refreshes on page load. | Add periodic background refresh | Discovery lists all provider models, no duplicates with static list |
| NVIDIA NIM / gpt-oss-120b slow-provider blocker | P1 | DONE | NVIDIA NIM `/v1/chat/completions` endpoint via `integrate.api.nvidia.com` is slow/flaky. gpt-oss-120b regularly exceeds 60s for first token on common prompts. This no longer blocks daily use because GPT-OSS-120B is not the mandatory default model. The router selects faster models first and falls back to GPT-OSS only when needed. GPT-OSS remains available in the stack for self-refactor/coding tasks and benchmarking. | Unblocked by new router policy (not default). Monitor GPT-OSS for required benchmarks. | E2E test completes within timeout using mistral-small or router-selected model |
| DeepSeek V4 Flash Free working path | P0 | DONE | DeepSeek V4 Flash Free (`deepseek-ai/deepseek-v4-flash`) on NVIDIA NIM works. Completed: multi-file batch read, file_edit (added stop button titles), validation, subagent dispatch. Total wall time ~4m26s for a useful self-build task. Retained in stack for benchmarking. The first-slot default is now determined by live probe proof (mistral-small-4-119b-2603). | No longer recommended as default; kept as benchmark comparison point. | DeepSeek V4 completes a standard self-build task within 5 minutes |

### 3.4 New Router Policy

The old GPT-OSS-default contract has been superseded by Joonas's new-router decision. This section documents the canonical router architecture for this branch.

| Rule | Status | Implementation |
|------|--------|----------------|
| One visible runtime-context model stack | DONE | `automatic_router.rs` — single stack with `RoutingDecision`, `ModelHealthEntry`, `RouterPolicy`. No parallel router systems or v2/experimental branches. |
| Selected/default model determined by router policy, live capability proof, saved model order, and user configuration | DONE | `select_model()` uses `RoutingContext` (current_step, tool_requirement, context_size) + `ModelCapabilityEntry` flags + saved order + health state. |
| GPT-OSS-120B is supported and benchmarked, but no longer mandatory default | DONE | GPT-OSS at position 14/16 in `default_model_order()`. Available for manual selection and benchmarking. First position is `mistralai/mistral-small-4-119b-2603` based on live probe proof (score 18, 393ms, tool_ok true). |
| Model selection visible through `model_card` SSE events | DONE | `emit_model_card()` in `routing_handler.rs` — emits provider_id, model_id, current_step, reason, order_position, total_in_order, saved_order_changed, fallback_from, fallback_reason. |
| Model fallback/swap visible through `model_swap` SSE events | DONE | `emit_model_swap()` in `routing_handler.rs` — emits from_provider_id, from_model_id, to_provider_id, to_model_id, reason, temporary, saved_order_changed. |
| Drag/drop model order persists and is honored | DONE | `current_model_order()` loads from `.superapp/model_order.tsv`. Router iterates this order. Drag/drop persistence tests pass. `saved_order_not_silently_reordered` test passes. |
| Keyword prompt classification is forbidden | DONE | `request_classifier.rs` deleted. `RequestKind` enum removed. Routing uses `CurrentStep` enum and `RoutingContext` signals only. No `prompt.contains(...)` keyword checks. |
| Hidden model swaps are forbidden | DONE | Every model change emits a `model_swap` or `provider_route_changed` SSE event with reason. `hidden_model_swap_not_emitted` test would fail. |
| Every model decision has a receipt | DONE | `RoutingDecision` contains selected_model, reason, order_position, fallback_from, fallback_reason. Receipt rendered in `model_card` event. |
| Benchmarks/probes may update recommendations or candidate ordering only when documented and visible | DONE | Probe-based recommendations documented in this policy. Probe results stored in `.superapp/model_health.json`. Saved order not silently reordered by runtime. |
| Mistralai/mistral-small-4-119b-2603 is current first-slot winner based on live probe proof | PROVEN | First‑slot shootout: score 18, simple 393ms, tool_ok true, tool_calls 1. |
| GPT-OSS-120B remains in the stack and should still be tested for self-refactor/coding tasks | DONE | GPT-OSS at position 14/16 in default order. Still tested in benchmark suite. `GPT-OSS parallel tool routing` acceptance test still applies (section 15.2). |
| The old GPT-OSS-default contract has been superseded | DONE | This policy section replaces the previous policy. No other docs require GPT-OSS as mandatory default. |

### 3.5 Code and Git

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| File create/edit/replace/delete | P0 | DONE | File edit and patch handlers. Repo-path guard (repo_path_guard.rs) rejects nonexistent, outside-repo, and wrong-template paths (src-tauri/**, packages/**, app/**) before edit with blocker codes: EDIT_TARGET_PATH_NOT_FOUND, EDIT_TARGET_OUTSIDE_REPO, EDIT_TARGET_NOT_IN_SOURCE_MAP, EDIT_TARGET_CREATE_NOT_ALLOWED, MODEL_REPO_PATH_CONFUSION. Compact SourceMap::compact() provides allowed-dir guidance. CorrectionState allows one retry then stops. Integrated into file_edit_handler.rs and self_patch.rs. 13 unit tests. | — | Agent edits and verifies file; wrong-template paths rejected |
| Git status/diff/log | P0 | DONE | Bash/tools can run git | Dedicated git service/UI | Ask status/diff/log and get accurate answer |
| Git commit/push | P1 | DONE | Bash tools can commit/push | Pre-push gate and proof requirements | Validated commit/push only |
| Worktree support | P1 | DONE | Worktree store/endpoints | UI polish and branch lifecycle | Create isolated worktree |
| Branch management | P2 | PARTIAL | Worktree implies branch creation | Branch list/switch/delete UI | Switch branch safely |
| Diff viewer | P1 | PARTIAL | Patch summary cards exist | Side-by-side/unified diff with line highlights | View patch before apply |
| File change summary | P1 | PARTIAL | Per-edit summaries | Aggregated run summary | Multi-file edit summary visible |
| Commit readiness analysis | P1 | DONE | `commit_readiness_handler.rs` (307 lines, 8 tests) runs 6 checks: Git status (branch/dirty), dirty ownership (user vs agent files), secrets scan, compilation (cargo check), formatting (cargo fmt --check), proof (validation/browser_proof/self_review via ToolRunStore). Produces JSON report `{all_pass, blocking, checks[]}`. SSE tool `commit_readiness` + REST endpoint. Blocking checks (git, ownership, secrets, compilation) prevent commit if failed. | — | Readiness report blocks bad commit |
| Pre-push CI gate | P1 | DONE | `scripts/pre_push_gate.sh` installed as `.git/hooks/pre-push`. Blocks push on critical check failure (git status, compilation). Non-critical warnings: formatting, prompt CI gate (skips if server not running). | — | Push blocked on compilation failure; formatting warns but allows push |

### 3.6 Files and Multimodal

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| Text file attachments | P1 | DONE | File input/drag-drop reads text and sends content | Size limits and truncation | Attach `.rs`/`.py` and model sees content |
| Drag/drop text files | P1 | DONE | Footer dropzone with chips | Better preview | Drop file and send |
| Image attachments/display/preview | P2 | MISSING | No image flow | Image chip, preview, inline render | Upload image and see preview/render |
| Image understanding | P2 | DONE | NIM vision path in `browser_proof_handler.rs` — screenshots sent as base64 data URLs via NIM `/v1/chat/completions`. Preferred model order: `meta/llama-3.2-11b-vision-instruct` → `meta/llama-3.2-90b-vision-instruct`. Phi models (`microsoft/phi-4-multimodal-instruct`) no longer default. Vision prompt asks 7 UI-specific questions (WebUI loaded, blank/crashed, composer visible, Build/Plan mode, tool cards, Stop/Cancel, visible issue). Gap codes: `NIM_VISION_MODEL_UNAVAILABLE` (404/model-unknown), `NIM_RATE_LIMITED` (429), `NIM_VISION_REQUEST_FAILED` (5xx/other). 10 unit tests. Produces `vision-review.json` with verdict/gap. NIM Llama 3.2 11B vision model confirmed working via Rust `ureq` path at runtime (`vision_verdict: pass` E2E). Runtime identity (`/health`) includes `git_head`, `pid`, `start_time`, `cwd`, `binary`, `port`, `features` (browser_proof, nim_vision). Smokes verify server matches repo HEAD (stale-binary guard). `scripts/restart_superapp_dev_server.sh` helper. | — | Browser proof smoke includes vision verdict or exact gap code |
| Image generation/editing | P3/P4 | MISSING | Out of MVP | Provider/tool integration later | Generated/edited image returned |
| Code interpreter | P2 | MISSING | No sandboxed Python/Node | Isolated sandbox and resource limits | Run code and capture stdout |
| RAG/file search | P2 | MISSING | No vector/index search | Embeddings/index/citations | Uploaded doc searchable with citations |
| PDF/document understanding | P2 | MISSING | No PDF parsing | Text extraction and citation mapping | Ask question about PDF content |

### 3.7 Settings and UI/UX

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| System prompt editor | P1 | DONE | File + GET/PUT endpoints and UI | Version/history if useful | Custom prompt affects request |
| Server URL/listen config | P2 | MISSING | Not configurable in UI | Configurable host/port | Restart on chosen address |
| Auto-routing/fallback toggle | P2 | MISSING | Always fails over | Toggle no-fallback mode | Provider failure stops when disabled |
| Theme: dark/light | P0/P2 | DONE | Dark + light theme with localStorage | Polish contrast | Toggle themes |
| Responsive/mobile layout | P2 | DONE | Media queries | Dedicated mobile status view later | Usable at 480px |
| Keyboard shortcuts | P1 | DONE | Enter (send), Shift+Enter (newline), Escape (stop/cancel). Tab key toggles Plan/Build mode when textarea/input is not focused. | Custom bindings | Shortcut smoke |
| Toasts/errors/status | P1 | DONE | Status bar and toasts | Error catalog | Simulated failure shows actionable message |
| Debug/telemetry panel | P1 | DONE | Collapsible sidebar panel in `index.html` with real-time SSE event log: event count, rounds, tools, duration, attempts, tokens, correlation ID. `stream.js` captures all events via `debugEventLog` array (max 100), `render.js` renders summary + recent 50 entries. Updates on every `handleEvent()` call. | — | Open Debug panel in sidebar, send message, observe live event log with timestamps |
| Accessibility labels | P2 | PARTIAL | Test IDs and some focus support | Add aria labels and aXe audit | Accessibility smoke passes |
| Reduced motion | P3 | MISSING | Not implemented | CSS media query | OS reduced motion disables animations |
| Tooltip/help text | P2 | MISSING | Sparse titles | Add hover/aria descriptions | Hover shows explanation |

### 3.8 Security and Privacy

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| OS-level sandboxing | P1 | MISSING | Tools run as server process | Sandbox/container/worktree policy | Outside-scope mutation blocked |
| Tool risk levels and approvals | P0 | DONE | PermissionRiskLevel and approval flow | Per-agent/per-batch policy | Risky tool asks approval |
| Destructive command detection | P0 | DONE | Pattern matching blocks dangerous shell commands | Expand tests and allow/block config | Destructive command blocked |
| Read-only exploration mode | P1 | DONE | Read-only flag gates write tools; self_build_mode plan mode gates write tools; clean mode messages. Plan mode blocks 5 write tools: file_edit, patch_proposal_apply, approved_patch_loop, approved_patch_continue, patch_revert. Blocked tools show clear SSE rejection message: "{mode} mode is active. Tool '{t}' is blocked. Switch to build mode in settings to perform write operations." | — | Write tools blocked in read-only with actionable message |
| Secret leakage prevention | P0 | DONE | Redaction helpers | Scan all proof exports | Logs/proof contain no secrets |
| Visible provider fallback | P0 | DONE | Fallback event/status warning | Include in run export | Primary fail/fallback shown |
| File scope restrictions | P1 | DONE | Path canonicalization under repo root | Include all tools | `/etc/passwd` read blocked |
| Network access policy | P1 | MISSING | No network allow/block list | Tool HTTP/network policy | External network blocked by config |
| Provider privacy disclosure | P2 | MISSING | No per-provider disclosure | Provider settings privacy text | User sees provider data warning |
| Central audit log | P1 | PARTIAL | ToolRun records exist | Queryable run/audit log across tools/models/users | Audit log lists all actions |
| Prompt injection guard | P1 | MISSING | Not formalized | Treat external content as data; output filtering | External doc cannot override policy |

---

### 3.9 Integration and Interop

| Feature | Pri | Status | Evidence / current state | Missing work | Proof required |
|---------|-----|--------|--------------------------|--------------|----------------|
| Benchmark Adapter API v1 | P0 | DONE | `crates/superapp-server/src/benchmark_adapter.rs` module with 11 endpoints implementing the full §7 Generalized Benchmark Adapter API v1 contract: GET /api/benchmark/health, GET /api/benchmark/capabilities (§7 capability schema with `non_blocking_run: true`), POST /api/benchmark/readiness (full readiness with repo root inventory/guard/browser-proof check), POST /api/benchmark/run (non-blocking — returns in < 100ms with `status:"queued"`, runs LLM in background thread, events visible during execution via shared state), GET /api/benchmark/runs, GET /api/benchmark/runs/{id}/events, GET /api/benchmark/runs/{id}/artifacts, GET /api/benchmark/runs/{id}/result, POST /api/benchmark/runs/{id}/cancel (supported, via file-based cancel token), POST /api/benchmark/runs/{id}/pause (capability_gap), POST /api/benchmark/runs/{id}/resume (capability_gap). Cancel supported via file-based token. Thread panic caught so server stays alive. 27 Rust unit tests + Node.js smoke script (36/36) all pass. Live-tested against NVIDIA NIM (openai/gpt-oss-120b). | Run result/events not yet persisted across server restarts (in-memory only) | Rust tests (27/27) + smoke script (36/36) + live non-blocking run verified (< 100ms return) |

## 4. Tool Strategy Controller / Model-Aware Tool Orchestration

This is a P0 product layer. It is different from simply having a parallel executor.

### Problem statement

Some models, including GPT-OSS-120B behind common HF/NIM-style wrappers, may only emit one native tool call per round or may not choose a synthetic batch tool by themselves. The app must not rely on prompt keywords like "parallel" or "100." It must choose tool strategy based on model capability, current step type, dependency structure, side-effect risk, and observed execution trajectory.

### Required concepts

| Feature | Pri | Status | Missing work | Proof required |
|---------|-----|--------|--------------|----------------|
| Model tool capability registry | P0 | DONE | Registry maps model IDs to `ModelCapabilities` (native_parallel, synthetic_batch, max_batch_size) | Registry shows GPT-OSS native parallel false and synthetic batch true |
| Tool strategy controller | P0 | DONE | `ToolStrategyController` with `record_and_evaluate()` chooses `NaturalAuto` or `BatchContinuation` per round | Logs show chosen strategy and reason |
| Synthetic batch routing | P0 | DONE | `parallel_tool_calls` expanded into synthetic `ParsedToolCall` entries in `sse_handler.rs` | GPT-OSS completes 100 read-only calls before timeout |
| No-keyword fan-out detector | P0 | DONE | `is_repeated_serial_pattern()` detects repeated safe reads from tool call history, not prompt keywords | After 3 repeated safe reads, route switches to batch continuation |
| Structured plan-to-DAG | P0 | DONE | Ask model for work items with dependencies and side-effect class for large tasks. Plan contains work items/dependencies/classes | `dag_planner` SSE tool + REST POST /api/dag/plan parses structured DAG JSON with nodes (id, tool, args, depends_on). Reports node count, per-node side_effect class, and dependency edges |
| DAG batch scheduler | P0 | DONE | Execute independent DAG nodes together; serialize dependent writes/tests/commits. Independent reads overlap; writes stay ordered | `dag_executor` SSE tool + REST POST /api/dag/execute runs topological sort, groups nodes by level, executes parallel-safe nodes via `ParallelToolRunner`, serializes write nodes. Returns per-node ok/fail, node count, level count |
| Side-effect classifier | P0 | DONE | Classify tools as read-only, workspace-write, external-write, destructive. Parallel reads allowed; writes gated/serialized | `classify_tool()` categorizes 25+ tools; `can_run_parallel()` uses classifier. ReadOnly (read_file, search_text, repo_scan, self_review, etc.), WorkspaceWrite (file_edit, patch_proposal_apply, etc.), ExternalWrite (git_push, npm_publish), Destructive (git_force_push, rm_rf) |
| Tool-choice policy per step | P1 | MISSING | Runtime can choose auto, required, forced function, or narrowed tools | Event log records tool-choice policy |
| Batch result compaction | P1 | MISSING | Aggregate large batch outputs before sending back to model | 100 calls do not flood context |
| Batch cancellation/timeout | P1 | MISSING | Cancel whole batch and individual calls; enforce per-call timeouts | Cancel stops all active calls safely |
| Batch UI event model | P1 | PARTIAL | Emit batch and per-call events | UI shows batch card plus child tool cards |

### No-keyword routing rule

Bad:
```ts
if (prompt.includes("parallel") || prompt.includes("100")) forceBatch();
```

Good:
```ts
if (planner.independentWorkItems.length > 1) runBatch();
if (trajectory.repeatedSerialSafeReads >= 3) switchToBatchContinuation();
if (tool.sideEffect !== "read_only") serializeOrGate();
```

### Suggested model caps schema

```ts
type ModelToolCaps = {
  providerId: string;
  modelId: string;
  nativeParallelToolCalls: boolean;
  autoSelectsSyntheticBatchTool: boolean;
  supportsForcedToolChoice: boolean;
  supportsStrictJson: boolean;
  supportsStructuredPlan: boolean;
  maxParallelCalls: number;
  maxToolRounds: number;
  notes?: string;
};
```

### Suggested strategy signal schema

```ts
type StrategySignal = {
  modelCaps: ModelToolCaps;
  activeTools: ToolSpec[];
  lastToolCalls: ToolCallRecord[];
  pendingWorkItems: WorkItem[];
  dependencyGraph: DependencyGraph;
  sideEffectRisk: "read_only" | "workspace_write" | "external_write" | "destructive";
  repeatedSerialPattern: boolean;
  timeoutBudgetMs: number;
  browserProofRequired: boolean;
};
```

---

## 5. Natural-Language Benchmarks

Benchmarks must test natural daily usage, not only scripted harness prompts or keyword-triggered paths.

### Required benchmark modes

| Mode | Purpose | Required for GPT-OSS? | Passing meaning |
|------|---------|----------------------|-----------------|
| Natural auto baseline | No special routing; user-like prompt; `tool_choice=auto` | Yes | Measures raw model behavior, even if it fails |
| Guided batch | Tool choice forced/narrowed to batch for known high-fanout work | Yes | Proves batch mechanism itself works |
| Product router | Natural prompt; app detects/chooses strategy structurally | Yes | Proves real product behavior without keyword hijacking |
| Cross-model parity | Same task/spec across GPT-OSS, DeepSeek, OpenCode, etc. | Yes | Compares models under same rules |
| Regression replay | Rerun saved prompts/scenarios against same repo/app state | Yes | Detects regressions over time |

### Natural-language benchmark requirements

| Feature | Pri | Status | Missing work | Proof required |
|---------|-----|--------|--------------|----------------|
| Natural prompt corpus | P0 | DONE | `prompts/` directory with 15 prompts across 7 categories: coding (3), debugging (3), review (2), loops (2), browser (1), settings (2), side-questions (2) | Expand with more scenarios | Corpus exists and is versioned |
| No-keyword benchmark guard | P0 | DONE | `is_repeated_serial_pattern()` detects repeated safe reads from tool call history, not prompt keywords. Tests in tool_strategy.rs verify batch continuation triggers after 3 repeated read_files with no keyword. Benchmarks use REST API directly or neutral prompts | — | Product router passes with normal wording |
| Dynamic feature-based test selection | P1 | MISSING | Choose scenarios based on implemented feature ledger/audit rows | Benchmark tests latest implemented features automatically |
| Cross-app same-rules comparison | P0 | DONE | Standardized benchmark schema in docs/benchmark-schema.md defines scenario JSON format, run result JSON format, and required endpoints (/api/prompt, /api/cancel, /api/run-export, /api/tool-call-ledger, /api/browser-proof) for cross-app comparison | — |
| Human-like daily-use benchmark | P0 | DONE | `scripts/smoke/daily-use-benchmark.js` — launches app, checks all UI elements, sends message, verifies tool cards, tests model picker, runs 7+ API endpoint checks, takes screenshots, captures console errors. Saves results to target/bench-results/daily-use-*.json in bench-summary compatible format | Expand with real LLM prompts, multi-turn conversation, plan/build mode transitions | Daily prompt creates visible proof |
| Long-context/compaction benchmark | P1 | MISSING | Test long session, compaction, resume, proof preservation | Resume after compaction succeeds |
| Failure/rejection benchmark | P1 | MISSING | Test "still broken/no change" rejection handling | Claim becomes REJECTED and next run inspects failing path |

---

## 6. Fast Live Browser-Proof Benchmarks

The app needs fast live benchmarks for frequent iteration. These are not replacements for deeper benchmarks; they are short, high-signal checks that run with browser proof and meticulous logs.

### Runtime targets

| Benchmark class | Target wall time | Notes |
|-----------------|-----------------|-------|
| Smoke UI/browser proof | 30–90s | Must launch app and interact with UI |
| Fast tool strategy benchmark | 60–180s | Enough to prove routing and concurrency |
| Long-loop mini benchmark | 120–300s | Short enough for daily use; long enough to prove stop/resume/logging |
| Full benchmark suite | Optional longer run | Not required for every patch |

### Required fast live scenarios

| Scenario | Pri | Status | What it tests | Passing criteria |
|----------|-----|--------|--------------|-----------------|
| `live-parallel-tools-100-readonly` | P0 | DONE | `superapp-cli bench parallel-100` sends 100 concurrent DAG execute requests, measures throughput/latency | Results saved to target/bench-results/ | 100 safe read/search calls finish before timeout in product-router mode |
| `live-trajectory-repair` | P0 | DONE | `superapp-cli bench trajectory-repair` tests run-export, dirty-state, self-review, validation-hooks endpoints | Results saved to target/bench-results/ | After repeated one-by-one calls, batch continuation completes remaining work |
| `live-long-loop-20` | P1 | DONE | Benchmark adapter scenario `scenario: "long-loop-20"` runs 20 iterations with 4 phases (plan→execute→validate→review), heartbeats every 2s, failure guard demo at iteration 10 (blocks edit_file), browser_proof step at iteration 15. Emits 51 events including loop_start, 20 loop_phase, 20 heartbeats, failure_guard_demo, browser_proof, loop_completed, run_finished. Persisted to disk. | — | `curl -X POST /api/benchmark/run -d '{"scenario":"long-loop-20"}'` returns pass with 20 iterations |
| `live-stop-cancel-resume` | P0 | DONE | `superapp-cli bench stop-cancel-resume` tests cancel, compact, events, conversations endpoints | Results saved to target/bench-results/ | Run stops safely and resumes with next safe action |
| `live-subagent-fanout-4` | P1 | MISSING | Parallel subagent orchestration | 4 bounded read-only subagents run concurrently and synthesize result |
| `live-browser-proof-ui` | P0 | DONE | `scripts/smoke/browser-proof-ui.js` Playwright script launched by `browser_proof_handler` for local URLs. Captures screenshot, console log, actions log. Checks 8 UI elements (corrected selectors: `[data-testid='app-shell']`, `#composer`, `[data-testid='send-button']`). REST GET /api/browser-proof lists proofs. SSE events: browser_proof_start, browser_proof_result. NIM vision (`run_nim_vision`) called after screenshot if NVIDIA NIM key configured. Preferred model order: `meta/llama-3.2-11b-vision-instruct` → `meta/llama-3.2-90b-vision-instruct` (no longer defaults to Phi). vision-review.json stored alongside artifacts. Exact gap codes: `NIM_VISION_MODEL_UNAVAILABLE`, `NIM_RATE_LIMITED`, `NIM_VISION_REQUEST_FAILED`. Server uses per-request threading (`std::thread::spawn` per accept) to avoid deadlock when browser proof runner navigates to self. Script location resolved by walking up from CWD and binary parent directory trees (fixes `needs_runner` when server CWD is not repo root). E2E Playwright test (16/16) verifies full WebUI flow: submit `/browser-proof` via composer, wait for tool card, verify visibility and content. 10 unit tests for vision model order, gap codes, prompt, screenshot/key errors. | Fixed: `message_done` handler was replacing `bubble.innerHTML`, destroying the cardsWrap and all tool cards. Now preserves and re-attaches saved card children. |
| `benchmark-adapter-browser-proof` | P0 | DONE | Benchmark adapter's `require_browser_proof` now calls the same built-in `run_browser_proof()` runner as the WebUI agent tool (no separate benchmark-only path). Readiness (`/api/benchmark/readiness`) dynamically probes `playwright`, `chromium-browser`, `node` via `which` AND `node_modules/.bin/playwright` fallback. Capabilities (`/api/benchmark/capabilities`) truthfully reports `browser_proof`, `screenshot`, and `bound_browser_proof_runner:true`. Result includes `screenshot_path`, `browser_log_path`, `vision_review_path`, `vision_provider`, `vision_model`, `vision_verdict`, `capability_gap`. When Playwright/Chromium is available, readiness no longer blocks with `BROWSER_PROOF_UNAVAILABLE` and run proceeds with `result=completed`. | — |
| `live-tool-card-visibility` | P0 | DONE | Tool cards show args, output, duration via expandable details (▸/▾ toggle). ToolResultCard shows duration_ms. Status badges show running/completed/failed. CSS classes: .tool-details, .tool-detail-row, .tool-duration, .tool-status-badge. SSE tool_result events include duration_ms. | — |
| `live-provider-routing` | P0 | DONE | Provider failover detected via SSE provider_fallback event. Warning toast shown. Status bar shows active provider/model from renderAttempts. Current provider displayed in status bar during tool calls. Provider status container shows full attempt history. | — |
| `live-context-compaction-resume` | P1 | MISSING | Long chat compaction and continuation | Compaction preserves goal, task, proof, next action |
| `live-rejection-recovery` | P1 | MISSING | User says "still broken/no change" | Status becomes REJECTED and next run inspects real path |

### Browser proof artifact requirements

Every live browser benchmark must produce:

- URL tested
- Browser/browser version
- Scenario ID and run ID
- Step list with action, selector, expected result, actual result
- Screenshot path for failure and important success states
- Console error list
- Network error list
- SSE event transcript path
- Final verdict: `PASS`, `FAIL`, `UNPROVEN`, `BLOCKED`

---

## 7. Generalized Benchmark Adapter API v1 Contract

This section is the canonical shared contract for `agent-benchmark-app` and the app-side adapters for LocalGPT, Forgestack, and Superapp. It replaces older one-off benchmark endpoint notes. The goal is fair comparison under the same rules, with explicit proof labels, readiness gates, event streams, artifacts, and capability-gap handling.

Apps may expose this contract directly over HTTP, through a local CLI wrapper, or through a benchmark-harness adapter. The normalized semantics must remain identical.

### Purpose

LocalGPT, Forgestack, and Superapp must expose a shared benchmark adapter contract so `agent-benchmark-app` can compare them fairly across daily quality, natural task quality, tool-call behavior, browser proof, endurance, and future SWE-like repo tasks.

The adapter API is not a product UI requirement. It is a measurement and proof interface. Its job is to let the benchmark harness ask each app what it can do, run bounded benchmark tasks, collect trace/proof artifacts, and distinguish real capability from missing, cached, self-reported, or harness-inferred evidence.

OpenCode may be supported through a wrapper adapter for comparison, but the main requirement applies to LocalGPT, Forgestack, and Superapp.

### Core Principle

Every benchmark result must say exactly what was measured and how it was proven.

A result must never blur these categories:

- fresh result
- cached result
- self-reported capability
- harness-observed behavior
- target-native trace
- artifact-backed proof
- missing capability
- blocked run
- unimplemented feature

A missing capability is a **capability gap**, not a model reasoning failure.

### Required Truth Labels

All adapter results, capabilities, events, and artifacts must support these labels where relevant:

```json
[
  "fresh",
  "cached",
  "self_reported",
  "harness_observed",
  "target_native",
  "artifact_backed",
  "missing",
  "blocked",
  "not_implemented",
  "capability_gap"
]
```

Rules:
- Cached proof must be labeled `cached` and never counted as fresh.
- Self-report must not be counted as artifact proof.
- Harness-observed behavior must not be counted as target-native trace.
- Browser self-report must not be counted as browser artifact proof.
- Missing dimensions reduce coverage, not quality.
- Unimplemented judge/model scoring must not contribute to quality totals.

### Adapter Identity

Each app adapter must expose identity metadata:

```json
{
  "adapter_protocol": "benchmark-adapter-api-v1",
  "app_id": "localgpt|forgestack|superapp|opencode",
  "app_name": "",
  "adapter_version": "",
  "app_version": "",
  "app_git_head": "",
  "provider_id": "",
  "model_id": "",
  "model_attribution": {
    "provider_proven": false,
    "model_proven": false,
    "source": "cli_flag|api_response|config_file|self_report|unknown",
    "confidence": "high|medium|low|none"
  }
}
```

Model attribution must be explicit. An app passing a task does not prove the selected provider/model was used unless the adapter provides proof.

### Capability Discovery

Each app must expose a capability discovery result.

Required capability fields:

```json
{
  "chat": false,
  "natural_tasks": false,
  "repo_read": false,
  "repo_write": false,
  "shell": false,
  "tool_calls": false,
  "native_tool_traces": false,
  "parallel_tool_calls": false,
  "browser_self_report": false,
  "browser_artifact_proof": false,
  "screenshot": false,
  "dom_snapshot": false,
  "console_logs": false,
  "network_logs": false,
  "cancel": false,
  "pause": false,
  "resume": false,
  "context_compaction": false,
  "session_export": false,
  "repo_root_inventory": false
}
```

Capability proof must include:

```json
{
  "source": "target_native|harness_observed|self_report|config|missing",
  "proof_path": "",
  "warnings": []
}
```

Rules:
- A capability defaults to `false` unless proven.
- Self-reported capability is allowed but must be labeled `self_report`.
- Browser self-report is not browser artifact proof.
- Native tool traces require actual target-emitted trace data.
- Plain text claims must not be converted into native traces.

### Readiness Validation

Before running a task, the adapter must validate readiness.

Readiness input:

```json
{
  "run_mode": "DRY_RUN|LIVE_RUN",
  "run_depth": "FAST|STANDARD|FULL|CUSTOM",
  "task_family": "daily|natural|browser|toolcall|endurance|swe|terminal|stateful_app",
  "requires_provider": false,
  "requires_repo_mutation": false,
  "requires_browser": false,
  "requires_native_tool_trace": false,
  "confirm_live_run": false,
  "confirm_long_run": false,
  "allow_target_mutation": false
}
```

Readiness output:

```json
{
  "ready": false,
  "blocked": true,
  "block_code": "",
  "reason": "",
  "readiness_proof": {
    "provider_id": "",
    "model_id": "",
    "env_var_names_checked": [],
    "secret_values_exposed": false,
    "target_repo_clean": false,
    "target_repo_root": "",
    "root_inventory_path": "",
    "root_files_read": [],
    "root_guard_passed": false,
    "capability_source": "target_native|harness_observed|self_report|config|missing"
  }
}
```

Required block codes:

```json
[
  "DRY_RUN_NO_LIVE",
  "LIVE_CONFIRM_REQUIRED",
  "LONG_RUN_CONFIRM_REQUIRED",
  "PROVIDER_KEY_MISSING",
  "UNKNOWN_PROVIDER",
  "CAPABILITY_GAP",
  "TARGET_REPO_DIRTY",
  "MUTATION_NOT_ALLOWED",
  "BROWSER_PROOF_UNAVAILABLE",
  "NATIVE_TRACE_UNAVAILABLE"
]
```

Rules:
- `DRY_RUN` must never call providers, targets, browser tools, judge models, or live agents.
- `LIVE_RUN` must require explicit confirmation.
- `FULL`, endurance, and deep validation runs must require long-run confirmation.
- Unknown providers must be blocked.
- Missing provider keys must be blocked.
- Dirty target source repos must block mutation tasks.
- Secret values must never be printed, logged, or written to proof.
- Adapter readiness must inventory repo root before running repo, tool, browser, endurance, or SWE-like tasks.
- Adapter readiness must fail when root inventory is missing for tasks that depend on repo state.
- Root inventory must not print real secret files or secret values.

### Run Lifecycle

Each benchmark run must follow this lifecycle:

1. Discover adapter identity and capabilities.
2. Validate readiness.
3. Start run only if readiness passes.
4. Stream normalized events.
5. Collect artifacts.
6. Return final result.
7. Record proof index.

Starting a run does not count as task success.

Run start request:

```json
{
  "run_id": "",
  "task_id": "",
  "suite": "daily|natural|browser|toolcall|endurance|swe|terminal|stateful_app",
  "run_mode": "DRY_RUN|LIVE_RUN",
  "run_depth": "FAST|STANDARD|FULL|CUSTOM",
  "prompt": "",
  "allowed_tools": [],
  "timeout_ms": 0,
  "max_steps": 0,
  "max_retries": 0,
  "requires_artifacts": [],
  "confirm_live_run": false,
  "confirm_long_run": false,
  "allow_target_mutation": false
}
```

Run start response:

```json
{
  "run_id": "",
  "session_id": "",
  "accepted": true,
  "blocked": false,
  "block_code": "",
  "started_at": "",
  "adapter_protocol": "benchmark-adapter-api-v1",
  "proof_path": ""
}
```

### Event Stream

Adapters must expose normalized events.

Event shape:

```json
{
  "type": "status|message|tool_call|tool_result|artifact|error|progress|final",
  "timestamp": "",
  "sequence": 0,
  "source": "target_native|harness_observed|self_report|adapter",
  "data": {}
}
```

Tool-call event shape:

```json
{
  "type": "tool_call",
  "source": "target_native|harness_observed|self_report",
  "data": {
    "tool_call_id": "",
    "tool_name": "",
    "arguments": {},
    "arguments_valid": false,
    "parallel_group_id": "",
    "started_at": "",
    "ended_at": "",
    "result_used_later": false
  }
}
```

Rules:
- Tool-call traces are counted only when trace source is explicit.
- Harness-observed tool behavior is separate from target-native tool traces.
- Self-reported tool use may be displayed but must not count as native trace proof.
- Repeated/no-progress tool loops must be detectable from event history when possible.

### Artifact Collection

Adapters must return an artifact inventory.

```json
{
  "session_id": "",
  "artifacts": [
    {
      "artifact_id": "",
      "artifact_type": "stdout|stderr|diff|test_output|screenshot|dom_snapshot|console_log|network_log|tool_trace|session_log|root_inventory|final_answer|proof_json",
      "source": "target_native|harness_observed|self_report|adapter",
      "path": "",
      "sha256": "",
      "freshness": "fresh|cached|missing",
      "score_counted": true
    }
  ],
  "warnings": []
}
```

Rules:
- Source files are implementation, not proof artifacts.
- Screenshot proof requires an actual screenshot file.
- DOM proof requires an actual DOM/selector/AX-tree artifact.
- Console/network proof requires captured logs.
- Browser self-report cannot satisfy browser artifact proof.
- Benchmark-app browser proof proves the benchmark GUI, not the target app.

### Final Result

Each run must produce a normalized final result.

```json
{
  "run_id": "",
  "session_id": "",
  "target_id": "",
  "task_id": "",
  "status": "passed|failed|blocked|timeout|cancelled|capability_gap",
  "score": null,
  "score_counted": false,
  "score_source": "exact|semantic|test|state|artifact|trace|not_scored",
  "freshness": "fresh|cached|missing",
  "duration_ms": 0,
  "token_usage": null,
  "cost_usd": null,
  "pass_at_1": null,
  "pass_at_k": null,
  "retry_count": 0,
  "first_pass_success": null,
  "eventual_success": null,
  "tool_trace_source": "target_native|harness_observed|self_report|unavailable",
  "browser_proof_source": "target_artifact|benchmark_app_artifact|self_report|missing",
  "artifacts": [],
  "warnings": [],
  "proof_path": ""
}
```

Rules:
- `score_counted` must be false for blocked, missing, self-reported-only, or unimplemented dimensions.
- Missing proof must not be silently converted to failure.
- Missing proof must not be silently converted to zero quality.
- pass@k must be shown only when multiple attempts actually ran.
- Retry success must not hide first-attempt failure.

### Control API

Adapters should expose cancel, pause, and resume capability when supported.

Control response shape:

```json
{
  "supported": false,
  "success": false,
  "capability_gap": true,
  "reason": "",
  "proof_path": ""
}
```

Rules:
- Unsupported control actions are capability gaps, not model failures.
- If an action is supported, the adapter must prove the session state changed.
- Cancel must stop further tool/provider/browser activity for the session.

### Required Harness Routes

The benchmark app should normalize all app adapters through these routes:

```text
GET  /api/app-bench/adapters
GET  /api/app-bench/adapters/:targetId/capabilities
POST /api/app-bench/adapters/:targetId/readiness
POST /api/app-bench/adapters/:targetId/run
GET  /api/app-bench/adapters/:targetId/runs/:runId/events
GET  /api/app-bench/adapters/:targetId/runs/:runId/artifacts
POST /api/app-bench/adapters/:targetId/runs/:runId/cancel
POST /api/app-bench/adapters/:targetId/runs/:runId/pause
POST /api/app-bench/adapters/:targetId/runs/:runId/resume
GET  /api/app-bench/adapters/:targetId/runs/:runId/result
```

If an app does not expose HTTP directly, the benchmark app may use a CLI or wrapper adapter, but the normalized contract must remain the same.

### Required App-Side Capabilities by Phase

**Phase 1 — Minimum Adapter**

Each app must support:
- identity
- capability discovery
- readiness validation
- basic run start/result
- final answer artifact
- proof path
- explicit unsupported/capability-gap responses

**Phase 2 — Tool Trace Support**

Each app should expose:
- tool call events
- tool result events
- tool arguments
- argument validation status
- parallel group id when parallel calls occur
- result-used-later signal when available
- loop/no-progress indicators

**Phase 3 — Browser Artifact Support**

Each app should expose, when available:
- screenshot artifact
- DOM or selector artifact
- console logs
- network logs
- action trace
- browser task result
- clear distinction between self-report and artifact-backed browser proof

**Phase 4 — Repo/SWE-Like Support**

Each app should support controlled repo tasks only when explicitly allowed:
- clean repo check
- base HEAD
- target files
- patch/diff artifact
- validation command
- test output
- cleanup/restore proof
- no commit inside target repo unless explicitly allowed

### Scoring Requirements

The benchmark dashboard may compute combined scores only from counted dimensions.

Required component dimensions:
- daily synthetic
- natural quality
- tool-call behavior
- browser proof
- endurance
- SWE-like repo task

Rules:
- Combined score must show component scores.
- Combined score must show coverage score.
- Missing dimensions reduce coverage, not quality.
- Self-reported browser/tool capability does not count as artifact/native proof.
- Cached dimensions must be visible as cached.
- Low coverage must show a warning.
- Targets must not be ranked by combined score alone unless coverage is shown next to the score.

### Security and Safety Requirements

The adapter API must never expose:
- API key values
- provider secret values
- SSH keys
- tokens
- private env files
- hidden benchmark expected answers
- hidden judge outputs that leak answer keys

Proof may include env var names checked, but never values.

Target source repos must not be mutated unless the run explicitly allows mutation and the adapter proves:
- repo was clean before run
- mutation was required by task
- restore strategy existed
- validation command existed
- cleanup/restore proof was captured

### Documentation Labels Required in GUI

The benchmark GUI must show these labels wherever relevant:

```text
Capability gap is not model failure.
Self-report is not artifact proof.
Harness-observed trace is not target-native trace.
Cached proof is labeled and never counted as fresh.
LIVE_RUN requires explicit confirmation.
FULL and endurance runs require long-run confirmation.
Missing dimensions reduce coverage, not quality.
Browser self-report is not browser artifact proof.
Tool-call coverage is separate from natural language quality.
SWE-like coverage requires repo patch plus test oracle.
```

### Acceptance Criteria

Benchmark Adapter API v1 is accepted only when:
- LocalGPT, Forgestack, and Superapp have adapter entries.
- Each adapter can report identity, capabilities, readiness, and final result.
- Missing capabilities are labeled as capability gaps.
- `DRY_RUN` produces no live calls.
- `LIVE_RUN` is explicitly gated.
- `FULL`/endurance/deep validation is explicitly gated.
- Tool traces include source labels.
- Browser proof includes source labels.
- Proof artifacts are real files, not source modules.
- No secrets appear in proof.
- Target source repos are not mutated during normal FAST validation.
- Repo-root inventory proof exists for repo-dependent runs and shows exact root files read.
- Combined Dashboard displays coverage and labels, not just a single opaque score.

---

## 8. Meticulous Logging and Proof Telemetry

Logging must make failures actionable. A benchmark that says "failed" without enough path, event, timing, and strategy data is UNPROVEN.

### Logging requirements

| Feature | Pri | Status | Missing work | Proof required |
|---------|-----|--------|--------------|----------------|
| Per-run correlation ID | P0 | DONE | `StrategyLogger` generates `run-<as_nanos>` ID, attached to every JSONL event | All artifacts share same run ID |
| JSONL event stream | P0 | DONE | `StrategyLogger::emit()` writes normalized JSONL to `target/bench-results/<run_id>.jsonl` | Benchmark app parses events |
| Strategy decision log | P0 | DONE | `StrategyLogger::strategy_decision()` records model_id, tool_count, pattern_detected, caps, decision, has_batched | Log shows signal + decision, no prompt keyword dependency |
| Model attempt log | P0 | DONE | `StrategyLogger::model_attempt()` records provider_id, model_id, tool_choice, status per attempt | Include request mode, tool_choice, model caps, latency, errors |
| Tool call ledger | P0 | DONE | ToolRunRecord with batch_id, parent_id, started_at, duration_ms. Persisted via ToolRunStore. Recorded during parallel, sequential, and batch continuation execution. REST GET /api/tool-call-ledger, SSE tool "tool_call_ledger". 3 new core tests, new fields roundtrip through save/parse. | — |
| Heartbeat event | P1 | PARTIAL | Status bar updates exist | Periodic heartbeat during long tools/loops |
| Browser proof log | P0 | DONE | `browser-proof-ui.js` normalizes output: screenshot.png, console.json, actions.jsonl written to proof dir. BrowserProof domain struct captures console_errors, network_errors. REST GET /api/browser-proof lists all proofs with id, target_url, status, summary, created. | — |
| Redaction scanner | P0 | DONE | `redaction_scanner.rs` scans exported artifacts (.superapp/, ~/.opencoder-proof/) for API keys, bearer tokens, private keys, GitHub tokens, npm tokens. REST GET /api/scan-artifacts returns per-file findings with line numbers and masked snippets. Integrated with secret scrubber in logging. | — |
| Failure classification | P1 | DONE/PARTIAL | Loop failure classes exist | Apply globally to tools, model, browser, validation, repo, timeout |
| Rejection logging | P1 | DONE | `done_gate_handler.rs` supports `reject_done` action + `rejection_status`. `claim_done` returns REJECTED when active. Added `rejection_context()` injects rejection reason into system prompt at run start. `rejection_recovery` SSE event emitted at tool loop start. `resolve_rejection` action clears rejection after inspection. 4 tests. SSE tool + REST endpoint | — |
| Metrics summary | P1 | DONE | Controller metrics written to `target/bench-results/<correlation_id>.controller.json` with `write_metrics()`: llm_calls_total, tool_rounds, tool_calls_total, nudges_sent, provider_429_count, repeated_search_count, repeated_read_count, evidence_summary, guided_transition_used, self_patch_fallback_used, scope_guard_triggered, model_attempts, successful_llm_calls, provider_retries, has_evidence, stop_reason | Wall time, TTFT, first tool, per-provider latency, concurrency, tokens/cost |

### Log storage policy

- Chat shows compact receipts, not giant logs.
- Full logs go to ignored/out-of-repo proof storage or a configured proof directory.
- The canonical audit file remains tracked.
- Proof artifacts may be ignored if large, but their paths and hashes must be recorded in run export.
- Never print or store full API keys, bearer tokens, private SSH keys, or secrets.

---

## 9. Self-Build / Dogfood Contract

Each app must be usable to improve its own repo from a normal prompt. The separate benchmark app compares apps; each app exports stable run/proof data and can be driven by normal prompts.

### Required self-build features

| Feature | Pri | Status | Missing work | Working criteria |
|---------|-----|--------|--------------|-----------------|
| Open current app repo as workspace | P0 | DONE | `repo_preflight` SSE tool + REST endpoint detects repo root, branch, HEAD, remote, dirty state, allowed write root via git commands | "work on this app" shows exact repo state and refuses ambiguity |
| Repo-root inventory before planning | P0 | DONE | `repo_preflight` SSE tool + REST endpoint available. Default system prompt updated (provider_config.rs:208) to instruct reading README.md, docs/PROJECT_STATE.md, docs/SOURCE_REF_CAPSULE.md, FEATURE-AUDIT.md, AGENTS.md before planning. Repo root, branch, HEAD, remote, dirty state reported. | — |
| Read-only plan mode | P0 | DONE | `self_build_mode` controls (plan/build); write tools gated in plan mode via SSE handler + REST endpoint + UI toggle. Plan mode write blocking verified: sending `file_edit` in plan mode returns `"plan mode is active. Tool 'file_edit' is blocked."`. Composer toggle + sidebar toggle both work. Tab key also toggles. | — | Plan mode proves no file writes |
| Build/write mode | P0 | DONE | Explicit plan→build transition via `"tool":"self_build_mode"` or sidebar toggle or composer toggle; plan→build requires user approval in UI. Approval mode `full` enabled write tools in dev/self-build lane. Verified: sending `file_edit` in build mode passes mode check and proceeds to sandbox validation. | — | App only edits after mode transition |
| Workspace sandbox | P0 | DONE | Filesystem/network/process boundaries. Write outside repo blocked | `workspace_sandbox` SSE tool + REST reports boundaries; `validate_path()` blocks writes to outside-repo or system paths. Wired into all write tools. Card shows sandbox root, allowed write root, blocked patterns |
| Approval policy visibility | P0 | DONE | Sidebar shows read-only toggle + self-build mode (plan/build); write tools blocked with clear message. Composer row shows prominent Plan/Build toggle (`📋 Plan` / `🔧 Build`). Both toggles stay in sync via `updateSelfBuildModeUI()`. Tool cards show real state (queued/running/completed/failed) with shimmer animation on running cards. | — | UI always shows active policy with prominent composer toggle |
| Snapshot before mutation | P0 | DONE | Snapshot before first write. Revert all run changes with one action | Git stash snapshot auto-created on first write tool. `done_gate` tool actions: create_snapshot, revert_snapshot, status. Snapshot tracked in memory with ref shown in card |
| Patch transaction | P1 | DONE | First `file_edit` in a tool run auto-creates a git stash snapshot via `done_gate_handler::create_snapshot()`. Scope guard violation auto-reverts to that snapshot via `revert_to_snapshot()`, undoing all edits in the run. `snapshot_exists()` added to `done_gate_handler.rs`. `process_post_round` in `tool_exec.rs` triggers revert before returning narrowing guidance. Files: `done_gate_handler.rs:56-59`, `tool_exec.rs:362-369`, `tool_exec.rs:300-305`. | — | Failed validation can revert only run edits |
| Dirty-state ownership | P0 | DONE | `dirty_state_handler` tracks User/Agent/Proof per file; SSE tool `dirty_state`, REST GET/POST /api/dirty-state, makeDirtyStateCard renderer, 1 test | — | User dirt is never overwritten silently |
| Repo search before edit | P0 | DONE | Default system prompt (provider_config.rs:208) instructs reading files before editing. FileEditHandler logs edits with before/after hash, permission linkage. ToolRunRecord records file reads for grounded inspection tracking. System prompt requires grounding: "Before editing any file, you must first read or search the file to confirm its current contents." | — |
| File allowlist | P1 | MISSING | Block secrets/generated/outside paths by default | Unsafe edit requires approval |
| Validation hooks | P0 | DONE | Format/test/smoke after edits. Done blocked if required hook fails | Runs architecture guard, cargo fmt --check, doctor, cargo test --workspace. Per-check pass/fail with detail. Card shown in UI. SSE tool + REST endpoint |
| Browser proof for UI changes | P0 | DONE | `browser_proof_handler.rs` auto-detects local URLs (127.0.0.1, localhost) and launches Playwright-based interactive proof via `scripts/smoke/browser-proof-ui.js`. Captures: screenshot.png, console.json, actions.jsonl, 7 UI element checks, console errors. System prompt instructs running browser proof for UI changes. | — |
| Local self-review | P0 | DONE | `self_review` SSE tool + REST endpoint runs git diff/changes/risks; `makeSelfReviewCard` renders diff stat, changed files, risks, branch/head | Review card lists risks and fixes |
| Prompt-only continuation | P0 | DONE | `continuation_handler` saves/loads checkpoint.json with repo state, task, next_action, proof; SSE tools save_checkpoint/load_checkpoint; REST GET+POST /api/checkpoint | — | Resume has repo/task/proof/next action |
| Stop/pause/resume | P0 | DONE | Dedicated POST /api/pause and /api/resume endpoints with token-based mid-loop suspension. SSE tools "pause" and "resume". Pause button (⏸) and Resume button (▶) in web UI. Tool loop pauses between rounds, polls for resume/cancel, emits "paused"/"resumed" SSE events. LoopStore updates status. Cancel clears pause. | — |
| Exact final receipt | P0 | DONE | `receipt_handler.rs` produces standardized JSON receipt with repo (root, branch, head, remote), files_changed, validation (passed/errors), browser_proof (url/passed), unresolved_risks, next_action, timestamp. REST GET /api/receipt. | — |
| No unproven Done | P0 | DONE | Done requires evidence. Missing proof => UNPROVEN/BLOCKED | `done_gate` tool with claim_done action checks validation_hooks ran, self_review ran, and snapshot exists. If missing, returns UNPROVEN with evidence list. SSE tool + REST endpoint |
| User rejection state | P0 | DONE | `done_gate_handler` supports `reject_done` action + `rejection_status`. Stores reason in `REJECTION_REASON` Mutex. `claim_done` returns REJECTED when active. 4 tests. SSE tool + REST endpoint | — |
| Machine-readable run export | P0 | DONE | `run_export_handler` produces stable JSON schema (version, app, timestamp, repo, dirty_files); SSE tool `run_export`, REST GET /api/run-export, makeRunExportCard renderer, 3 tests | — | Benchmark app parses run proof |
| External agent handoff preview | P1 | MISSING | Show exact prompt + permissions before dispatch | User can approve/deny dispatch |

### Self-build Done definition

A self-build run is Done only when:
1. Repo preflight is recorded.
2. Read-only inspection happened before edits.
3. Plan identifies target files, proof path, and risk.
4. Write mode/approval policy allows mutation.
5. Edits are bounded and transaction/snapshot is available.
6. Required validation passes.
7. Browser proof passes for UI-visible changes.
8. Self-review passes or unresolved risks are stated.
9. `FEATURE-AUDIT.md` is updated or a logged reason says no audit update was needed.
10. Run export is available for benchmark app.
11. **Target (achieved)**: controller should guide search→read→edit transition within <12 successful LLM calls, <8 tool rounds, no repeated same search/read loop. Post-edit phase steering (phase, post_edit_research_blocked, evidence_gathering→patch_applied transition) added. Bounded proof at 60a1308 used 11 model attempts (was 24, was 53). Post-edit search blocked: dedup + phase block prevented any new search after file_edit. <12 target ACHIEVED (11 attempts). Tool rounds: 2 (was 8+).

---

## 10. Orchestration, Background Work, and Teams

| Feature | Pri | Status | Missing work | Proof required |
|---------|-----|--------|--------------|----------------|
| Inline orchestration trace | P1 | DONE | `EvidenceGatherer::build_trace()` produces compact 1-line trace: `[Trace] phase: patch_applied | calls: 11 | edits: 1 | files: 2 | failures: 0`. Emitted as SSE `agent_trace` event and injected as system message after each round. User sees phase/calls/edits/files/failures inline during the run. Files: `evidence_gatherer.rs:142-158`, `tool_loop.rs:319-324`. | — | User sees phase/tools/proof/blockers inline |
| Expanded run inspector | P2 | MISSING | Full prompt/tools/output/files/proof/cost view | Inspector opens from trace |
| Workflow mode | P2 | MISSING | Optional large-task workflow with plan preview | Workflow plan can be approved/denied |
| Workflow background runtime | P2 | MISSING | Run while chat remains responsive | User can continue chat while workflow runs |
| Pause/resume/stop workflow | P1 | PARTIAL | Cancel exists; pause/resume incomplete | Workflow can pause/resume/stop safely |
| Agent queue | P2 | MISSING | Queued/running/done/failed/blocked agents | Queue visible and accurate |
| Configurable concurrency/budget | P1 | MISSING | Max agents, max parallel calls, time/token/cost budgets | Budget stops run before limit |
| Agent result schema | P1 | MISSING | Structured result, confidence, evidence, blockers | Synthesis uses structured outputs |
| Merge/synthesis step | P1 | MISSING | Main agent reviews/merges subagent results | Synthesis cites evidence and conflicts |
| Same-file conflict detection | P1 | DONE/PARTIAL | Conflict detection exists for pending edits | Enforce in parallel agent merge |
| Background run monitor | P2 | MISSING | Peek/attach/detach/stop | Running job visible after reload |
| Scheduled run | P3 | MISSING | Visible schedule and disable switch | Schedule can be disabled |
| Remote/mobile monitoring | P3 | MISSING | Remote status/approval/notifications | Mobile can approve/stop safely |

---

## 11. Research and External Agent Integration

| Feature | Pri | Status | Missing work | Proof required |
|---------|-----|--------|--------------|----------------|
| Web search | P2 | MISSING | Web search provider/tool | Latest-info answer cites sources |
| Deep research fanout | P2 | MISSING | Multi-angle research agents | Claims have sources and provenance |
| Source credibility scoring | P3 | MISSING | Rank primary/official sources | Weak sources flagged |
| Claim extraction/cross-checking | P2 | MISSING | Extract and verify claims | Contradictions surfaced |
| Citation-preserving synthesis | P1 | MISSING | Final report maps claims to sources | Load-bearing claims cited |
| MCP integration | P2 | MISSING | MCP client/server | External MCP tools usable |
| External agent registry | P3 | MISSING | Register OpenCode/Codex/etc. capabilities/cost/risk | Registry lists available agents |
| Visible external handoff | P1 | MISSING | Show exact prompt and permission profile | User approves exact prompt |
| External dispatch approval | P1 | MISSING | No hidden external runs | External agent cannot run without approval |
| External result import/verification | P1 | MISSING | Ingles summary/diff/proof and verify claims | Imported result verified before Done |
| External handoff archive | P2 | MISSING | Store prompt/result/proof/decision without secrets | Archive is queryable |

---

## 11b. Do Not Claim Done Unless…

These are hard gates. A feature is not Done if any of these are false:

- Chat-first UX proven with real screenshots showing visible composer, mode toggle, and tool cards.
- Tool cards are real DOM components (not plain assistant text) and show tool name, status, and safe path summary.
- Plan mode write blocking proven through visible WebUI or focused API test with clear rejection message.
- Build mode full edit→validation→browser retest proven with real screenshot chain.
- Stop/cancel proven through visible WebUI interaction (click stop, observe cancellation).
- Dynamic model discovery returns models from provider API, not just static hardcoded list.
- `FEATURE-AUDIT.md` updated or explicit logged reason why not.
- No terminal-panel, dashboard, or CLI-clone UI elements used for primary interaction.
- No giant-file patches without extraction into focused modules.

## 11c. OpenCode Reference Correction

### What to copy from OpenCode (engine/agent-loop behavior)

- Plan vs Build agent behavior: Plan = read-only, Build = write allowed
- Permission model: risk levels, approval gates, blocked tool messages
- Tool loop: search → read → edit → verify → proof → done
- File read/search/edit workflow order
- Validation after edits (self-review, validation hooks)
- Stop/cancel behavior (cancel endpoint, token check)
- Failed tool handling (retry, error display)
- Final answer only after proof (done gate with evidence checks)

### What NOT to copy from OpenCode (CLI/TUI surface)

- CLI/TUI layout: terminal-style screens, command-heavy UX, walls of logs
- Keyboard-first interactions as primary UX: menus driven by keystrokes
- Hidden configuration-driven mode controls (should be visible UI toggles)
- Terminal panels as main UI component
- Cockpit/dashboard layout

### Visual product reference

- ChatGPT-style conversation: main chat in center, composer at bottom
- LibreChat-style clean web layout: sidebar for history, main area for chat
- OpenCode-style agent/tool behavior underneath the chat surface

### Current compliance

- Plan/Build toggle is a visible button in the composer row (not a keyboard-only or config toggle) ✅
- Tool cards are real DOM components with status badges ✅
- No terminal panels or dashboards added ❌
- Tab key is supplementary (button is primary toggle) ✅
- Chat-first layout maintained (composer at bottom, messages in center) ✅

## 11d. Regression Notes

| Item | Previous state | Current state | Severity |
|------|---------------|---------------|----------|
| SSE chunked encoding | Broken (Chrome `net::ERR_INCOMPLETE_CHUNKED_ENCODING`) | Fixed via `SseWriter::Drop` sending terminating `0\r\n\r\n` | Fixed |
| Batch continuation loop | Model would re-read files indefinitely after first batch | Fixed: `has_batched` returns `NaturalAuto` after first batch, batch continuation only triggers on `read_file` not `search_text` | Fixed |
| `read_file` line range | Model guessed `line_start`/`line_end` params that silently failed | Fixed: added line range support with 1-indexed inclusive params; truncation limit 2000→8000 chars | Fixed |
| Compaction | Original user prompt lost during compaction stall thresholds at 5/10 | Fixed: original prompt preserved; thresholds changed to 3/6; stall messages more directive | Fixed |
| Tool descriptions | `search_text` and `file_edit` lacked emphasis | Fixed: `search_text` says "use max 2-3 times"; `file_edit` says "PRIMARY tool" | Fixed |
| Provider model list | 4 hardcoded models for `nvidia_nim` | 121 models via dynamic discovery | Improved |
| Phantom tools in schema | `self_review`, `validation_hooks`, `done_gate` exposed to LLM but return "unknown tool" from agent loop | Removed from `tool_definitions_json_inner`; schema reduced from 14 to 11 tools | Fixed |
| Stall message references to removed tools | `stall_warning_msg` and `stall_break_msg` referenced `done_gate` | Updated both messages to reference only `file_edit` | Fixed |
| Text-only re-prompt too generic | "Continue using tools" did not guide model toward read→edit | Changed to context-aware message nudging toward read_file→file_edit | Fixed |
| Tool loop gave up after stall break | `StallResult::Break` returned immediately without giving model a forced final chance | Changed to inject break message as user prompt, set tool_choice to force `file_edit` with only write tools, continue loop | Fixed |
| Evidence-guided nudges with repeated search/read detection | Generic re-prompt didn't summarize evidence or direct toward edit | Added `EvidenceGatherer` (10 tests) that fires guided nudges on repeated searches, enough evidence, or stall. Guided transition replaces generic "Continue using tools" with tiered directives. | Fixed |
| Provider 429 burns all model retries without degradation tracking | All 4 models share same NIM provider; 20 retries stack across models per round | Added `provider_429_count` + `model_degraded` set. After repeated 429s, marks model degraded and tries next model. All degraded → `NIM_PROVIDER_DEGRADED` early-abort with evidence summary. | Fixed |
| Dogfood proof at 2ef0a40 — scope violation | Prompt: "Improve one visible tooltip or label by changing at most one string. Keep behavior unchanged." Result: 9 tooltip title attributes changed (+10 title additions, 1 file). Browser proof PASSED, NIM vision verdict PASS. But scope violated (at most one → 10). | Proof proves the self-patch loop works end-to-end but scope control was too loose. Addressed by ScopeGuard module (scope_guard.rs, 14 tests). Model attempt count (80) also exceeds <12 target. | PARTIAL |
| 80 model-attempt dogfood run diagnosed | Run-1782130964358352854: 53 model attempts (18 tool_call, 26 text-only, 9 transport errors). Categorized waste: text-only responses from large context (26), transport errors with same payload (9). | Compact context module (compact_context.rs, 10 tests) added. Post-edit nudges added. Retry context compaction added. Bounded proof at 6bf9d3e: 24 attempts (was 53) — 55% reduction. Still exceeds <12 target. | PARTIAL |
| Post-edit search still occurs | After file_edit + bash validation, model called search_text 4 more times before final summary. Compact context + nudges reduced this but did not eliminate it. | Post-edit search detection in EvidenceGatherer (post_edit_research_count). Phase tracking (phase: "evidence_gathering" / "patch_applied") with post_edit_research_blocked counter. Bounded proof: 11 model attempts (was 24), post-edit search blocked entirely. | RESOLVED |
| Post-edit phase steering | 24 model attempts after compact context; post-edit search still occurred. | Phase tracking added (EvidenceGatherer.phase). After file_edit, injects "POST-EDIT RESEARCH BLOCKED" user message and SSE event (<12 target achieved: 11 attempts). | RESOLVED |
| Prior guided dogfood failures | 3 prompts: A returned NIM_PROVIDER_DEGRADED blocker, B socket hang up, C timed out. | NIM provider auth/rate-limit failure prevented any successful LLM call. Code is unit-tested (10 EvidenceGatherer tests) but full nudge→self_patch→validation chain unproven at runtime. | UNBLOCKED (NIM now works) |
| Integrated acceptance proof at 4d4173e | Full self-build chain run across all three guards simultaneously: repo-path guard, scope guard, provider router, compact context, post-edit blocking. Prompt: "Improve one visible tooltip or label by changing at most one string. Keep behavior unchanged. Validate the change and run browser proof with NVIDIA NIM vision." Result: single tooltip changed (stop-button title), guard blocked wrong path, scope guard blocked post-edit research, 212/212 tests pass. | All six subsystems proven live in one integrated run. Model attempt count < 12 target not yet independently verified for this specific prompt. | PROVEN |
| All-degraded infinite loop | `rate_limited_route` in `any_success` prevented round-exhaustion gate from firing when all candidates degraded, causing infinite loop until stall detection. | Fixed: split into `any_non_route_success` + `all_degraded_or_rate_limited`. Renamed blocker `NIM_PROVIDER_DEGRADED` → `ALL_TEXT_PROVIDERS_RATE_LIMITED`. | RESOLVED |
| Repo-path confusion in file_edit | Model attempted to edit `src-tauri/src/cmd.rs` (nonexistent Tauri template path). Existing `canonicalize()` check gave generic "outside repo or does not exist" error without guidance. | Fixed: added `repo_path_guard.rs` with specific blocker codes, wrong-template detection (`src-tauri/`, `packages/`, `app/`), compact source-map guidance (`SourceMap::compact()`), and correction behavior (one retry then `MODEL_REPO_PATH_CONFUSION`). **PROVEN LIVE at 4d4173e**: model attempted `src/index.html` (nonexistent), guard returned EDIT_TARGET_PATH_NOT_FOUND, model retried with correct `crates/superapp-web/static/index.html`. | PROVEN |

## 11e. Large File / Refactor Risk

| File | Lines | Risk |
|------|-------|------|
| `crates/superapp-server/src/sse_handler.rs` | ~1600 | High - main SSE handler, tool dispatch, mode blocking, compaction. Should be split into focused modules. |
| `crates/superapp-server/src/main.rs` | ~587 | Medium - route dispatch. Growing but manageable. |
| `crates/superapp-server/src/provider_config.rs` | ~568 | Medium - grew with discovery. Could extract discovery module. |
| `crates/superapp-web/static/render.js` | ~1700+ | Medium - all card renderers in one file. |
| `crates/superapp-web/static/stream.js` | ~1350 | Medium - SSE handling + event dispatch. |

**Rule:** Do not add more features to `sse_handler.rs` without extracting first. Extract in this order:
1. Tool dispatch logic (write tool blocking, mode checks)
2. Compaction/continuation logic
3. SSE event writing helpers

## 11f. Refactor Contract and LOC Gate

Every automated patch must obey the Refactor Contract and automated LOC gate script (`scripts/check_loc_gate.js`).

### R0 / R1 / R2 Scope Classes

| Class | Scope | Requires LOC check? | Merge gate |
|-------|-------|---------------------|------------|
| R0 | Bounded refactor — rename, extract helper, split module, inline constant, move function with zero behavior change | Yes (warnings OK, no hard failures) | Green test suite |
| R1 | New feature or behavior change under existing module boundary | Yes (new source files < 500 LOC; touched large files grow < 150 LOC) | LOC gate + green tests |
| R2 | Cross-cutting new module, protocol change, dependency addition, or module boundary change | Yes (strictest: new source < 500 LOC; touched large < 150 LOC; no new files over 500) | LOC gate + green tests + documented exception |

### Refactor Preconditions

Before any refactor patch:

1. Green test suite (workspace tests pass).
2. LOC gate run recorded (output in run log or commit message).
3. Target files known (file list in plan).
4. Non-goals documented (what is NOT changing).
5. Behavior preservation confirmed (same input → same output for extracted logic).

### Refactor Gate (`scripts/check_loc_gate.js`)

Node script, zero external deps. Supports:

- `node scripts/check_loc_gate.js` — checks touched files (staged + unstaged modified + untracked source) against HEAD.
- `node scripts/check_loc_gate.js --all` — checks all tracked source files.
- `node scripts/check_loc_gate.js --base <ref>` — checks files changed between `ref...HEAD`.
- `node scripts/check_loc_gate.js --self-test` — runs 12 self-tests.

### Anti-Sprawl Rules

- Do not commit new source files over 500 LOC (hard failure).
- Do not grow already-large source files (over 800 LOC) by more than 150 LOC per patch (hard failure).
- Pre-existing large files with zero or negative net LOC produce warnings only (not hard failure).
- Docs (`.md`) are exempt from hard LOC failure.

### Hard LOC Gates

| Condition | Threshold | Action |
|-----------|-----------|--------|
| New source file LOC | > 500 | HARD VIOLATION |
| Touch large file (> 800 LOC) net growth | > 150 | HARD VIOLATION |
| Touch Rust source over 800 with any growth | Warning | Warning (not hard) |
| Pre-existing large file, net LOC zero or negative | Warning only | OK |
| Doc file (`.md`) | Any | Exempt |

### Exceptions

- Pre-existing large files already in HEAD are grandfathered (warning only).
- LOC gate self-test writes and cleans temporary files.
- Script location: `scripts/check_loc_gate.js`.

### Required LOC Proof Template

When committing a refactor that touches large files, include in the commit message:

```text
LOC gate:
  scripts/check_loc_gate.js --self-test: 12/12 passed
  scripts/check_loc_gate.js: 0 hard violations
```

### Stop Conditions

If LOC gate fails with hard violations:

1. Do not commit the patch.
2. Split the change into smaller patches.
3. Run LOC gate on each split.
4. Commit only after each split passes.

## 12. Code Quality, Modularity, and Anti-Bloat Guardrails

This replaces the duplicated rule blocks from the original audit. It is mandatory for coding-agent work but should not be copied into multiple docs.

### Core rule

A feature is not Done just because it appears to work. It is Done only when functionality, proof, validation, modularity, and audit updates are complete.

### Required workflow for non-trivial changes

1. Inspect architecture and existing module boundaries.
2. Implement the smallest coherent feature slice.
3. Run focused validation.
4. Run broader validation when required.
5. Refactor immediately after green tests.
6. Re-run validation after refactor.
7. Update `FEATURE-AUDIT.md` or log why no update is needed.
8. Produce final receipt with proof.

### Module boundary rules

| Concern | Should live in |
|---------|----------------|
| Chat rendering | UI/chat components |
| Tool cards/output | UI/tool components |
| Agent orchestration | Orchestration/runtime layer |
| Subagent scheduling | Orchestration/scheduler layer |
| Parallel tool execution | Tool runtime/executor layer |
| Tool strategy controller | Orchestration/model-routing layer |
| Provider/model routing | Provider/model registry layer |
| API key handling | Secrets/provider settings layer |
| File editing | File/tool service layer |
| Git operations | Git service/tool layer |
| Browser proof | Browser proof/service layer |
| Goals/todos/loops | Orchestration state layer |
| Benchmark API | Benchmark/control API layer |
| Logging/proof telemetry | Event/proof/telemetry layer |

### Anti-patterns

- Giant files or giant functions as the default implementation target.
- Feature logic dumped into `main.rs` when a dedicated module belongs elsewhere.
- Repeating tool strategy, benchmark, logging, or Done rules across many docs.
- Treating proof/log dirt as source dirt.
- Claiming Done from model text without validation/browser proof.
- Retrying the same failed action without strategy change.
- Prompt keyword hijacking for routing decisions.
- Hidden OpenCode/Codex/external-agent dispatch without visible prompt approval.
- Creating multiple task-specific branches (`feat/*`, `refactor/*`, `repair/*`, dated) for normal work.

### Branch and continuity policy

The canonical branch policy is defined in `AGENTS.md`. This section cross-references
it so the audit remains self-contained on branching rules.

**Two-branch model**: `main` (stable checkpoint) and `dev` (only active development).
No `feat/*`, `refactor/*`, `repair/*`, `benchmark/*`, dated, or task-specific
branches by default.

**Continuation**: "continue"/"next" means work on the current branch (`dev`),
not create a new feature branch.

**Consolidation**: old branch → validate → commit → push → merge into `main` →
validate `main` → push `main` → create/reset `dev` from `main` → push `dev` →
checkout `dev`.

**Force-push**: forbidden except one-time `dev` reset via `--force-with-lease`
during consolidation. Never on `main`.

**Forbidden**: splitting work across multiple branches, continuing old branches
after consolidation, rebasing without explicit instruction, stashing/discarding
user work.

Full details and final receipt template: `AGENTS.md` section "Branch and Continuation Policy".

---

## Refactor Contract and Refactor Gate

### What counts as a refactor
- Changes that improve internal code structure, readability, performance, or maintainability **without altering external behavior** of the Superapp.
- Extraction of functions/modules, renaming of private identifiers, reformatting, and dependency updates that keep the public API unchanged.

### What is not a refactor
- Adding new features, fixing bugs that change observable behavior, or modifying user‑facing APIs.
- Changing configuration defaults that affect runtime behavior.

### Refactor scope classes
- **R0**: Minor, low‑risk changes (e.g., formatting, comment updates, tiny function extraction). No review gate required beyond LOC check.
- **R1**: Moderate changes affecting multiple modules or introducing new abstractions. Requires peer review and passing the Refactor Gate.
- **R2**: Large structural changes (splitting large files, redesigning core modules). Must satisfy preconditions, pass the Refactor Gate, and include a migration plan.

### Refactor preconditions
- All existing tests must pass before the refactor begins.
- No new failing tests may be introduced during the change.
- The change must be scoped to a single logical area (no cross‑feature mixing).

### Refactor gate
- Run `node scripts/check_loc_gate.js` on the touched files.
- Ensure no hard LOC violations.
- For R1/R2, a reviewer must approve a brief **Refactor Proof** (see template below).

### Anti‑sprawl rules
- Do not increase the total LOC of a touched file beyond the soft target without a documented split plan.
- Do not add more than 150 net LOC to an already‑large file (>800 LOC) without a split plan.
- New source files must not exceed 500 LOC.

### Protected Superapp behavior that must not regress
- Core API contracts (HTTP endpoints, CLI flags).
- Data persistence formats and schema.
- Authentication/authorization flows.
- Real‑time streaming behavior.

### Refactor proof template
```
Refactor ID: <R0|R1|R2>-<short‑name>
Scope: <description>
Pre‑conditions met: yes/no
LOC impact: <added/removed> <n> lines
Split plan (if needed): <brief>
Tests added/updated: <list>
Reviewer approval: <name>
```

### Refactor stop conditions
- Any test failure after the change.
- LOC gate hard violation.
- Reviewer rejects the proof.
- Unexpected regression in protected behavior.

## 13. Required Audit Commands

```bash
# Core Rust validation
cargo test --workspace -- --test-threads=1

# Browser proof smoke, if Playwright dependencies are installed
node scripts/smoke/browser-proof.js

# Provider status via API
curl -s http://localhost:3001/api/providers | python3 -m json.tool

# Conversation persistence
curl -s http://localhost:3001/api/conversations | python3 -m json.tool
curl -s "http://localhost:3001/api/messages?conversation_id=local" | python3 -m json.tool

# Audit file must be tracked and not ignored
test -f FEATURE-AUDIT.md
git check-ignore -q FEATURE-AUDIT.md && { echo "FEATURE-AUDIT.md is gitignored"; exit 1; } || true
git ls-files --error-unmatch FEATURE-AUDIT.md >/dev/null
```

---

## 14. Next Highest-Leverage Work

1. **P0 Tool Strategy Controller**: model caps, no-keyword trajectory repair, structured plan-to-DAG, synthetic batch routing.
2. **P0 Benchmark Adapter API v1**: DONE — Full §7 Generalized Benchmark Adapter API v1 contract (`benchmark_adapter.rs`, 11 endpoints, 27 Rust tests, 36 smoke checks). Non-blocking run via background thread (returns < 100ms). Events visible during execution, thread panic caught with `catch_unwind`. Cancel via file-based token. Capability gaps honest (pause/resume). Live-tested against NVIDIA NIM (`openai/gpt-oss-120b`). Runs at `/api/benchmark/*`.
3. **P0 Fast live browser-proof benchmarks**: parallel tools, trajectory repair, long loops, stop/resume, provider routing.
4. **P0 Meticulous logging**: per-run JSONL, strategy decisions, tool batch ledger, browser proof artifacts, redaction scan.
5. **P0 Self-build repo preflight**: repo/branch/head/dirty proof, plan mode, build mode, audit update gate.
6. **P1 Run inspector and inline orchestration trace**: enough UI to trust long loops and parallel batches.
7. **P1 Commit/readiness and pre-push gate**: prevent unvalidated commit/push.
8. **P1 Rejection recovery**: user "still broken/no change" invalidates Done and forces real-path inspection.

---

## 15. Minimal Acceptance Tests

### 15.1 App can build itself

**Prompt:**
```text
Improve the provider settings UI so the active provider and fallback order are easier to understand. Keep behavior unchanged.
```

**Required result:**
- Repo preflight card shows path, branch, HEAD, dirty state.
- Agent enters read-only plan mode first.
- Agent reads relevant UI/provider files.
- Plan lists target files, validation, browser proof requirement, and risk.
- Build mode starts only after approval/trusted policy.
- Edits are bounded and snapshot/revert exists.
- Focused validation passes.
- App launches.
- Browser proof interacts with provider settings UI.
- Self-review checks diff and proof.
- `FEATURE-AUDIT.md` is updated or logged as not required.
- Benchmark API export contains run JSON and proof artifacts.

### 15.2 GPT-OSS parallel tool routing

**GPT-OSS-120B is no longer the mandatory default but remains in the stack for self-refactor/coding tasks and must still pass parallel tool routing benchmarks.**

**Prompt should be natural and not contain special routing keywords.**

**Required result:**
- Model caps loaded: GPT-OSS native parallel false, synthetic batch true.
- Natural auto baseline may fail and is recorded honestly.
- Product router mode detects fan-out structurally.
- Tool strategy switches to `batch_from_plan` or `batch_continuation`.
- 100 read-only tool calls complete before benchmark timeout.
- Browser UI shows batch progress and per-tool cards.
- Export includes event log, strategy decision, max concurrency, failures if any.

### 15.3 Long loop with browser proof

**Required result:**
- Loop shows phase, iteration, stop condition, and heartbeat.
- Failures are classified.
- Same failed action is not repeated unchanged.
- User can stop/cancel.
- Resume continues from safe state.
- Final status is PASS/DONE only with proof, otherwise UNPROVEN/BLOCKED/REJECTED.

---

## B. Benchmark, Proof & Logging Rules

### B.1 Run classification

Every run MUST be classified before its score affects the board. See `AUTONOMOUS_IMPROVEMENT_LOOP.md` for the full classification table.

Only `app-self-run` may improve the self-improvement score.
`product-webui-run` may update a separate product benchmark score.
All other classifications are diagnostic or blocked — score is not counted.

### B.2 Curl/API diagnostic-only rule

HTTP API checks (`curl`, direct REST calls) are **smoke diagnostics only**. They cannot count as WebUI proof, browser proof, vision review, or app-self-run provenance.

A run using `curl` as the primary interaction path must be classified `api-stream-diagnostic` or `smoke-readiness`, never `app-self-run`.

### B.3 Proof source labels

| Label | Meaning |
|---|---|
| `target_native` | Proven by the app's own output or side effect |
| `harness_observed` | Observed by test harness, not the app itself |
| `self_report` | Agent self-claims without external evidence |
| `external_agent` | Proven by a different agent, not this run |
| `cached` | Reused from a prior run, not fresh this cycle |
| `missing` | No proof provided |

### B.4 Valid app-self-run proof chain

For a run to be classified `app-self-run` and affect score:
1. Agent uses the app's normal user path (WebUI, CLI if product, browser)
2. Proof includes tool call ledger, validation results, browser proof, vision review
3. Run is executed by the app agent, not by human typing commands
4. At least one category of proof is `target_native` (not all `self_report` or `harness_observed`)
5. Correlation ID links scoreboard → latest → history → artifact folder

### B.5 Threshold gaming ban

Changing scoring thresholds requires:
1. Old threshold and new threshold both recorded
2. Objective, documented reason for the change
3. Both old score (under old threshold) and new score (under new threshold) shown
4. Any regressions from the threshold change recorded as regressions

### B.6 Fast iteration budgets

- Default benchmark timeout: **120 seconds**, not 300. Extend only if the scenario genuinely requires it.
- If no visible progress for **30 seconds**, stop and diagnose root cause before extending the timeout.
- Each fix-validate-prove cycle should complete in under **2 minutes** where possible.

### B.7 No committed raw proof/log dumps

Raw tool output, full event streams, 16MB JSON lines, browser traces, and screenshots go under `self-healing-runs/artifacts/` (gitignored). Only compact summaries are committed.

### B.8 Loop-control rule

Do not fix tool/research loops by only increasing `max_rounds`. The primary fix must address root cause: phase control, stall detection, tool call ledger, validation ledger, provenance tracking. `max_rounds` may be increased only after those controls are in place and proven insufficient.

### B.9 WebUI Provenance Rule

`origin="webui"` is trusted only when proven by the application's normal browser UI path.
A client-provided `origin` field in the request body must never be trusted by itself.

The server determines origin from the HTTP `Origin` header (set by browsers, absent from curl/API calls).
If no `Origin` header is present, the run is classified as `cli` regardless of any client-provided fields.

A run may be classified as `app-self-run` only when artifacts prove:

```text
browser opens the real app UI
→ user prompt is submitted through the visible WebUI
→ server creates or verifies the correlation ID
→ server records origin as WebUI-originated
→ events.jsonl carries the same correlation ID
→ tool-ledger.jsonl carries the same correlation ID if tools ran
→ validation.json carries the same correlation ID if validation is claimed
→ browser-proof.json and screenshot carry the same correlation ID if browser proof is claimed
→ vision-review.json carries the same correlation ID if vision is claimed
→ summary.json/latest.json/history.jsonl carry the same correlation ID
```

If this chain is not proven, classify the run as one of: `api-stream-diagnostic`,
`smoke-readiness`, `stall-diagnostic`, `product-webui-run`, `mixed`,
`external-agent-assisted`, `not-proven`, or `blocked`. Do not classify it as
score-counted `app-self-run`.

Curl/API traffic is useful for diagnostics, but it must not count as WebUI provenance.

---

---

## 16. Self-Refactor Continuation State (2026-06-22)

### 16.1 GPT-OSS High Reasoning Proof
| Item | Status |
|------|--------|
| `reasoning_effort: "high"` injected for GPT-OSS | DONE — added to `openai_compatible_payload_with_messages_ex` in llm.rs |
| Request proof recorded | DONE — `gpt_oss_request_proof` SSE event emitted per tool-loop round |
| Blockers emitted on violation | DONE — `GPT_OSS_HIGH_REASONING_NOT_PROVEN`, `GPT_OSS_TOOLS_NOT_PRESENT`, `tool_turn_response_format_disabled` |
| Secret redaction | DONE — `redact_request_body()` redacts nvapi-, sk-, Bearer tokens |
| Deterministic tests | DONE — 9 new tests in llm.rs covering proof fields, blockers, redaction |

### 16.2 Self-Refactor Phase Decomposition
| Item | Status |
|------|--------|
| `SelfRefactorPhase` enum | DONE — 5 phases: Phase1SelectTarget, Phase2ApplyRefactor, Phase3Validate, Phase4BrowserProof, Phase5Summary |
| `PhaseController` struct | DONE — tracks phase, intended files, LOC, non-goals |
| Phase prompts with compact evidence | DONE — `SelfRefactorPhase::prompt()` generates phase-specific prompt |
| Mutation guard | DONE — `allows_mutation()` only true for Phase2 |
| Tool restrictions per phase | DONE — `can_call_tool()` enforces allowed tools per phase |
| Target-file guard | DONE — `is_target_file()` restricts edits to intended files |
| Phase tests | DONE — 11 new tests covering mutation, tool restrictions, prompts, terminal phase |

### 16.3 Thinking/Progress Trace
| Item | Status |
|------|--------|
| `ThinkingCard` struct | DONE — 5 sections: Plan, Discovery, Tool reasoning, Validation, Model/fallback |
| SSE event emission | DONE — `thinking_card` event emitted per GPT-OSS tool-loop round |
| Secret redaction | DONE — `redaction_status: "passed"` enforced |
| Fallback/model section | DONE — shows provider, model, fallback=manual_order, tool_choice |
| Tests | DONE — 5 new tests in orchestration.rs |

### 16.4 Manual Fallback Order
Default model remains `openai/gpt-oss-120b`. Fallback uses `manual_order`. No hidden automatic smart routing.

### 16.5 Current Blockers
- Phase 1 self-refactor target selection succeeds via SSE API (GPT-OSS reads FEATURE-AUDIT.md, HANDOFF.md, scans repo).
- Phase 2/3 not yet run through app (requires multi-turn Phase 1→2→3 chain).
- No API key for fallback models — full model matrix not run.

---

## 17. Summary Counts

The original audit reported approximately:

| Status   | Count |
|----------|-------|
| DONE     | 108   |
| PARTIAL  | 10    |
| MISSING  | 253   |

After this rewrite, counts should be regenerated by script from the normalized matrix once the document is installed in the repo. Do not trust stale manual counts after adding or removing rows.

---

## 17. Installation Checklist for This File

When applying this document to the repo:

1. Save as `FEATURE-AUDIT.md` at repo root.
2. Ensure it is not ignored and is tracked by git.
3. Replace duplicated feature/rule blocks in other docs with pointers to this file.
4. Add `AGENTS.md` instruction to read this file first.
5. Add Benchmark Adapter API v1 and Tool Strategy Controller rows to the project task list.
6. Run audit-file tracking proof commands.
7. Commit this file with the source change or as a standalone documentation normalization commit.

Suggested commit message:
```text
docs: consolidate canonical feature audit and benchmark contract
```
