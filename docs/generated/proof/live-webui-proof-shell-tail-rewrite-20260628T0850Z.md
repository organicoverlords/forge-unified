# Live WebUI proof shell tail rewrite — 2026-06-28T08:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Target branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open and non-draft
- Inspected failed head: `a7d081a666da2359b86a040c4856569dc6822687`
- Repair commit: `03d36ddb5538df91401ff22ae5e80f7c146fa83d`

## Failure inspected

Latest same-head workflows for `a7d081a666da2359b86a040c4856569dc6822687` all failed:

- Live WebUI Feature Sprint: `28315640506`
- Build Proof: `28315640508`
- CI: `28315640515`

Live WebUI job `83888237891` built `forge-app`, then failed before any live browser benchmark proof could run:

```text
scripts/smoke/live-webui-feature-sprint.sh: line 272: unexpected EOF while looking for matching `"'
```

The root cause was not application runtime behavior. The proof harness itself still contained literal line breaks inside Python and shell string literals in the final status/echo block. That makes Bash parse fail before screenshot proof or checker artifacts can be produced.

## Repair

Replaced the fragile final proof-status writer with a quote-safe shell function:

- `write_status()` emits one `key=value` line per `echo` inside a grouped redirect.
- Final success reporting uses plain `echo` lines, not embedded escaped newlines.
- Conversation creation now uses Python-generated JSON plus `curl --data-binary` so titles do not rely on hand-built shell JSON escaping.
- NIM-only checks, local/scripted-shortcut rejection, provider tool checks, checker gates, and browser screenshot gates are preserved.

## OpenCode source backing retained

The proof workflow continues to verify behavior grounded in these upstream OpenCode references:

- `packages/opencode/src/session/processor.ts` — ToolPart lifecycle, provider-executed tool metadata, same-call part update semantics, `completeToolCall`, `failToolCall`, `toolResultOutput`, normalized attachment handling, doom-loop interruption behavior.
- `packages/schema/src/v1/session.ts` — ToolPart / ToolState / FilePart schema envelopes.
- `packages/opencode/src/event-v2-bridge.ts` — event receipt bridge behavior.
- `packages/opencode/src/tool/apply_patch.ts`, `todo.ts`, `write.ts`, `edit.ts`, `read.ts`, `bash.ts`, `glob.ts`, `grep.ts`, `ls.ts`, `webfetch.ts` — provider-visible tool catalog and tool behavior anchors.

## Proof status

Not proven on this repair commit yet. The previous run failed before screenshot/checker artifact creation, and same-head workflow proof for `03d36ddb5538df91401ff22ae5e80f7c146fa83d` was not yet complete when this note was committed.
