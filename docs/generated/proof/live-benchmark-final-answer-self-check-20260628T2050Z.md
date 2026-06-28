# Live benchmark final-answer self-check repair — 2026-06-28T20:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft, target branch from the provided URL.
- Previous latest head inspected: `009137bf1760144ce841efb8d4975a368afed50a`.

## Failed workflow inspected

- CI run `28333950469`: failed only in Smoke Test; Test, Check, Cargo Deny, Security Audit, and File Size Gate were green.
- Live WebUI Feature Sprint run `28333950462`: failed in `live-webui-feature-sprint` after real WebUI/NVIDIA NIM execution.
- Build Proof run `28333950465`: success.
- Failed proof artifact: `7938035145`, digest `sha256:2f7b85d30a0ae06793b64cea216155e653c0fd1873d74d5be49c658fe36c2174`.

## Failure diagnosis

The run used provider `nvidia_nim` with model `deepseek-ai/deepseek-v4-flash` and produced real tool-call/tool-result evidence. The checker failed because the final assistant text did not satisfy final-report contract requirements:

- missing `VERIFIED` / `LIKELY` / `UNKNOWN` labels in final answer,
- no accepted Founder report section,
- no accepted Technical report section,
- no final summary labels for files/tests/risks/confidence,
- no accepted Phase 4 risk/rollback prose.

## Repair

Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` to make final output constraints harder to miss:

- final answer must be Markdown text, not JSON/tool calls/empty output;
- exact confidence block still appears before all prose;
- final self-check now rejects blank, JSON, tool-call, or label-incomplete output;
- self-check enumerates the exact strings required by the checker.

## OpenCode source backing

Source path used for behavior reference:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`

Relevant upstream behavior: reports are only meaningful after the processor has settled tool calls into completed/failed ToolPart state with output/error evidence. This repair keeps the same evidence-first direction by forcing the benchmark prompt to report only after tool evidence exists and by making final prose contract failure visible to the checker instead of accepting vague success claims.

## Do not overclaim

This commit does not prove latest-head parity. It repairs the latest known failed final-answer contract. Same-head CI / Build Proof / Live WebUI Feature Sprint must pass before claiming proof on the new head.
