# Feature Audit — Forge Unified

Audit date: 2026-06-26
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3 into `master`

## Latest code baseline before this docs sync

Latest code HEAD: `541e67fe40ef51dff5dc5b2507606dd68f7a0e2c`

Latest fully green baselines:

| HEAD | CI | Build Proof | Live WebUI Feature Sprint |
|---|---|---|---|
| `e31d678277c0527d36f14f8eac8fc65f07c3b265` | success | success | success |
| `541e67fe40ef51dff5dc5b2507606dd68f7a0e2c` | success | success | success |

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
- WebUI SSE emits OpenCode-inspired lifecycle events for tool input, tool call, tool result/error, text, and run finish.
- Browser proof captures `browser-proof.json` and `webui.png`.
- The live screenshot proof requires a completed human-readable WebUI prompt response.
- `apply_patch` has an OpenCode-compatible `patchText` surface and parser-level hunk metadata for review.
- `apply_patch` now validates patch/move paths before metadata is accepted.
- `apply_patch` now records edit-permission metadata and OpenCode-style `A/D/M` summary lines.
- CI and Build Proof enforce a hard 500-line source file limit through `scripts/ci/check-file-lines.sh`.
- The graphify CLI oversized source was split into `crates/unifiedgraph/src/cli.rs` and a compact `crates/unifiedgraph/src/main.rs`; the gate is kept real, not weakened.

## Partial / do not overclaim

- `apply_patch` is not full parity yet. Current Forge implementation parses OpenCode patch markers/hunks for review, validates paths, records permission metadata, and returns summary lines, but it does not yet implement the full file update / watcher event / diagnostics flow.
- Orchestrator prompting is not yet fully copied from OpenCode. The proof prompt references OpenCode default response behavior, but the engine system prompt still needs a source-gated rewrite.
- Provider routing and fallback are basic; receipts and policy are immature.
- Conversation persistence is mostly in-memory plus snapshots.
- Benchmark adapter is shallow and not yet the full artifact-backed contract.

## Highest-priority next work

1. Check latest Actions for the docs-updated HEAD and fix any real failures.
2. Finish `apply_patch` file mutation parity from `packages/opencode/src/tool/apply_patch.ts` and `packages/opencode/src/patch/index.ts`.
3. Keep all checked source files under 500 lines by splitting before monoliths form.
4. Rewrite Forge's system prompt from studied OpenCode prompt behavior, not invented wording.
5. Copy OpenCode tool part states from `packages/opencode/src/session/processor.ts` into WebUI cards.
6. Add durable session/message/part persistence.
7. Add context compaction parity.

## Claim rule

Before calling a slice done:

- Update `CONTINUE_HERE.md`, `PROJECT_STATE.md`, `FEATURE-AUDIT.md`, and `OPENCODE-PARITY.md` if status changed.
- Validate with CI, Build Proof, and Live WebUI Feature Sprint.
- Keep proof artifacts in Actions; keep only compact summaries in git.
- Do not merge files over the 500-line hard gate.
