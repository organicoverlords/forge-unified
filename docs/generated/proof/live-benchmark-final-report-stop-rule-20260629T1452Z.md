# Live benchmark final-report stop rule

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection

Selected PR #3 / branch `mvp/nim-freellmapi-router-20260626` because the target URL remains the source of truth and no newer open PR was found that supersedes it with meaningful app changes.

## Failure inspected

Latest pre-slice head: `9ad3db25c029c97f0c36b575add4ddebbe06b033`.

Workflow status on that head:

- Build Proof `28378470751`: success.
- CI `28378470831`: success.
- Live WebUI Feature Sprint `28378470554`: failure.
- Failed job: `84074310768`.

The failed live run used real NVIDIA NIM:

- Provider: `nvidia_nim`.
- Model: `deepseek-ai/deepseek-v4-flash`.
- Tool events: 28 `tool-call` and 28 `tool-result` events.

The checker showed Phase 1, Phase 2 tool-loop evidence, Phase 3 file write/read/delete evidence, Phase 4 edit evidence, Phase 4 validation evidence, and cleanup/state evidence were present. The failure remained in final Markdown report sections: confidence labels, risk/rollback wording, Founder report, Technical report, and final report files/tests/risks/confidence sections.

## OpenCode source backing

Inspected upstream OpenCode source:

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant behavior copied into the Forge live benchmark prompt contract:

- When the maximum/evidence-complete boundary is reached, tools are disabled.
- The model must respond with text only.
- The response must summarize completed work, remaining tasks, and next recommendations.
- Additional tool calls after that boundary violate the contract.

## Change made

Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` so that after the evidence-completing validation shell result:

- tools are explicitly disabled by instruction;
- the model must not rerun validation, inspect git status, write another file, or ask for more evidence;
- Phase 6 must use the existing step 10 validation/status output instead of another tool call;
- the final Markdown template remains mandatory.

Updated `PROJECT_STATE.md` with the live failure diagnosis and latest proof status.

## Validation status

This is a source-backed behavioral prompt change. CI / Build Proof / Live WebUI were expected to start on push. Latest head must not be claimed as same-head proven until those workflows complete green and the Live WebUI artifact contains browser screenshot proof, stream/transcript, checker JSON, quality-score JSON, conversation JSON, and NVIDIA NIM provider/model/tool evidence.
