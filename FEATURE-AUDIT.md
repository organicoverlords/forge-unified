# Feature Audit — Forge Unified

Audit date: 2026-06-26
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3 into `master`

## Latest code baseline before this docs sync

Latest code-fix HEAD: `74eba32f57e9bfb682effaa202bdeac07f970c35`

Pre-fix failure found at `6d2faa89a9b0e2637eb9f8a58c51459d5da55e77`:

- CI: failed only because `File Size Gate` failed.
- Build Proof: failed only because `File line gate` failed before cargo check/test/smoke.
- Live WebUI Feature Sprint: success.

Post-fix signal at `74eba32f`:

- CI File Size Gate: passed.
- CI Test: passed.
- Build Proof File line gate: passed.
- Build Proof Cargo check: passed.
- Full CI / Build Proof / Live WebUI completion must still be checked on latest HEAD before calling the branch green.

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
- CI and Build Proof enforce a hard 500-line source file limit through `scripts/ci/check-file-lines.sh`.
- The graphify CLI oversized source was split into `crates/unifiedgraph/src/cli.rs` and a compact `crates/unifiedgraph/src/main.rs`; the gate is kept real, not weakened.

## Partial / do not overclaim

- `apply_patch` is not full parity yet. Current Forge implementation parses OpenCode patch markers/hunks for review, but it does not yet implement the full permission metadata / file update / watcher event / diagnostics flow.
- Orchestrator prompting is not yet fully copied from OpenCode. The proof prompt references OpenCode default response behavior, but the engine system prompt still needs a source-gated rewrite.
- Provider routing and fallback are basic; receipts and policy are immature.
- Conversation persistence is mostly in-memory plus snapshots.
- Benchmark adapter is shallow and not yet the full artifact-backed contract.

## Highest-priority next work

1. Check latest Actions for the docs-updated HEAD and fix any real failures.
2. Finish `apply_patch` parity from `packages/opencode/src/tool/apply_patch.ts` and related patch parser files.
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
