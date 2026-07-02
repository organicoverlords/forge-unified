# OpenCode parity proof — tool doom-loop guard

Date: 2026-06-27T23:58Z
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Source of truth

Target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

Selected branch remains `mvp/nim-freellmapi-router-20260626` because PR #3 is open, non-draft, and its head branch is the requested target branch.

## OpenCode source backing

Upstream path used:

- `anomalyco/opencode/packages/opencode/src/session/processor.ts`
  - `DOOM_LOOP_THRESHOLD = 3`
  - `tool-call` handling compares the latest `DOOM_LOOP_THRESHOLD` tool parts by tool name and serialized input.
  - When the same tool/input repeats across the threshold, OpenCode asks the permission layer for `doom_loop` before continuing.

## Implemented Forge slice

`crates/engine/src/orchestrator.rs` now carries a small source-backed runtime guard for repeated identical tool requests:

- tracks tool request signatures by round after safety filtering;
- detects three consecutive identical tool signature batches;
- interrupts the loop before executing the third repeated batch;
- writes a visible assistant-side guard message into the conversation;
- records `opencode_doom_loop_interrupted`, `opencode_doom_loop_threshold`, and `opencode_doom_loop_source` in the returned run metadata.

This is intentionally conservative: Forge does not yet have OpenCode's full permission-question layer, so the copied behavior is limited to threshold detection and safe interruption, not interactive permission recovery.

## Files changed

- `crates/engine/src/orchestrator.rs`
- `docs/generated/proof/opencode-doom-loop-guard-20260627T2358Z.md`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`

## Validation status

Connector-side code editing completed through GitHub API. Same-head GitHub Actions must prove compile/browser behavior. Browser screenshot proof with NVIDIA NIM only is still required for the final HEAD before live parity can be claimed.

## Do not overclaim

This is not full OpenCode parity. Forge now has OpenCode-backed repeated-tool-loop detection and interruption, but it does not yet implement the full OpenCode `permission.ask({ permission: "doom_loop" ... })` interaction path.