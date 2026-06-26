# Feature Audit — Forge Unified

Audit date: 2026-06-26
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3 into `master`

## Latest proven code baseline before this docs sync

Latest code HEAD: `e160fa4bf9326c26d5731e9fb474574a4d068b2f`

Latest fully green baselines:

| HEAD | CI | Build Proof | Live WebUI Feature Sprint | Notes |
|---|---|---|---|---|
| `0da7281dc0f85bb16906103343d2e9d24827dafa` | success | success | success | `apply_patch` mutation slice |
| `65c1cb5f5c534149d4e08000e8553a498767ed00` | success | success | success | Cleaner WebUI tool cards |
| `7f46ea1c0e7498a353fa18a3781b062580105236` | success | success | success | Natural file creation + repo inspection proof |
| `e160fa4bf9326c26d5731e9fb474574a4d068b2f` | success | success | success | Compact repo inspection output with raw metadata preserved |

The latest docs-updated HEAD after this sync still needs its own Actions check before merge/green claims.

## Source-first OpenCode rule

OpenCode-equivalent work must be grounded in exact upstream source files. Do not claim parity from broad similarity.

Canonical parity tracker: `OPENCODE-PARITY.md`.

## Implemented / real enough to claim

- Root page serves a bundled single-page MVP chat UI.
- UI can create conversations, select conversations, send messages, display messages/tool events, and open graph view.
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
- `apply_patch` applies add/update/delete/move file mutations inside the workspace.
- `apply_patch` records diff metadata, edit-permission metadata, parsed hunk metadata, and OpenCode-style `A/D/M` summary lines.
- `apply_patch` file changes now appear as WebUI file cards in natural browser proof.
- Normal prompt `Please create a short proof note for this WebUI sprint.` creates a real proof note through `apply_patch`, persists the tool result, and returns a human summary.
- Normal prompt `Please inspect this repository and summarize what you find.` runs real `repo_info` and `file_list` tools and returns a human summary.
- Repo-inspection tool cards now show compact visible output (`Repository status`, `Top-level repository entries`) while preserving raw JSON in `metadata.raw_output`.
- CI and Build Proof enforce a hard 500-line source file limit through `scripts/ci/check-file-lines.sh`.
- The graphify CLI oversized source was split into `crates/unifiedgraph/src/cli.rs` and a compact `crates/unifiedgraph/src/main.rs`; the gate is kept real, not weakened.

## Partial / do not overclaim

- `apply_patch` is still not full upstream parity. Current Forge implementation mutates files for add/update/delete/move, but it does not yet implement real interactive edit approval, a watcher/file-edited event bus, LSP diagnostics, full BOM preservation, or formatter hooks.
- File-change cards are implemented, but watcher events are not equivalent to OpenCode's `FileSystem.Event.Edited` and `Watcher.Event.Updated` yet.
- Orchestrator prompting is not yet fully copied from OpenCode. The natural proof style is closer to OpenCode, but the engine system prompt still needs a source-gated rewrite.
- Provider routing and fallback are basic; receipts and policy are immature.
- Conversation persistence is mostly in-memory plus snapshots.
- Benchmark adapter is shallow and not yet the full artifact-backed contract.
- WebUI cards are cleaner, but the durable OpenCode `ToolPart` state model is not fully implemented.

## Highest-priority next work

1. Check latest Actions for the docs-updated HEAD and fix any real failures.
2. Implement real edit permission gating for `apply_patch` from OpenCode source.
3. Implement durable OpenCode-style `ToolPart` state: pending, running, completed, error.
4. Implement watcher/file edited events and LSP diagnostics for patch changes.
5. Keep all checked source files under 500 lines by splitting before monoliths form.
6. Rewrite Forge's system prompt from studied OpenCode prompt behavior, not invented wording.
7. Add durable session/message/part persistence.
8. Add context compaction parity.

## Claim rule

Before calling a slice done:

- Update `CONTINUE_HERE.md`, `PROJECT_STATE.md`, `FEATURE-AUDIT.md`, and `OPENCODE-PARITY.md` if status changed.
- Validate with CI, Build Proof, and Live WebUI Feature Sprint.
- Keep proof artifacts in Actions; keep only compact summaries in git.
- Do not merge files over the 500-line hard gate.
- Do not call a screenshot proof acceptable unless the visible PNG shows normal user prompts, real tool output, and a human-readable result.
