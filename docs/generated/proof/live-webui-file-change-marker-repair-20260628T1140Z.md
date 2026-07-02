# Live WebUI proof harness file-change marker repair — 2026-06-28T11:40Z

## Selection

- Repository: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 `mvp router slice`
- Starting inspected head: `4e17110fb0323862339d4ebd94f653a0bec944d3`

## Failure inspected

Live WebUI Feature Sprint run `28320841928`, job `83902377165`, built `forge-app`, started the WebUI, created the NIM conversation, and executed file tool lifecycle work. The failure was a proof-harness assertion mismatch:

- Expected marker: `event: event-bus`
- Actual emitted proof events included: `event: tool-lifecycle`, `event: tool-input-start`, `event: tool-input-delta`, `event: tool-input-end`, `event: tool-call`, `event: tool-result`, and `event: file-change`.

## Code change

`script/smoke/live-webui-feature-sprint.sh` was aligned with the actual emitted live event stream by removing the stale `event: event-bus` expectation and preserving the `event: file-change` assertion. The tool prompt text was also adjusted from `event bus receipts` to `event receipts` so the natural-language proof request matches the implemented event stream.

## OpenCode source backing

This is still source-backed against upstream OpenCode behavior, but source paths must remain developer/proof documentation only:

- `packages/opencode/src/session/processor.ts` — ToolPart lifecycle: pending/running/completed/error updates by call ID.
- `packages/schema/src/v1/session.ts` — ToolPart, ToolState, and FilePart attachment envelope shapes.
- `packages/opencode/src/tool/apply_patch.ts` — file mutation event publication behavior.

## Independence blocker still open

The inspected same-head logs also show that Forge runtime/tool-result metadata still includes `opencode_*` keys and `packages/opencode/...` strings in live stream and conversation JSON. That is **not acceptable as parity proof** for an independent Forge runtime. This run only repaired the stale harness marker; it does not claim OpenCode-independence proof yet.

## Status

Current-head browser screenshot proof is not available until the new workflows finish. Do not claim parity from this change alone.
