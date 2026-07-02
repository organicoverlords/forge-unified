# WebUI startup readiness proof slice

Date: 2026-06-28
Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection basis

The target URL points at `mvp/nim-freellmapi-router-20260626`. The matching PR is #3, open, non-draft, and points at that branch. No newer open PR superseded it during this run.

## Failing live state inspected

Current inspected head before this slice: `1ed62be3c68e7eb402505afba552e18f6352ca39`.

Same-head workflow state at that head:

- CI `28306664889`: failed.
- Build Proof `28306664881`: failed.
- Live WebUI Feature Sprint `28306664885`: failed.

The CI failed in `Smoke Test` at the WebUI startup readiness gate. The binary compiled, then the harness ran `start webui` / `wait health`, and failed with:

```text
curl: (7) Failed to connect to 127.0.0.1 port 3320 after 0 ms: Couldn't connect to server
```

The failed smoke still uploaded artifact `7929566682` (`webui-smoke-proof`), digest `sha256:ee92ec9b609bd80be771b26c396a822e8a250edc3b87d644c7b2374a1763e1be`.

## Slice implemented

File changed: `scripts/smoke/live-webui-feature-sprint.sh`.

The live proof harness now:

- Builds `forge-app` as before.
- Starts the already-built binary directly from `target/debug/forge` instead of invoking `cargo run` again.
- Records the exact server command in `server-command.txt` for uploaded proof artifacts.
- Waits up to 180 seconds for `/api/health`.
- Distinguishes process-exit-before-health from health-timeout.
- Prints the server log on either startup failure path.

This is not a parity claim by itself; it is a proof-infrastructure repair required to restore same-head WebUI/NVIDIA NIM proof after app changes.

## OpenCode source backing retained

This run also re-inspected upstream OpenCode source for the next functional parity target:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `DOOM_LOOP_THRESHOLD = 3`.
  - The repeated recent tool-part comparison.
  - The full recovery path using `permission.ask({ permission: "doom_loop", patterns, sessionID, metadata, always, ruleset })`.
  - Tool cleanup that marks interrupted tool calls with error state and `metadata.interrupted = true`.

Forge already has the threshold/interruption slice. The remaining functional slice is the real permission/recovery path, not just interruption text.

## Proof status

Not proven yet on the new head. A same-head workflow run must complete and upload WebUI/NVIDIA NIM screenshots before parity or benchmark correctness is claimed.
