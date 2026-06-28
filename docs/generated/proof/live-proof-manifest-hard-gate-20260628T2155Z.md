# Live WebUI proof manifest hard gate — 2026-06-28T21:55Z

## Selection

- Repo: `organicoverlords/forge-unified`.
- Branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open and non-draft.
- Baseline head before this slice: `0579b550bfb489a0a16289b98a6eafabcd14c0d6`.
- Baseline same-head workflows were green: CI `28336034206`, Build Proof `28336034212`, Live WebUI Feature Sprint `28336034202`.
- Baseline Live WebUI artifact: `7938654225`, `live-webui-feature-sprint-proof`, digest `sha256:c631cbb7570af2563475220513924f134ec1b96d4c36b2304bb85af344c5448b`.

## Source-backed parity slice

OpenCode source path used as reference:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`.

Reference semantics:

- `completeToolCall` only marks a ToolPart completed after it records output, metadata, title, timing, and attachments.
- `failToolCall` records an explicit error state and timing before settling the tool call.
- `tool-result` handling normalizes output/attachments before calling completion.

Forge change:

- Hardened `scripts/smoke/check-live-webui-proof-manifest.py` so proof acceptance is artifact-first rather than workflow-name-first.
- The checker now validates:
  - required proof files exist and are non-empty;
  - `full-benchmark-webui.png` is a real non-trivial PNG screenshot;
  - `full-benchmark-browser-proof.json` contains the natural benchmark, Phase 1/2, Founder/Technical report, and `.agent_test` file evidence markers;
  - both full benchmark and OpenCode-style workflow checkers passed;
  - stream has run-finish, tool-call, and tool-result evidence;
  - runtime stream has no local shortcut or upstream identity leakage;
  - conversation records `nvidia_nim`, a model, and enough tool results;
  - status file points at the expected screenshot and workflow checker filenames.

## Proof status

- This commit is a hardening slice after the accepted same-head proof for `0579b550...`.
- Do not claim the new head is same-head proven until CI / Build Proof / Live WebUI Feature Sprint complete for the new commit.
