# OpenCode doom-loop permission envelope proof

Date: 2026-06-28
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`

## Source of truth

Target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

Selected work branch: PR #3 / `mvp/nim-freellmapi-router-20260626`.

## Failed workflow inspected

Current pre-slice HEAD inspected: `f7a4cdc6f24a9327eeca4dc07adda903512dc041`.

Workflow failures for that head:

- Live WebUI Feature Sprint `28307685427`: failed.
- Build Proof `28307685436`: failed.
- CI `28307685432`: failed.

Root cause visible in Live WebUI Feature Sprint job `83866754064`:

- The Rust app compiled successfully.
- `scripts/smoke/live-webui-feature-sprint.sh` failed at shell parse time: `line 91: syntax error near unexpected token '('`.
- The benchmark checker then failed because `full-benchmark-conversation.json` and `full-benchmark-stream.sse` were missing.

Repair included in this run:

- Replaced the fragile model-stream marker loop at that point with explicit `need_marker` checks.
- Kept all NIM-only/local-shortcut rejection checks intact.
- Fixed the jq non-agent-test path predicate parentheses so the shell script remains parseable.

## OpenCode source backing

Primary upstream source read:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`

Exact copied behavior anchors:

- `DOOM_LOOP_THRESHOLD = 3`.
- Recent ToolPart comparison by tool name and serialized input.
- `permission.ask({ permission: "doom_loop", patterns, sessionID, metadata, always, ruleset })` after the repeated tool-call window is detected.

Relevant source regions inspected:

- `packages/opencode/src/session/processor.ts` lines containing `const DOOM_LOOP_THRESHOLD = 3`.
- `packages/opencode/src/session/processor.ts` tool-call handling block where recent parts are sliced by `DOOM_LOOP_THRESHOLD` and compared.
- `packages/opencode/src/session/processor.ts` `permission.ask({ permission: "doom_loop", ... })` envelope.

## Forge slice implemented

File changed: `crates/engine/src/orchestrator.rs`.

Forge already interrupted three repeated identical tool batches. This slice adds a visible, structured OpenCode-style doom-loop permission envelope before interruption:

- Records `doom_loop_permission_recorded` in run metadata.
- Emits a `ToolResult` with `success: false` and `error: doom_loop_permission_required`.
- Includes metadata keys: `permission`, `patterns`, `always`, `ruleset`, `input`, `recent_tool_signatures`, `opencode_doom_loop_permission`, and `opencode_source`.
- Keeps the safety behavior conservative: Forge blocks the loop instead of silently continuing.

## Validation status

Not same-head green yet.

Expected next validation:

- GitHub Actions should run for the new branch HEAD.
- Same-head CI / Build Proof / Live WebUI Feature Sprint must pass before claiming live parity.
- WebUI screenshot proof must come from the Live WebUI Feature Sprint artifact and must use NVIDIA NIM only.

## Do not overclaim

This is not full OpenCode permission UX parity yet. It is the structured permission-envelope and proof-visible interruption slice copied from OpenCode processor behavior. Full interactive allow/deny recovery is still pending.
