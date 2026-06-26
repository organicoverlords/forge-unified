# Forge Unified — Current State

Updated: 2026-06-26

## Current branch

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Server port: `3000`
- Latest code commit before this docs refresh: `541e67fe40ef51dff5dc5b2507606dd68f7a0e2c`

## Latest validation state

Latest fully green baselines:

| HEAD | CI | Build Proof | Live WebUI Feature Sprint |
|---|---|---|---|
| `e31d678277c0527d36f14f8eac8fc65f07c3b265` | success | success | success |
| `541e67fe40ef51dff5dc5b2507606dd68f7a0e2c` | success | success | success |

The latest docs-updated HEAD after this sync still needs its own Actions check before merge/green claims.

## Latest code change

### OpenCode `apply_patch` review-metadata slice

Studied upstream sources:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/patch/index.ts`

Forge changes:

- Added `crates/engine/src/tool/patch_ops.rs`.
- Split patch behavior out of `crates/engine/src/tool/task_ops.rs`.
- Updated `crates/engine/src/tool.rs` to wire `patch_ops` and expose `workspace_root` as `pub(crate)` for sibling-module path validation.
- Updated the `apply_patch` tool description to mention path validation and edit-permission metadata.

Behavior now present:

- Accepts `patchText`.
- Rejects empty patch text.
- Rejects empty Begin/End patch text with OpenCode-compatible wording.
- Parses add/update/delete/move hunks.
- Validates all patch and move paths before recording metadata.
- Records per-file metadata, edit-permission metadata, parsed hunks, validated paths, and OpenCode source references.
- Returns human-readable OpenCode-style `A/D/M` summary lines.
- Does not yet mutate files.

### Earlier source-size recovery

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
- `apply_patch` currently parses OpenCode-style patch text for review, validates patch paths, records edit-permission metadata, and returns `A/D/M` summary lines, but does not yet implement full mutation parity.

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
| Engine | Full OpenCode `apply_patch` mutation behavior | P0 |
| Engine | Source-gated OpenCode system prompt rewrite | P0 |
| WebUI | OpenCode-like tool-part state cards | P1 |
| Engine | Durable session/message/part persistence | P1 |
| Router | Visible routing/fallback receipts and cooldown policy | P1 |
| Engine | Context compaction parity | P2 |
| Benchmark | Artifact-backed adapter contract | P2 |

## What not to do

- Do not weaken the 500-line gate to make CI pass.
- Do not claim full OpenCode parity without an upstream OpenCode source path in `OPENCODE-PARITY.md`.
- Do not claim `apply_patch` mutates files until that is implemented and proved.
- Do not commit provider secrets or local proof blobs.
- Do not build multi-user/auth before the core single-user OpenCode-like workflow is solid.
