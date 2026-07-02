# Live WebUI finalization time budget proof — 2026-06-29T16:55Z

## Selection

- Repo: `organicoverlords/forge-unified`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open, non-draft, mergeable in PR metadata.
- Pre-slice selected head: `adcca81b32ac9d6968d1804077f53c36b22e5c41`.

## Failed run inspected

- CI `28386515572`: success.
- Build Proof `28386515636`: success.
- Live WebUI Feature Sprint `28386515646`: failure.
- Failed job: `84102450955`.
- Provider/model evidence from the failed run: `nvidia_nim` / `deepseek-ai/deepseek-v4-flash`.
- Tool evidence from the failed run: 25 tool-call events, 21 tool-result events, 36 tool results.
- OpenCode workflow checker passed.
- The run timed out while final Markdown text was still streaming after tool evidence and validation evidence were already present.

## OpenCode source backing

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Relevant behavior: after max steps, tools are disabled and the agent must respond with text only, summarize completed work, list remaining work, and recommend next steps.

## Forge slice

- Added `scripts/smoke/check-live-webui-time-budget.py`.
- The gate asserts the Live WebUI workflow keeps at least an 840 second benchmark timeout and preserves final-report/text-only finalization markers in `scripts/smoke/full-agentic-benchmark-prompt.txt`.
- Updated `.github/workflows/live-webui-feature-sprint.yml` from `FORGE_BENCH_TIMEOUT_SECONDS=660` to `840` so real NIM runs have room to finish the required final Markdown report after the evidence-completing validation command.
- Wired the new gate into `.github/workflows/ci.yml` smoke validation.

## Proof status

This slice is pushed but not same-head proven until CI, Build Proof, and Live WebUI Feature Sprint all pass on the new head and the Live artifact contains browser screenshot proof plus NVIDIA NIM provider/model/tool evidence.
