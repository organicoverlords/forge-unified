# Live WebUI benchmark repair — mandatory Phase 4 before final report

Date: 2026-06-29T00:48Z
Branch: `mvp/nim-freellmapi-router-20260626`

## Live state checked

Selected PR #3 because it is the open, non-draft router PR for the target branch.

Latest checked head before this repair: `ed308b67a10a63392e00c693a99bdc08e66e8d05`.

Same-head workflow status for that head:

- CI `28340643191`: success.
- Build Proof `28340643221`: success.
- Live WebUI Feature Sprint `28340643197`: failure.
- Failed job: `83954399249`.

The failed Live WebUI run used the real NVIDIA NIM provider and model `deepseek-ai/deepseek-v4-flash`, with 29 tool-call events and 24 tool-result events, but the final benchmark checker rejected missing Phase 4/final-report evidence:

- `phase2_confidence_labels_in_answer`
- `phase4_real_low_risk_edit`
- `phase4_risk_and_rollback_in_answer`
- `phase5_founder_report_present_and_under_180_words`
- `phase5_technical_report_has_required_sections`
- `final_reports_files_tests_risks_confidence`

## Source-backed parity reference

OpenCode reference paths used as behavior backing:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
- `anomalyco/opencode:packages/schema/src/v1/session.ts`

The relevant behavior is exact ToolPart state/result evidence: final reports should be grounded in completed tool parts and should not infer file edits or tests that are not represented by a successful tool result.

## Change made

Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` to make Phase 4 mandatory before final answer:

- Requires a dedicated `file_edit`, `file_write`, or `apply_patch` tool result outside `.agent_test`.
- Forbids shell redirection for the Phase 4 edit because the checker requires file-editing tool evidence.
- Requires validation/state inspection after the Phase 4 edit.
- Repeats that the agent is not done after Phase 3 and must produce the final Markdown report only after Phase 4 evidence exists.

## Expected proof

The new head must still pass same-head CI, Build Proof, and Live WebUI Feature Sprint before this is accepted. Until those runs are green, this repair is not same-head proven.