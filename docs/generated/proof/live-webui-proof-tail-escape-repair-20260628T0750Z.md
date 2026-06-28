# Live WebUI proof tail escape repair — 2026-06-28T07:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Source-of-truth target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Failed head inspected

- Inspected head: `ac43703fae3b70b6c7dd47a26034188426910a54`
- Live WebUI Feature Sprint run: `28314329079`
- Job: `83884630279`
- Result: failed before WebUI/NIM browser proof could run.
- Error: `scripts/smoke/live-webui-feature-sprint.sh: line 270: unexpected EOF while looking for matching '"'`.

## Diagnosis

The final proof-status block had literal newlines embedded inside Python and shell string literals:

- `fh.write(f"{key}={value}` was split before the intended `\n` escape.
- final `printf` format strings were split before the intended `\n` escape.

That made the script syntactically invalid, so no current-head browser screenshot or checker artifact was produced.

## Repair landed

- File: `scripts/smoke/live-webui-feature-sprint.sh`
- Commit: `55cc5238f7801baae08422acb20fc3542b4b7686`
- Replaced literal line breaks in final Python writer and `printf` format strings with escaped `\n` sequences.
- Added an early `bash -n "$0"` self-check so the workflow fails immediately on future shell syntax regressions instead of compiling first.
- Preserved the NVIDIA NIM-only gates, local/scripted shortcut rejection, browser screenshot proof checks, and full benchmark checker calls.

## OpenCode source backing retained

This repair is proof-harness work, not a new runtime parity claim. The runtime parity slice being protected by this harness is still backed by:

- `packages/opencode/src/session/processor.ts` — ToolPart lifecycle, `completeToolCall`, `failToolCall`, `toolResultOutput`, repeated tool-call doom-loop guard.
- `packages/schema/src/v1/session.ts` — ToolPart / ToolState / FilePart schema envelopes.

## Status

Current-head WebUI/NVIDIA NIM proof remains **NOT PROVEN** until same-head workflow artifacts include:

- `full-benchmark-webui.png`
- `full-benchmark-browser-proof.json`
- `full-benchmark-checker.json`
- `opencode-workflow-checker.json`
- `live-proof-status.txt`
