# Forge Unified — Current State

Updated: 2026-06-27

## Current branch

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Server port: `3000`
- Latest fully green baseline: `a83ddac8542264cf69bd18988cd6e7dc6f518d95` for real OpenCode-style edit approval gating.
- Latest Live WebUI proof artifact: `/mnt/data/live-webui-feature-sprint-proof-a83ddac.zip`.

## Latest validation state

Latest fully green baselines:

| HEAD | CI | Build Proof | Live WebUI Feature Sprint | Notes |
|---|---|---|---|---|
| `0da7281dc0f85bb16906103343d2e9d24827dafa` | success | success | success | OpenCode `apply_patch` mutation slice |
| `65c1cb5f5c534149d4e08000e8553a498767ed00` | success | success | success | Cleaner WebUI tool cards: output/file cards first, metadata collapsed |
| `7f46ea1c0e7498a353fa18a3781b062580105236` | success | success | success | Natural proof note + repo inspection two-prompt proof |
| `e160fa4bf9326c26d5731e9fb474574a4d068b2f` | success | success | success | Compact `repo_info`/`file_list` presentation with raw JSON preserved in metadata |
| `b7b0e7eb88570900ad8e3252d8190004342678fd` | success | success | success | OpenCode `SnapshotPart` persistence and visible WebUI card proof |
| `c3f15e4a5ac9c84fb07a6a49ec87118c97c4c3e7` | success | success | success | OpenCode `FilePart` persistence and visible DOM proof |
| `a0efdb6372cd92ac6b579bd152f009bb3debefbd` | success | success | success | OpenCode `ReasoningPart` safe public summary card proof |
| `84e459ef3bd4d4f88636239c76136617a98b68e3` | success | success | success | OpenCode `CompactionPart` durable request marker proof |
| `a83ddac8542264cf69bd18988cd6e7dc6f518d95` | success | success | success | Real edit approval-before-write gate for `apply_patch` |

The docs-updated HEAD after this refresh still needs its own Actions check before merge/green claims.

## Latest product behavior

### Natural file creation with edit approval

A normal user prompt now creates a pending edit approval first:

```text
Please create a short proof note for this WebUI sprint.
```

Proven behavior:

- Records the user message.
- Executes `apply_patch` locally without depending on a provider.
- Returns a pending OpenCode-style edit permission request before writing.
- Persists `pending_edit_approval`, `permission_request`, `approval_state.status=pending`, and `applied=false`.
- The proof note does not exist before approval.
- The WebUI renders `OpenCode edit permission request`, `Approve edit`, and `Edit approval metadata`.
- Approval route re-runs the patch with `approved=true` and writes the file.
- Approved result persists `approved_via_api=true`, `approval_state.status=approved`, `applied=true`, file events, FilePart, and PatchPart.
- The approved assistant summary is human-readable: `Approved and applied edit ... Updated 1 file`.

### Natural repository inspection proof

A second normal user prompt inspects the repository:

```text
Please inspect this repository and summarize what you find.
```

Proven behavior:

- Runs real `repo_info` and `file_list` tools.
- Presents compact visible output:
  - `Repository status:`
  - `Top-level repository entries`
- Preserves raw JSON under `metadata.raw_output` for details.
- Persists the tool results and assistant summary.
- Browser proof requires the compact output in the live DOM/screenshot.

### OpenCode session parts

Implemented/proofed through `a83ddac`:

- `TextPart` metadata for user/assistant public text.
- `ReasoningPart` card for safe public progress summaries only.
- `SnapshotPart` card for explicit snapshot save.
- `CompactionPart` card for durable compaction request markers.
- `FilePart` card for changed files after approved edits, including `workspace://...` URL.
- `ToolPart` cards for running/completed/error states.
- `PatchPart` card with patch hash and file list after approved edits.

## OpenCode-source work copied so far

Studied upstream sources:

- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/patch/index.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/session/processor.ts`
- `anomalyco/opencode`, branch `dev`, `packages/schema/src/v1/session.ts`
- `anomalyco/opencode`, branch `dev`, `packages/opencode/src/session/compaction.ts`

Forge changes:

- Added `crates/engine/src/tool/patch_apply.rs` for mutation helpers.
- Kept `crates/engine/src/tool/patch_ops.rs` as parser/entrypoint glue.
- Updated `crates/engine/src/tool.rs` to register `patch_apply` and advertise mutation support.
- Added WebUI file-change cards and compact tool-result presentation.
- Added natural local action paths for file creation and repository inspection, using real Forge tools and normal user prompts.
- Added OpenCode-style session part helpers and WebUI cards for text, reasoning, snapshot, compaction, file, tool, and patch parts.
- Added real edit approval-before-write handling for `apply_patch`.

Behavior now present:

- Accepts `patchText`.
- Rejects empty patch text and empty Begin/End patch text.
- Parses add/update/delete/move hunks.
- Validates all patch and move paths before mutation.
- Pauses before mutation and returns an edit approval request unless `approved=true`.
- Applies add/update/delete/move file mutations inside the workspace only after approval.
- Records per-file metadata, diff metadata, edit-permission metadata, parsed hunks, validated paths, and OpenCode source references.
- Returns human-readable `Success. Updated the following files:` and `Updated N file(s)` with `A/D/M` summary lines after approval.
- Emits/persists file-change metadata and renders `ADDED` file cards after approval.
- Presents repo inspection as human-readable output while keeping raw details under metadata.
- Persists safe public `ReasoningPart` summaries without exposing private chain-of-thought.
- Persists durable `CompactionPart` request markers and optional local pruning metadata.

Remaining parity gaps:

- Watcher/file edited events are not yet published as a real event bus.
- LSP touch/diagnostics are not yet collected.
- BOM preservation and formatter hooks are not yet equivalent to upstream OpenCode.
- Tool cards and parts are visible/durable enough for proof, but not full OpenCode lifecycle parity.
- ReasoningPart is a safe public summary only, not provider/private reasoning capture.
- CompactionPart is not full OpenCode compaction process parity yet.

## Features implemented

### Chat & Conversation

- Multi-turn conversation threads with snapshot persistence.
- Conversation create / list / get / delete.
- Chat completion via REST API and WebUI streaming route.
- Cancel / pause / resume API shape exists.
- Snapshot and compaction request routes exist.
- Edit approval route exists.
- Message history with metadata.

### Agent & Tool Execution

- Tool calling: file read/write/edit/delete/list/glob/search, web fetch/search, shell, terminal, task, batch parallel, repo info, propose patch, apply_patch, switch mode, browser proof, vision review, graph build/query.
- Parallel tool execution via `futures::stream::buffer_unordered`.
- `apply_patch` parses OpenCode-style patch text, validates patch paths, derives update contents, records pending edit approval before mutation, mutates files for add/update/delete/move only after approval, records diff/edit metadata, and returns `A/D/M` summary lines.

### Model & Provider

- Multi-provider support: NVIDIA NIM first, then Groq and OpenRouter when env vars are available.
- FreeLLMAPI-style deterministic provider/model fallback work is in progress.
- NIM provider sends tool definitions and parses tool calls.
- Provider key management via env vars.

### WebUI and Proof

- Bundled root chat UI.
- Live SSE events for run phases, text deltas, tool calls, tool results/errors, file-change events, and run finish.
- Browser proof route and NIM vision review route.
- Live WebUI Feature Sprint proves normal-prompt edit approval, approved file creation, repository inspection, snapshot, compaction, and screenshot artifacts.
- Latest proof script now requires visible OpenCode edit approval plus `TextPart`, `ReasoningPart`, `SnapshotPart`, `CompactionPart`, `FilePart`, `ToolPart`, and `PatchPart` markers.

### CI/CD

- GitHub Actions: CI, Build Proof, and Live WebUI Feature Sprint.
- Hard 500-line gate: `scripts/ci/check-file-lines.sh`.
- Keep proof artifacts in Actions; keep only compact handoff/status docs in git.

## Current gaps, highest priority

| Area | Feature | Priority |
|------|---------|----------|
| Engine | Watcher/file edited events and LSP diagnostics for patch changes | P0 |
| Engine/WebUI | Full durable OpenCode tool-part lifecycle parity | P0 |
| Engine | BOM preservation and formatter hooks | P1 |
| Engine | Full OpenCode context compaction process parity | P1 |
| Router | Visible routing/fallback receipts and cooldown policy | P1 |
| Benchmark | Artifact-backed adapter contract | P2 |

## What not to do

- Do not claim full OpenCode parity for `apply_patch` yet.
- Do not claim full OpenCode compaction parity yet.
- Do not add invented workflows when OpenCode has a source-defined behavior.
- Do not remove or weaken the 500-line hard gate.
- Do not expose private chain-of-thought through `ReasoningPart`; only public progress summaries are allowed.
- Do not accept JSON-only screenshots as UX proof.
