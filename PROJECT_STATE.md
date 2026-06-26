# Forge Unified — Current State

Updated: 2026-06-26

## Current branch

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Server port: `3000`
- Latest code-fix commit before this docs refresh: `74eba32f57e9bfb682effaa202bdeac07f970c35`

## Latest validation state

The previous failed Actions were investigated from GitHub Actions logs.

| Workflow | Previous state at `6d2faa8` | Root cause | Post-fix signal at `74eba32f` |
|---|---:|---|---|
| CI | Failed | `File Size Gate` failed on an oversized Rust file | File Size Gate passed; Test passed; remaining jobs were still running when this docs sync started |
| Build Proof | Failed | `File line gate` failed before cargo check/test/smoke | File line gate passed; Cargo check passed; Cargo test/WebUI smoke still needed final completion check |
| Live WebUI Feature Sprint | Passed | No failure at previous head | Re-running for latest head |

Do not call the latest branch fully green until CI, Build Proof, and Live WebUI Feature Sprint are all complete and successful on the latest HEAD.

## Latest code change

- Split oversized graphify CLI source:
  - Added `crates/unifiedgraph/src/cli.rs` for Clap command and argument definitions.
  - Reduced `crates/unifiedgraph/src/main.rs` to the dispatch entrypoint.
- This preserves the hard 500-line source gate instead of weakening it.

## Features implemented

### Chat & Conversation

- Multi-turn conversation threads with snapshot persistence.
- Conversation create / list / get / delete.
- Chat completion via REST API and WebUI streaming route.
- Cancel / pause / resume API shape exists.
- Message history with metadata.

### Agent & Tool Execution

- Tool calling: file read/write/edit/delete/list/glob/search, web fetch/search, shell, terminal, task, batch parallel, repo info, propose patch, apply_patch, switch mode, browser proof, vision review, graph build/query.
- Parallel tool execution via `futures::stream::buffer_unordered`.
- Tool approval gates through current safety checker.
- `apply_patch` currently parses OpenCode-style patch text for review, but does not yet implement full mutation parity.

### Model & Provider

- Multi-provider support: NVIDIA NIM first, then Groq and OpenRouter when env vars are available.
- FreeLLMAPI-style deterministic provider/model fallback work is in progress.
- NIM provider sends tool definitions and parses tool calls.
- Provider key management via env vars.

### WebUI and Proof

- Bundled root chat UI.
- Live SSE events for run phases, text deltas, tool calls, tool results/errors, and run finish.
- Browser proof route and NIM vision review route.
- Live WebUI Feature Sprint captures screenshot proof and requires a completed human-readable answer.

### CI/CD

- GitHub Actions: CI, Build Proof, and Live WebUI Feature Sprint.
- Hard 500-line gate: `scripts/ci/check-file-lines.sh`.
- Keep proof artifacts in Actions; keep only compact handoff/status docs in git.

## Current gaps, highest priority

| Area | Feature | Priority |
|------|---------|----------|
| Engine | Full OpenCode `apply_patch` behavior | P0 |
| Engine | Source-gated OpenCode system prompt rewrite | P0 |
| WebUI | OpenCode-like tool-part state cards | P1 |
| Engine | Durable session/message/part persistence | P1 |
| Router | Visible routing/fallback receipts and cooldown policy | P1 |
| Engine | Context compaction parity | P2 |
| Benchmark | Artifact-backed adapter contract | P2 |

## What not to do

- Do not weaken the 500-line gate to make CI pass.
- Do not claim full OpenCode parity without an upstream OpenCode source path in `OPENCODE-PARITY.md`.
- Do not commit provider secrets or local proof blobs.
- Do not build multi-user/auth before the core single-user OpenCode-like workflow is solid.
