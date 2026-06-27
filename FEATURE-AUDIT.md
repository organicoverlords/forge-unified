# Feature Audit — Forge Unified

Audit date: 2026-06-27
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3 into `master`

## Latest proven code baseline before this docs sync

Latest fully green code HEAD: `a83ddac8542264cf69bd18988cd6e7dc6f518d95`
Latest edit-approval proof artifact: `/mnt/data/live-webui-feature-sprint-proof-a83ddac.zip`

Latest fully green baselines:

| HEAD | CI | Build Proof | Live WebUI Feature Sprint | Notes |
|---|---|---|---|---|
| `0da7281dc0f85bb16906103343d2e9d24827dafa` | success | success | success | `apply_patch` mutation slice |
| `65c1cb5f5c534149d4e08000e8553a498767ed00` | success | success | success | Cleaner WebUI tool cards |
| `7f46ea1c0e7498a353fa18a3781b062580105236` | success | success | success | Natural file creation + repo inspection proof |
| `e160fa4bf9326c26d5731e9fb474574a4d068b2f` | success | success | success | Compact repo inspection output with raw metadata preserved |
| `b7b0e7eb88570900ad8e3252d8190004342678fd` | success | success | success | OpenCode `SnapshotPart` persistence |
| `c3f15e4a5ac9c84fb07a6a49ec87118c97c4c3e7` | success | success | success | OpenCode `FilePart` persistence |
| `a0efdb6372cd92ac6b579bd152f009bb3debefbd` | success | success | success | OpenCode `ReasoningPart` persistence |
| `84e459ef3bd4d4f88636239c76136617a98b68e3` | success | success | success | OpenCode `CompactionPart` persistence |
| `a83ddac8542264cf69bd18988cd6e7dc6f518d95` | success | success | success | Real edit approval-before-write gate for `apply_patch` |

The latest docs-updated HEAD after this sync still needs its own Actions check before merge/green claims.

## Source-first OpenCode rule

OpenCode-equivalent work must be grounded in exact upstream source files. Do not claim parity from broad similarity.

Canonical parity tracker: `OPENCODE-PARITY.md`.

## Implemented / real enough to claim

- Root page serves a bundled single-page MVP chat UI.
- UI can create conversations, select conversations, send messages, display messages/tool events, save snapshots, request compaction, approve edits, and open graph view.
- Provider configs include NIM and OpenAI-compatible providers.
- Runtime state selects the first enabled provider/model from config.
- Tool schema generation and tool-call conversion are wired.
- WebUI SSE emits OpenCode-inspired lifecycle events for tool input, tool call, tool result/error, file change, text, and run finish.
- Browser proof captures `browser-proof.json` and `webui.png`.
- The live screenshot proof requires completed human-readable WebUI prompt responses, not marker-only or JSON-only output.
- `apply_patch` has an OpenCode-compatible `patchText` surface.
- `apply_patch` rejects empty/malformed Begin/End patch text.
- `apply_patch` parses add/update/delete/move hunks.
- `apply_patch` validates patch/move paths before mutation.
- `apply_patch` derives update contents from chunks using exact/rstrip/trim/Unicode matching.
- `apply_patch` now returns a pending edit approval and does not write files before approval.
- `apply_patch` approval API re-runs the same patch with `approved=true` and only then applies add/update/delete/move file mutations inside the workspace.
- `apply_patch` records diff metadata, edit-permission metadata, pending/approved approval state, parsed hunk metadata, and OpenCode-style `A/D/M` summary lines.
- `apply_patch` file changes appear as WebUI file cards only after approval.
- Normal prompt `Please create a short proof note for this WebUI sprint.` creates a pending approval, then the proof approves it and writes the file.
- Normal prompt `Please inspect this repository and summarize what you find.` runs real `repo_info` and `file_list` tools and returns a human summary.
- Repo-inspection tool cards show compact visible output (`Repository status`, `Top-level repository entries`) while preserving raw JSON in `metadata.raw_output`.
- OpenCode-style `TextPart`, `ReasoningPart`, `SnapshotPart`, `CompactionPart`, `FilePart`, `ToolPart`, and `PatchPart` persistence/rendering are proven green through `a83ddac`.
- CI and Build Proof enforce a hard 500-line source file limit through `scripts/ci/check-file-lines.sh`.

## Partial / do not overclaim

- `apply_patch` is still not full upstream parity. Current Forge implementation now gates writes behind approval, but it does not yet implement a real watcher/file-edited event bus, LSP diagnostics, full BOM preservation, or formatter hooks.
- File-change cards are implemented, but watcher events are not equivalent to OpenCode's `FileSystem.Event.Edited` and `Watcher.Event.Updated` yet.
- Orchestrator prompting is not yet fully copied from OpenCode. The natural proof style is closer to OpenCode, but the engine system prompt still needs a source-gated rewrite.
- Provider routing and fallback are basic; receipts and policy are immature.
- Conversation persistence is mostly in-memory plus snapshots.
- Benchmark adapter is shallow and not yet the full artifact-backed contract.
- WebUI cards are cleaner, and session parts are visible/durable enough for proof, but this is not full OpenCode session storage parity.
- `ReasoningPart` is only a safe public progress summary. It must not expose hidden/private chain-of-thought.
- `CompactionPart` is a durable request marker and optional local pruning path, not full OpenCode LLM compaction summary/replay/autocontinue parity.

## Highest-priority next work

1. Check latest Actions for the docs-updated edit-approval HEAD and fix any real failures.
2. Implement watcher/file edited events and LSP diagnostics for approved patch changes.
3. Implement full durable OpenCode-style `ToolPart` lifecycle parity: pending, running, completed, error.
4. Keep all checked source files under 500 lines by splitting before monoliths form.
5. Rewrite Forge's system prompt from studied OpenCode prompt behavior, not invented wording.
6. Add durable session/message/part persistence.
7. Complete context compaction parity beyond the request marker.
8. Implement `AgentPart` or `RetryPart` only when backed by a real Forge behavior/proof path.

## Claim rule

Before calling a slice done:

- Update `CONTINUE_HERE.md`, `PROJECT_STATE.md`, `FEATURE-AUDIT.md`, and `OPENCODE-PARITY.md` if status changed.
- Validate with CI, Build Proof, and Live WebUI Feature Sprint.
- Keep proof artifacts in Actions; keep only compact summaries in git.
- Do not merge files over the 500-line hard gate.
- Do not call a screenshot proof acceptable unless the visible PNG shows normal user prompts, real tool output, and a human-readable result.
