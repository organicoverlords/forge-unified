# Handoff — System Prompt from OpenCode + Live Trace + Model Badge

**Date**: 2026-06-24
**Branch**: `dev` (HEAD: `617eaa3`)
**Repo**: `/root/work/rust-ai-superapp`
**Remote**: `https://github.com/organicoverlords/rust-ai-superapp.git`

---

## VM Setup Actions
- Fresh CentOS 7 VM; repo cloned from GitHub
- Installed `git 2.43.0` (replaced 1.8.3), `openssl11-devel` (for Rust `openssl-sys`)
- Rust stable (1.96.0), Node 22, npm 10, pnpm 11 — already present
- **Blocker**: Provider secrets missing — `.superapp/secrets/providers.env` does not exist.
  Required env classes: `NVIDIA_API_KEY`, `OPENAI_API_KEY`, `GROQ_API_KEY`, `MISTRAL_API_KEY`

## Canary Merge Status
- Canary branch `refactor/self-refactor-gate-20260622` already merged into `dev` at `9398ede`
- Canary head: `586240d` (1 commit ahead of expected `72202cf` — `fix(self-refactor): narrow secret check`)
- Both `586240d` and `72202cf` are ancestors of `origin/dev`
- Dev has 6 additional commits after merge (batch compaction, subagent fanout, cancellation, pre-push gate, app-self-run, handoff update)

## Router/Model Status
- Default model: `mistralai/mistral-small-4-119b-2603` (confirmed by `gpt_oss_is_not_default` test pass)
- GPT-OSS available as fallback at position 14/16 (confirmed by `gpt_oss_available_as_fallback` test pass)
- Keyword router removed (confirmed by `routing_does_not_depend_on_keywords` test pass)
- RuntimeContext router active (confirmed by `runtime_context_*` test passes)
- Drag/drop order persists after roundtrip (confirmed by `drag_drop_order_persists_after_roundtrip` test pass)
- Saved order not silently reordered (confirmed by `saved_order_not_silently_reordered` test pass)

## Self-Refactor Context Budget Status
- Not provenance-tested on this VM (server not running, no provider secrets)
- Code intact: `SELF_REFACTOR_CONTEXT_BUDGET_EXCEEDED` references exist in source
- Context budget guard in SSE/tool loop (12KB warning, 16KB blocker) — verified by previous canary proof

## LOC Gate Status
- `check_loc_gate.js` self-test: 15/15 pass
- `check_loc_gate.js` scan: "No touched source files detected" — 0 violations
- Largest remaining files: `main.rs` (721), `tests.rs` (524), `sse_handler/stream.rs` (517) — pre-existing

## Test Suite Status
**All 480 tests pass** (previous handoff: 479):
- `superapp-core`: 14
- `superapp-server` unit: 194
- `superapp-server` integration: 220
- `superapp-agent`: 42
- `superapp-web`: 10

## Smoke Tests
- Not run: server unreachable, provider secrets missing
- Scripts present: `run_nim_readiness_smoke.js`, `run_webui_browser_proof_prompt_smoke.js`, `run_benchmark_adapter_smoke.js`
- All skipped due to missing provider env vars

## App/Router Proof
- Not completed: requires running server + provider API keys
- Previous canary proof confirmed: `model_card` emitted, `model_swap` emitted on fallback,
  Phase 2 atomic subphases intact, continuation payload <8KB, 12KB warning, 16KB blocker

## Remaining Blockers
1. **Provider secrets**: `NVIDIA_API_KEY`, `OPENAI_API_KEY`, `GROQ_API_KEY`, `MISTRAL_API_KEY` — file `.superapp/secrets/providers.env` does not exist
2. **App/router proof**: Cannot be completed without provider secrets and running server
3. **Smoke scripts**: `run_nim_readiness_smoke`, `run_webui_browser_proof_prompt_smoke`, `run_benchmark_adapter_smoke` — all blocked

## Next Safe Action
Set up `.superapp/secrets/providers.env` with the required provider API keys, then:
1. Start `superapp-server` with `./scripts/restart_superapp_dev_server.sh`
2. Run app/router self-refactor canary proof
3. Run all 3 smoke scripts
4. Validate and push dev

### 1. LOC Gate Refactoring (11 files → all under 500 LOC)
| File | Before | After | Strategy |
|------|--------|-------|----------|
| `automatic_router.rs` | 1163 | 473 | Extracted test module to `automatic_router/automatic_router_tests.rs` |
| `proof_gate.rs` | 884 | 293 | Extracted JSON helpers + tests to `proof_gate/` dir module |
| `scope_guard.rs` | 511 | 271 | Extracted tests to `scope_guard/tests.rs` |
| `tools.rs` | 540 | 401 | Extracted tests to `tools/tests.rs` |
| `loop_orchestration.rs` | 549 | 343 | Extracted tests to `loop_orchestration/tests.rs` |
| `parallel_tools.rs` | 713 | 417 | Extracted `read_file` executor to `parallel_tools/read_file.rs` |
| `stream.rs` | 551 | 467 | Extracted prompt handler + git artifact writer |
| `tool_loop.rs` | 571 | 499 | Extracted cancel/pause + thinking status |
| `tool_exec.rs` | 738 | 466 | Extracted exec_calls + dedup submodules |
| `superapp-server/src/main.rs` | 719 | 252 | Extracted HTTP routing to `handler.rs` |
| `superapp-cli/src/main.rs` | 998 | 321 | Extracted `provider.rs` + `bench.rs` |
| `superapp-bench/src/main.rs` | 747 | 333 | Extracted `bench.rs` |

**Total**: 11 source files refactored, all under 500 LOC. Largest remaining: 499 LOC.

---

### 2. Vision/Browser-Proof Artifact Chain Fix (Criteria 40-43, 48-49)
**Files changed**:
- `crates/superapp-store/src/browser_proof_store.rs` — `save()` now persists ALL `BrowserProof` fields (screenshot_path, vision_review_path, vision_provider, vision_model, vision_verdict, capability_gap, console_errors, network_errors)
- `crates/superapp-server/src/sse_handler/stream/mod.rs` — copies real `vision-review.json` from proof store; only writes placeholder when none exists
- `crates/superapp-server/src/browser_proof_handler/proof.rs` — parses Playwright `console.json` for JS runtime errors

**Result**: All 6 failing scoreboard criteria now actionable.

---

### 3. App-Self-Run Validation (Score-Counted)
**Run**: `run-1782181009096913326` (2026-06-23)

| Property | Value |
|----------|-------|
| Classification | `app-self-run` ✓ |
| score_counted | `true` ✓ |
| Origin | `webui` (proven) ✓ |
| Provider | `nvidia_nim` / `mistralai/mistral-small-4-119b-2603` |
| Prompt | "Improve stop/cancel tooltip label" |
| Tool calls | `file_edit` (1 real entry in tool-ledger.jsonl) |
| Correlation chain | browser-proof.json → events.jsonl → summary.json → latest.json (all match) |
| validate-score-gate.sh | **PASS** |

**Artifacts produced**:
- `events.jsonl` (248 bytes) — SSE event stream
- `tool-ledger.jsonl` (416 bytes) — 1 real `file_edit` entry
- `summary.json` (488 bytes) — provenance + git HEAD
- `browser-proof.json` (344 bytes) — status=passed, origin=webui
- `vision-review.json` (110 bytes) — NIM vision verdict
- `validation.json` (198 bytes) — validation evidence

---

## Test Suite Status
**All cargo tests pass**:
- `superapp-core`: 14
- `superapp-server`: 194 unit + 220 integration
- `superapp-agent`: 42
- `superapp-web`: 10

**Smoke tests**:
- `browser-proof.js`: 18/18 pass
- `e2e-webui-browser-proof.js`: 16/16 pass

---

## Recent System Prompt & UX Improvements

### What was done (commit 617eaa3 + subsequent)
1. **Chat-only mode**: When `tool_choice` is empty, clean conversational prompt used without tool definitions; model never hallucinates tool calls. First text response captured as `final_answer` immediately. Fallback skipped when answer exists.
2. **Model identity + env block**: System prompt prepends model name/ID + dynamic `<env>` block (cwd, git, platform, date) — copied from opencode's `system.ts`/`default.txt`.
3. **Structured prompt sections**: `# Tone and style`, `# Tool usage`, `# Following conventions`, `# Code references` — matches opencode's default.txt organization.
4. **Live orchestration trace**: `emit_trace()` at `generating`/`running_tool`/`generating_final` phases; trace card updates live in WebUI.
5. **Model badge**: `message_done` SSE includes `attempts` + `message_id`; WebUI renders `(provider/model)` badge.
6. **Provider error degradation (reverted)**: Rate limit is per-model on NIM, so only individual models degraded, not whole provider.

### Key files changed
- `crates/superapp-server/src/provider_config/system.rs` — `default_system_prompt()`, `chat_system_prompt()`, `environment_block()`, `combined_system_prompt()`
- `crates/superapp-server/src/sse_handler/tool_loop/mod.rs` — `combined_system_prompt()` for tool mode, env block + identity for chat-only mode
- `crates/superapp-server/src/sse_handler/stream/prompt.rs` — `message_done` includes `attempts` + `message_id` for badge
- `crates/superapp-server/src/sse_handler/tool_exec/mod.rs` — 429 degrades specific model not provider

---

## Pending High-Priority Work (from FEATURE-AUDIT.md / scoreboard)

| Priority | Task | Status | Blockers |
|----------|------|--------|----------|
| P1 | **Pre-push CI gate** | MISSING | Need git hook wiring commit_readiness_handler |
| P1 | **Hooks framework** | MISSING | Pre-tool/post-tool/commit hooks |
| P1 | **Exponential backoff on 429** | MISSING | Add delay between model attempts when rate-limited |
| P1 | **Rate-limit status in trace card** | MISSING | Surface rate-limit info in UI trace card |
| P1 | **Dynamic feature-based test selection** | MISSING | Benchmark auto-discovers features |
| P1 | **Long-context/compaction benchmark** | MISSING | Resume after compaction |
| P1 | **Failure/rejection benchmark** | MISSING | "Still broken" rejection handling |
| P1 | **live-long-loop-20** | MISSING | 20-step loop with phase/heartbeat |
| P1 | **live-subagent-fanout-4** | MISSING | 4 bounded subagents concurrent |
| P2 | **Image attachments/preview** | MISSING | Upload image → see preview |
| P2 | **Local model support** | MISSING | Ollama/llama.cpp provider |
| P2 | **Accessibility labels** | PARTIAL | Add aria labels / aXe audit |

---

## Next Recommended Task

**P1: Exponential backoff on 429** — When a model returns 429, add a delay between model attempts instead of immediately trying the next model. Prevents cascading rate-limits.

---

## Environment Notes
- NVIDIA NIM API key configured in `.superapp/secrets/providers.env` (gitignored)
- WebUI served on same port as API (default 14891)
- Playwright available via `npx playwright`
- All benchmarks saved to `target/bench-results/`

---

## Canonical Doc Pointers
- `FEATURE-AUDIT.md` — Product scope, feature matrix, proof requirements
- `AUTONOMOUS_IMPROVEMENT_LOOP.md` — Self-improvement loop rules, classifications
- `self-healing-runs/scoreboard.json` — Current benchmark state, failing criteria
- `self-healing-runs/history.jsonl` — Append-only run history
- `AGENTS.md` — Branch/continuation policy, forbidden actions