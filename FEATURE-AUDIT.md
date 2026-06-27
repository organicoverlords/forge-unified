# Feature Audit — Forge Unified

Audit date: 2026-06-27
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3 into `master`

## Latest proven code baseline before this docs sync

Latest fully green code HEAD: `6a34928048b86e6d7b91468789eeef4489744ae8`
Latest post-edit event proof artifact: `/mnt/data/live-webui-feature-sprint-proof-6a34928.zip`

Latest fully green baselines include `805406542b55f803924401459f881f5df43680b7` for the modern dark Codex/OpenCode WebUI theme and `6a34928048b86e6d7b91468789eeef4489744ae8` for OpenCode post-edit event and LSP touch receipts. The latest docs-updated HEAD after this sync still needs its own Actions check before merge/green claims.

## Source-first OpenCode rule

OpenCode-equivalent work must be grounded in exact upstream source files. Do not claim parity from broad similarity.

Canonical parity tracker: `OPENCODE-PARITY.md`.

## Implemented / real enough to claim

- Root page serves a bundled single-page MVP chat UI with a modern dark Codex/OpenCode-style theme.
- UI can create conversations, select conversations, send messages, display messages/tool events, save snapshots, request compaction, approve edits, and open graph view.
- WebUI SSE emits OpenCode-inspired lifecycle events for tool input, tool call, tool result/error, file change, text, and run finish.
- Browser proof captures `browser-proof.json` and `webui.png` and requires completed human-readable WebUI prompt responses.
- `apply_patch` has an OpenCode-compatible `patchText` surface.
- `apply_patch` rejects empty/malformed Begin/End patch text.
- `apply_patch` parses add/update/delete/move hunks and validates paths before mutation.
- `apply_patch` returns a pending edit approval and does not write files before approval.
- Approval re-runs the same patch with `approved=true` and only then applies add/update/delete/move mutations inside the workspace.
- `apply_patch` records diff metadata, edit-permission metadata, pending/approved approval state, parsed hunk metadata, and OpenCode-style `A/D/M` summary lines.
- `apply_patch` records OpenCode-shaped `FileSystem.Event.Edited`, `Watcher.Event.Updated`, and `lsp.touchFile(..., "document")` receipts after approved mutation.
- `apply_patch` file changes appear as WebUI file cards only after approval.
- Normal proof prompts cover edit approval, approved file creation, repository inspection, snapshot, compaction, and screenshot artifacts.
- Repo-inspection tool cards show compact visible output while preserving raw JSON in `metadata.raw_output`.
- OpenCode-style `TextPart`, `ReasoningPart`, `SnapshotPart`, `CompactionPart`, `FilePart`, `ToolPart`, and `PatchPart` persistence/rendering are proven green through `6a34928`.
- CI and Build Proof enforce a hard 500-line source file limit.

## Partial / do not overclaim

- `apply_patch` is still not full upstream parity. Current Forge implementation gates writes behind approval and records post-edit event receipts, but it does not yet implement a real watcher/file-edited event bus, live LSP diagnostics, full BOM preservation, or formatter hooks.
- File-change cards and OpenCode event receipts are implemented, but a live event bus equivalent to OpenCode's `FileSystem.Event.Edited` and `Watcher.Event.Updated` is not wired yet.
- Orchestrator prompting is not yet fully copied from OpenCode.
- Provider routing and fallback are basic; receipts and policy are immature.
- Conversation persistence is mostly in-memory plus snapshots.
- Benchmark adapter is shallow and not yet the full artifact-backed contract.
- WebUI cards and session parts are visible/durable enough for proof, but this is not full OpenCode session storage parity.
- `ReasoningPart` is only a safe public progress summary.
- `CompactionPart` is a durable request marker and optional local pruning path, not full OpenCode LLM compaction summary/replay/autocontinue parity.

## Highest-priority next work

1. Check latest Actions for the docs-updated post-edit-events HEAD and fix any real failures.
2. Implement full durable OpenCode-style `ToolPart` lifecycle parity: pending, running, completed, error.
3. Implement a real watcher/file edited event bus beyond receipts.
4. Implement live LSP diagnostics beyond touched-file receipts.
5. Keep all checked source files under 500 lines by splitting before monoliths form.
6. Rewrite Forge's system prompt from studied OpenCode prompt behavior, not invented wording.
7. Add durable session/message/part persistence.
8. Complete context compaction parity beyond the request marker.
9. Implement `AgentPart` or `RetryPart` only when backed by a real Forge behavior/proof path.

## Claim rule

Before calling a slice done:

- Update `CONTINUE_HERE.md`, `PROJECT_STATE.md`, `FEATURE-AUDIT.md`, and `OPENCODE-PARITY.md` if status changed.
- Validate with CI, Build Proof, and Live WebUI Feature Sprint.
- Keep proof artifacts in Actions; keep only compact summaries in git.
- Do not merge files over the 500-line hard gate.
- Do not call a screenshot proof acceptable unless the visible PNG shows normal user prompts, real tool output, and a human-readable result.
