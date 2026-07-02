# Live benchmark timeout repair — 2026-06-28T15:20Z

## Source of truth

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3
- Inspected failed head: `1b62fa7193c41ac980af828220ee2a685af608d8`
- Failed Live WebUI Feature Sprint run: `28325968948`
- Failed artifact: `7935806412`

## Artifact finding

The latest same-head Live WebUI run did not fail because the WebUI was bypassed or because the model used no tools. The stream started the full benchmark through `/api/conversations/:id/chat/stream`, used `nvidia_nim`, and began real tool execution. It failed because the full six-phase benchmark did not finish within the 600 second proof budget. The proof archive then lacked `full-benchmark-conversation.json`, `full-benchmark-browser-proof.json`, `full-benchmark-webui.png`, and `opencode-workflow-checker.json`, so the follow-up checker emitted `missing_full_benchmark_artifacts`.

The stream also showed an avoidable early batch call failure: the model supplied batch request items as one-key shorthand objects like `{"repo_info":{}}`, while the executor only accepted `{tool,args}` items. The failed tool was returned as a tool result, which is correct, but it wasted benchmark rounds and wall-clock time.

## Repair landed

- `crates/engine/src/tool/batch.rs` now normalizes both explicit `{tool,args}` calls and one-key shorthand calls before execution.
- `crates/engine/src/tool.rs` now documents the accepted batch call shapes in the provider-visible tool schema.
- `scripts/smoke/full-agentic-benchmark-prompt.txt` now gives explicit round discipline, caps Phase 2 investigation after the checker-required evidence exists, and points the model at a fast validation command.
- `.github/workflows/live-webui-feature-sprint.yml` now uses a bounded 36-round / 540-second full-benchmark budget instead of 60 rounds, reducing the chance of long non-finishing loops.
- `scripts/smoke/live-webui-feature-sprint.sh` now fetches the benchmark conversation and writes both checker JSON files even when the stream times out, so future failures preserve actionable artifacts instead of only `missing_full_benchmark_artifacts`.

## Validation expectation

The next same-head proof must still pass the real checks. This repair does not fake checker output and does not add a local/scripted benchmark shortcut. The full benchmark must still finish through the WebUI, use NVIDIA NIM, create/read/delete/edit files through real tool results, and produce the required browser/screenshot/checker artifacts.
