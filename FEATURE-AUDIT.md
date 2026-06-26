# Feature Audit — Forge Unified

Audit date: 2026-06-26
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`

## Current verified code baseline before this docs sync

Latest verified code HEAD: `b1b31301a1992dc3ea6accdecee1ac1b71dac256`

At that HEAD:

- CI: success
- Build Proof: success
- Live WebUI Feature Sprint: success

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
- The live screenshot proof now requires a completed human-readable WebUI prompt response.
- `apply_patch` has a first-stage OpenCode-compatible surface using `patchText`.

## Partial / do not overclaim

- `apply_patch` is not full parity yet. Current Forge implementation accepts patch text and records metadata; it does not yet implement the full OpenCode hunk parse / permission metadata / file update / watcher event / diagnostics flow.
- Orchestrator prompting is not yet fully copied from OpenCode. The proof prompt references OpenCode default response behavior, but the engine system prompt still needs a source-gated rewrite.
- Provider routing and fallback are basic; receipts and policy are immature.
- Conversation persistence is mostly in-memory.
- Benchmark adapter is shallow and not yet the full artifact-backed contract.

## Highest-priority next work

1. Finish `apply_patch` parity from `packages/opencode/src/tool/apply_patch.ts` and related patch parser files.
2. Rewrite Forge's system prompt from studied OpenCode prompt behavior, not invented wording.
3. Copy OpenCode tool part states from `packages/opencode/src/session/processor.ts` into WebUI cards.
4. Add durable session/message/part persistence.
5. Add context compaction parity.

## Claim rule

Before calling a slice done:

- Update `CONTINUE_HERE.md`, `PROJECT_STATE.md`, `FEATURE-AUDIT.md`, and `OPENCODE-PARITY.md` if status changed.
- Validate with CI, Build Proof, and Live WebUI Feature Sprint.
- Keep proof artifacts in Actions; keep only compact summaries in git.
