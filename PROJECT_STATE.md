# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current selected head before this docs update: `8abb0846467c7b3f98f129425c243d68645970a4`
- Latest same-head proof before this UI slice: CI `28410453891` success; Build Proof `28410453898` success; App Build Proof `28410453928` success; App Multistep Build Proof `28410453910` success; Fast WebUI Proof `28410453941` success; Live WebUI Feature Sprint `28410453924` success on head `1651dce2b6873c934c682b494ad7cdded044bb58`.
- Latest implementation slice: central browser session-turn rendering in `crates/webui/src/chat_ui.html`, loaded by `crates/webui/src/chat_ui.rs`, plus stricter Fast WebUI proof gates for session-turn UI.
- Do not claim the latest head containing this slice is same-head proven until CI / Build Proof / Fast WebUI Proof / Live WebUI Feature Sprint complete on that exact head and browser artifacts are inspected.

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626`.
- PR #3: open, non-draft, mergeable in PR metadata.
- No newer open PR superseded PR #3.

## Latest workflow state inspected before this slice

- Head `1651dce2b6873c934c682b494ad7cdded044bb58` had same-head green: CI, Build Proof, App Build Proof, App Multistep Build Proof, Fast WebUI Proof, and Live WebUI Feature Sprint.
- Live WebUI Feature Sprint proof on `1651dce2b6873c934c682b494ad7cdded044bb58` used provider `nvidia_nim`, model `deepseek-ai/deepseek-v4-flash`, and produced browser screenshots/artifacts for full benchmark and natural feature work.
- Current head after the central session-turn UI slice must be validated separately.

## Latest implementation changes

- `crates/webui/src/chat_ui.rs` now loads the browser UI from `include_str!("chat_ui.html")`, keeping the UI reviewable instead of hiding it inside a giant Rust raw string.
- `crates/webui/src/chat_ui.html` now renders central session turns: user prompt, assistant parts, live tool parts, completed tool cards, copy/retry actions, thinking/working state, changed files / file receipts, and a persistent right-side Session timeline.
- Tool cards keep human labels and status badges visible while collapsed technical details remain outside the final proof view.
- `scripts/smoke/fast-webui-proof.sh` now gates the browser proof on the central session-turn UI, copy/retry actions, collapsible tool cards, deferred technical content hooks, and absence of visible `OpenCode-style` branding.
- `scripts/smoke/check-webui-proof-part-contract.py` now validates both `crates/webui/src/chat_ui.rs` and `crates/webui/src/chat_ui.html`, so the deterministic CI harness follows the current UI source of truth.
- This is a source-backed browser WebUI parity slice, not a claim that complete OpenCode parity is done.

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

- OpenCode source backing: `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`, `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`, `anomalyco/opencode:packages/session-ui/src/components/basic-tool.tsx`, and `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`.
- Forge source paths under guard: `crates/webui/src/chat_ui.rs` and `crates/webui/src/chat_ui.html`.
- Required behavior tokens: `proof-final`, `proof-digest-visible`, `final-answer-visible`, `provider-model-visible`, `human-tool-label`, `session-turn-central`, `assistant-parts`, `message-part`, `opencode-tool-result-card`, `opencode-live-toolpart`, `collapsible-tool-card`, `deferred-technical-content`, `copy-retry-actions`, `session-turn-diffs-group`, `todo-status-summary`, and `todo-counts`.
- Session turn proof trail: central session turn rendering groups each user prompt with assistant parts, readable tool cards, copy/retry actions, thinking state, changed files / file receipts, and collapsed technical details.
- Todo status proof trail: Forge WebUI summarizes matching status counts for `todo_write` tool results instead of exposing raw JSON as the primary result.

## Natural WebUI finished-stream transport evidence retained for CI/proof

- OpenCode source backing: `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`, `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts`, and `anomalyco/opencode:packages/opencode/src/session/processor.ts`.
- Forge source path under guard: `scripts/smoke/natural-feature-work.sh`.
- Required behavior: proof acceptance follows recorded run completion plus downstream browser/tool/evidence gates; curl timeout after `event: run-finish` is treated as transport noise, not model/tool failure.

## OpenCode source anchors retained in developer docs only

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx` — browser SessionTurn grouping, assistant parts, active/pending status, thinking state, SessionRetry, and changed files diff group.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx` — browser MessagePart and AssistantParts rendering, copy action, part mapping, and tool/file/reasoning/text part handling.
- `anomalyco/opencode:packages/session-ui/src/components/basic-tool.tsx` — BasicTool trigger/status behavior, collapsible details, deferred heavy content, and pending/running handling.
- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx` — animated/count summary treatment for grouped tool activity.
- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts` — max-step no-tools finalization, text-only summary, remaining work list, next-step recommendations, and evidence-bound command claims.
- `anomalyco/opencode:packages/opencode/src/tool/edit.ts` — conservative file edit replacement behavior.
- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle, provider-executed state, same-call ToolPart update semantics, complete/fail tool-call handling, and tool-result output.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — part base, ToolPart, ToolState, and FilePart schema shape.
- `anomalyco/opencode:packages/opencode/src/format/index.ts` and `packages/opencode/src/format/formatter.ts` — formatter catalog and activation behavior.
- `anomalyco/opencode:packages/opencode/src/tool/glob.ts` and `packages/opencode/src/tool/grep.ts` — search/glob path resolution, result count metadata, bounded output, human-readable output, and `No files found` behavior.
- `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts` — concise final turn summary carrying model/status metadata.
- `anomalyco/opencode:packages/opencode/src/session/prompt.ts` — prompt/session path for file references and delegated prompt operations.

## Current behavior retained
