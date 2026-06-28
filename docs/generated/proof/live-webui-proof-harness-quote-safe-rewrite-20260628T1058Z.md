# Live WebUI proof harness quote-safe rewrite — 2026-06-28T10:58Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft into `master`
- Source-of-truth target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Failure inspected

Same-head workflow runs for `4a3c783b9993aa376f342e8aeff16da34625026a` appeared after the previous checkpoint:

- Live WebUI Feature Sprint `28318502067`: failed.
- Build Proof `28318502064`: failed.
- CI `28318502071`: failed.

Live WebUI job `83896051926` compiled `forge-app`, then failed before benchmark artifacts were created:

```text
scripts/smoke/live-webui-feature-sprint.sh: line 393: unexpected EOF while looking for matching '"'
missing full benchmark conversation or stream artifact
```

Uploaded artifact `7933446069` is failure evidence only, not accepted WebUI/NVIDIA NIM screenshot proof.

## Repair

Replaced `scripts/smoke/live-webui-feature-sprint.sh` with a shorter quote-safe harness that keeps the same proof intent while removing fragile tail quoting:

- self-lints with `bash -n "$0"` before doing work;
- uses shared `post_stream` for natural-language WebUI prompts;
- keeps NIM-only rejection gates for local/scripted proof paths;
- keeps provider-visible `/api/tools` independence guards rejecting `packages/opencode/` and `opencode_` markers;
- captures tool-lifecycle and full six-phase benchmark browser screenshots;
- requires full benchmark checker and OpenCode-workflow behavior checker to pass before success;
- writes `live-proof-status.txt` with plain `printf` lines.

## OpenCode source backing

OpenCode remains behavior reference material only. The harness records and verifies behavior-level parity without exposing OpenCode identity through Forge provider-visible `/api/tools` output.

Source anchors retained in developer docs:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — ToolPart lifecycle behavior: `ensureToolCall`, `updateToolCall`, `completeToolCall`, `failToolCall`, provider-executed tool result handling, and normalized file attachment behavior.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — ToolPart, ToolState, FilePart, and base part schema behavior.

## Do not overclaim

This commit repairs the proof harness. It is not accepted parity proof until the same-head Live WebUI Feature Sprint produces a green artifact containing browser screenshots, NIM stream/conversation JSON, and passing checker JSON.
