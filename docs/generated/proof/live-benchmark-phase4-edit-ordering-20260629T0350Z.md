# Live benchmark Phase 4 edit ordering hardening

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

The target URL branch remains the active source of truth for PR #3. PR #3 is open, non-draft, and mergeable into `master`.

## Failure inspected

Head `2f02415c3008f4e476b6176a9028fec7cb3293f1` had:

- CI `28346413124`: success.
- Build Proof `28346413122`: success.
- Live WebUI Feature Sprint `28346413121`: failure.

The Live WebUI run used real `nvidia_nim` / `deepseek-ai/deepseek-v4-flash` evidence and passed workflow/tool visibility checks, but `scripts/smoke/check-full-agentic-benchmark.py` rejected the full benchmark because `phase4_real_low_risk_edit` had no successful dedicated file-editing tool result outside `.agent_test`.

## Source-backed change

Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` so the Phase 4 repository edit is the immediate next tool operation after `.agent_test` verification and the final report is explicitly blocked until a successful dedicated `file_edit`, `file_write`, or `apply_patch` result outside `.agent_test` exists.

This keeps proof semantics aligned with upstream OpenCode ToolPart behavior: final reports must be based on completed tool states, not inferred prose.

## OpenCode source backing

Reference paths inspected/retained:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — `updateToolCall`, `completeToolCall`, `failToolCall`, and `toolResultOutput` semantics.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — ToolPart and ToolState evidence shape.
- `anomalyco/opencode:packages/opencode/src/tool/edit.ts` and `packages/opencode/src/tool/write.ts` — dedicated file mutation tool behavior.

## Files changed

- `scripts/smoke/full-agentic-benchmark-prompt.txt`
- `PROJECT_STATE.md`
- `docs/generated/proof/live-benchmark-phase4-edit-ordering-20260629T0350Z.md`

## Proof status

This commit is not same-head proven yet. CI, Build Proof, and Live WebUI must complete on the new head before claiming latest-head proof. Prior accepted WebUI/NIM proof remains `c12789a7b7c59ba7bfe0ba22118892396356fc7c` / artifact `7941120525`.
