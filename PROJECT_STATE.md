# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest selected head before this slice: `9a8f0157e75f2017200764abbcf10ac4ba4d1bf9`
- Latest same-head proof before this slice: CI `28409705430` success; Build Proof `28409705389` success; App Build Proof `28409705448` success; App Multistep Build Proof `28409705418` success; Fast WebUI Proof `28409705413` success; Live WebUI Feature Sprint `28409705410` failed only in the natural feature-build proof transport gate after full benchmark proof passed.
- Latest proof slice: natural WebUI finished-stream transport acceptance in `scripts/smoke/natural-feature-work.sh`.
- Latest proof doc: `docs/generated/proof/natural-webui-finished-stream-transport-20260630T0000Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI / Build Proof / Fast WebUI Proof / Live WebUI Feature Sprint complete on that exact head.

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626`.
- PR #3: open, non-draft, mergeable in PR metadata.
- Current selected head before this slice: `9a8f0157e75f2017200764abbcf10ac4ba4d1bf9`.
- No newer open PR superseded PR #3.

## Latest workflow state inspected

- CI `28409705430`: success.
- Build Proof `28409705389`: success.
- App Build Proof `28409705448`: success.
- App Multistep Build Proof `28409705418`: success.
- Fast WebUI Proof `28409705413`: success.
- Live WebUI Feature Sprint `28409705410`: failure in job `84179868033`.
- Live WebUI failure details: full benchmark proof passed with provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, 24 tool-call events, and 24 tool-result events; natural feature-build proof had provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, 17 tool-call events, 17 tool-result events, browser proof success, screenshot success, and failed only `stream_exit_code_zero` because curl returned `28` after receiving stream data.

## Latest implementation changes

- `scripts/smoke/natural-feature-work.sh` now records `stream_exit_code` and `stream_transport_ok`.
- The natural WebUI proof now accepts stream exit code `0`, or curl timeout codes `28` / `124` only when the SSE stream includes `event: run-finish`.
- Provider/model, tool-call/tool-result, edit marker, browser proof, screenshot, validation command, no-shortcut, and final-answer gates remain required.
- Removed the post-check hard fail that rejected a passed proof purely because curl timed out after completion.
- This is a source-backed proof harness parity slice, not a claim that complete OpenCode parity is done.

## User rejection that drove the WebUI proof work

- Previous tool cards exposed raw tool names as primary UI and were unintuitive.
- The full benchmark final screenshot mostly showed the benchmark prompt, not proof or the final answer.
- Process screenshots looked messy and too diagnostic-heavy for normal review.
- Fast WebUI Proof previously proved the DOM could still contain raw implementation text/JSON in final proof mode.

## Search/glob contract evidence retained for CI

- OpenCode source backing: `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `anomalyco/opencode:packages/opencode/src/tool/grep.ts`.
- Required behavior tokens: path resolution, result count metadata, `No files found`, bounded output, human-readable output, and grep/glob proof trail retention.
- Forge source path under guard: `crates/engine/src/tool/file_ops.rs`.

## Formatter activation evidence retained for CI

- The formatter proof trail must explicitly mention configuration/dependency-aware formatter activation.
- The formatter proof trail must preserve evidence for formatter service, extension matching, command probing/caching, contained formatter execution, status shape, and configuration/dependency-aware formatter activation.
- The formatter proof trail must preserve evidence for built-in formatter catalog, representative extensions, command semantics, and config/dependency-aware formatter enablement.

## WebUI proof part contract evidence retained for CI

- OpenCode source backing: `anomalyco/opencode:packages/web/src/components/share/part.tsx` and `anomalyco/opencode:packages/web/src/components/share/part.module.css`.
- Forge source path under guard: `crates/webui/src/chat_ui.rs`.
- Required behavior tokens: `proof-final`, `proof-digest-visible`, `final-answer-visible`, `provider-model-visible`, `human-tool-label`, `opencode-tool-result-card`, `opencode-live-toolpart`, `todo-status-summary`, `todo-counts`, human label rendering, and collapsed technical details outside final proof mode.
- Todo status proof trail: OpenCode `TodoWriteTool` sorts and renders `pending`, `in_progress`, and `completed` todos; Forge WebUI now summarizes matching status counts for `todo_write` tool results.

## Natural WebUI finished-stream transport evidence retained for CI/proof

- OpenCode source backing: `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`, `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`, and `anomalyco/opencode:packages/opencode/src/session/processor.ts`.
- Forge source path under guard: `scripts/smoke/natural-feature-work.sh`.
- Required behavior: proof acceptance follows recorded run completion plus downstream browser/tool/evidence gates; curl timeout after `event: run-finish` is treated as transport noise, not model/tool failure.

## OpenCode source anchors retained in developer docs only

- `anomalyco/opencode:packages/web/src/components/share/part.tsx` — user-facing componentized tool rendering for completed tool parts, including `TodoWriteTool` status handling.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css` — shared visual treatment for parts/tool UI, including `[data-component="todos"]` and `[data-status="..."]` styling.
- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts` — max-step no-tools finalization, text-only summary, remaining work list, next-step recommendations, and evidence-bound command claims.
- `anomalyco/opencode:packages/opencode/src/tool/edit.ts` — conservative file edit replacement behavior.
- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, provider-executed state, same-call ToolPart update semantics, complete/fail tool-call handling, and tool-result output.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — part base, ToolPart, ToolState, and FilePart schema shape.
- `anomalyco/opencode:packages/opencode/src/format/index.ts` and `packages/opencode/src/format/formatter.ts` — formatter catalog and activation behavior.
- `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `packages/opencode/src/tool/grep.ts` — search/glob path resolution, result count metadata, bounded output, human-readable output, and `No files found` behavior.
- `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts` — concise final turn summary carrying model/status metadata.
- `anomalyco/opencode:packages/opencode/src/session/prompt.ts` — prompt/session path for file references and delegated prompt operations.

## Current behavior retained
