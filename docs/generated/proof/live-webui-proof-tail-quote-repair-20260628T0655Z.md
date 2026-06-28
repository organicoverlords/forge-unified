# Live WebUI proof tail quote repair — 2026-06-28T06:55Z

Status: NOT PROVEN ON THIS HEAD YET.

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open and non-draft
- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Inspected current-head failure

- Previous current HEAD: `5d464d688e752af54d30c2d703ab1988aae7e98c`
- Live WebUI Feature Sprint run: `28312998276`
- Failed job: `83880953227`
- Failed artifact: `7931549702` (`live-webui-feature-sprint-proof`), failure evidence only, not screenshot proof
- Failure: `scripts/smoke/live-webui-feature-sprint.sh: line 249: unexpected EOF while looking for matching '"'`
- The script built `forge-app` successfully before failing at shell parsing, so the failure was in the proof harness tail, not in Rust compilation.

## Repair

- Replaced the final heredoc/status/echo block with a Python status writer and small `printf` lines.
- Kept the NIM-only gates, local/scripted shortcut rejection, tool lifecycle browser proof, full benchmark browser proof, OpenCode workflow checker, and normalized attachment proof markers.
- This is a proof-harness repair only; no parity claim is made until same-head Live WebUI/NVIDIA NIM screenshot artifacts pass.

## OpenCode source backing retained

- `packages/opencode/src/session/processor.ts` — `Handle.completeToolCall`, `updateToolCall`, `failToolCall`, `ensureToolCall`.
- `packages/opencode/src/session/processor.ts` — `toolResultOutput` normalized `FilePart` attachment handling.
- `packages/opencode/src/session/processor.ts` — EventV2Bridge and doom-loop workflow evidence anchors.
- `packages/schema/src/v1/session.ts` — ToolPart / ToolState / FilePart schema envelopes.

## Next check

Inspect workflows for the new head and only accept proof if the same-head Live WebUI Feature Sprint produces `full-benchmark-conversation.json`, `full-benchmark-stream.sse`, `full-benchmark-checker.json`, `full-benchmark-browser-proof.json`, and `full-benchmark-webui.png` from NVIDIA NIM.
