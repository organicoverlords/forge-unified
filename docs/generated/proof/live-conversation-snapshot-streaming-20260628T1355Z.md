# Live conversation snapshot streaming repair — 2026-06-28T13:55Z

Status: **implemented, pending same-head workflow/browser proof**.

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, `mvp router slice`, open and non-draft.
- Source of truth: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Failure inspected

Latest same-head failure before this repair:

- Head: `88796c89eeb2a3ab89d4702c12186848a3fde5d2`
- CI run: `28323689775`, failure
- Build Proof run: `28323689784`, failure
- Live WebUI Feature Sprint run: `28323689781`, failure
- Live WebUI job: `83909953124`
- Artifact: `7935117584`, failure evidence only

The Live WebUI job built `forge-app`, started the WebUI, completed the smaller live NIM and tool lifecycle stages, then timed out while the full six-phase natural-language benchmark was still streaming. The curl command received partial SSE bytes, but the endpoint only emitted full conversation/tool evidence after `agent.chat_with_max_rounds(...).await` returned, so the proof checker lacked complete artifacts.

## OpenCode source backing

Exact upstream paths used as behavior references:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `SessionProcessor.updateToolCall`
  - `ensureToolCall`
  - `completeToolCall`
  - `failToolCall`
  - live ToolPart updates keyed by call id
  - `providerExecuted` metadata propagation
- `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - ToolPart / ToolState envelope shape
  - completed/error state semantics

## Change landed

`crates/webui/src/events_live.rs` now emits live snapshots while the long agent run is executing:

- `conversation-snapshot` every 15 seconds
- current message count
- current tool result count
- provider/model identity
- deduplicated ToolPart lifecycle events
- deduplicated tool results
- deduplicated file-change and event-bus receipts
- text deltas from the latest assistant state

This keeps Forge behavior closer to OpenCode's mutable session-part model: visible tool/session state is updated during a long run, not only after a final agent return.

The workflow budget was also adjusted:

- `FORGE_BENCH_MAX_ROUNDS=36`
- `FORGE_BENCH_TIMEOUT_SECONDS=900`

This keeps the live NIM benchmark bounded while giving the real WebUI/NIM run enough wall-clock room to complete.

## Files changed

- `crates/webui/src/events_live.rs`
- `.github/workflows/live-webui-feature-sprint.yml`
- `PROJECT_STATE.md`
- `docs/generated/proof/live-conversation-snapshot-streaming-20260628T1355Z.md`

## Proof status

Do not overclaim.

This commit has not yet produced same-head green WebUI/NVIDIA NIM screenshot proof. The next required evidence is a green Live WebUI Feature Sprint artifact containing:

- `full-benchmark-webui.png`
- `full-benchmark-browser-proof.json`
- `full-benchmark-stream.sse`
- `full-benchmark-conversation.json`
- `full-benchmark-checker.json` with `passed: true`
- `opencode-workflow-checker.json` with `passed: true`
