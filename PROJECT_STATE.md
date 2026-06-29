# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Latest selected head before this slice: `6f43a985165a5b53db06e8c76ed2513c135d94ed`
- Latest accepted same-head proof baseline before this slice: `6f43a985165a5b53db06e8c76ed2513c135d94ed`.
- Same-head accepted baseline: CI `28406505084` success; Build Proof `28406505066` success; App Build Proof `28406505063` success; App Multistep Build Proof `28406505077` success; Fast WebUI Proof `28406505108` success; Live WebUI Feature Sprint `28406505078` success; Live WebUI artifact `7966282221`.
- Latest proof slice: WebUI TodoWrite-style plan status rendering in `crates/webui/src/chat_ui.rs` plus the source-backed gate in `scripts/smoke/check-webui-proof-part-contract.py`.
- Latest proof doc: `docs/generated/proof/webui-todo-plan-status-part-20260630T0146Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI / Build Proof / Live WebUI Feature Sprint / Fast WebUI Proof complete on that exact head.

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626`.
- PR #3: open, non-draft, mergeable in PR metadata.
- Current selected head before this slice: `6f43a985165a5b53db06e8c76ed2513c135d94ed`.
- This branch superseded earlier reported heads and had a full green same-head proof set before this slice.

## Latest workflow state inspected

- CI `28406505084`: success.
- Build Proof `28406505066`: success.
- App Build Proof `28406505063`: success.
- App Multistep Build Proof `28406505077`: success.
- Fast WebUI Proof `28406505108`: success.
- Live WebUI Feature Sprint `28406505078`: success.
- Live WebUI artifact: `7966282221`, digest `sha256:e20fd3f3cbc2661c77901a5eec3031baf21db8018cefbe2e8cfc43a324a4d92d`.

## Latest implementation changes

- `crates/webui/src/chat_ui.rs` now treats `todo_write` as a plan/status part instead of only a generic completed JSON-backed tool.
- Added `todoItems`, `todoSummary`, and `addTodoStatus` helpers for visible pending / in-progress / completed plan counts.
- Added `todo-status-summary` and `todo-counts` proof markers/styles so browser proof can verify the plan card is human-readable.
- `scripts/smoke/check-webui-proof-part-contract.py` now enforces this TodoWriteTool-style status rendering contract.
- This is a source-backed WebUI part parity slice, not a claim that complete OpenCode parity is done.

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

- WebUI uses the dark Codex/OpenCode-like theme.
- Live WebUI proof must use a real NVIDIA NIM route, not local shortcuts.
- Natural WebUI prompts must render live ToolPart lifecycle cards with provider metadata.
- File-change and event receipts are visible in chat.
- Normal file tools emit file/watch/LSP receipts, formatter metadata, BOM metadata, completed ToolPart attachments, schema-compatible part base fields, ToolPart-like result state envelopes, normalized tool attachment metadata, and TodoWrite-style plan status summaries.
- The Live WebUI Feature Sprint workflow also requires a dedicated natural feature-build prompt artifact under `natural-feature-work/`.
- Final proof screenshots must show a readable proof digest and final answer, not only the original benchmark prompt.
