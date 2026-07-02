# Final report template contract CI gate proof

Date: 2026-06-29T13:58Z
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection basis

The target URL branch remains the selected source of truth. PR #3 is open, non-draft, and mergeable in GitHub PR metadata. The inspected same-head workflow state for `6d020ea0458273046eca089db654d336718db9c3` was:

- Build Proof `28373524856`: success.
- CI `28373524848`: success.
- Live WebUI Feature Sprint `28373524809`: failure.

## Failure inspected

The failed Live WebUI job was `84061799401`.

The run executed the live WebUI benchmark harness with the NVIDIA NIM route configured. The checker output showed the benchmark reached and passed the hard file-operation portions:

- Phase 1 repo identity evidence: passed.
- Phase 2 long tool-loop evidence: passed.
- Phase 3 file write/read/delete proof: passed.
- Phase 4 real low-risk edit: passed.
- Phase 4 validation command: passed.

The remaining failure was final-report quality / presence:

- `phase2_confidence_labels_in_answer`: failed.
- `phase4_risk_and_rollback_in_answer`: failed.
- `phase5_founder_report_present_and_under_180_words`: failed with word_count 0.
- `phase5_technical_report_has_required_sections`: failed.
- `final_reports_files_tests_risks_confidence`: failed.

## Source-backed parity slice

Added `scripts/smoke/check-final-report-template-contract.py` and wired it into `.github/workflows/ci.yml`.

This is not a docs-only change. It adds an enforceable CI gate that validates Forge's max-step finalization contract against the app's orchestrator and the live benchmark prompt:

- The orchestrator must retain all required final Markdown labels.
- The live prompt must retain all required final Markdown labels.
- The forced finalization path must disable tools with `tools: None` and `tool_choice: None`.
- The forced finalization path must validate provider final text with `looks_like_final_report`.
- The forced finalization path must fall back to deterministic Markdown with `fallback_final_report` if the provider returns malformed or empty final text.
- The fallback must not hard-claim cargo build/check/test success as VERIFIED.

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant behavior copied as a contract:

- Tools are disabled once max steps are reached.
- The final response must be text-only.
- The response must summarize work done so far.
- The response must list remaining tasks.
- The response must include recommendations for next steps.

Forge-specific extension:

- For this WebUI benchmark, the text-only final response must also satisfy the benchmark's Founder/Technical report labels and evidence-bound claim discipline.

## Files changed

- Added `scripts/smoke/check-final-report-template-contract.py`.
- Updated `.github/workflows/ci.yml` to compile and run the new gate in CI smoke.
- Updated `PROJECT_STATE.md` after this proof.

## Proof status

This commit has not yet produced same-head CI / Build Proof / Live WebUI proof. The next accepted proof must be on this head or a later head containing this gate, and must include real NVIDIA NIM WebUI artifacts.
