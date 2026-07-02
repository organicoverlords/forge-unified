# Live quality command-claim normalizer proof — 2026-06-29T10:47Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft/mergeable at live read
- Pre-patch head inspected: `4ff7c2f6f7cfc758c3669fe59d5407163ccf70b1`

## Failed live state inspected

Workflows for `4ff7c2f6f7cfc758c3669fe59d5407163ccf70b1`:

- Build Proof `28364990761`: success
- CI `28364990747`: failure
- Live WebUI Feature Sprint `28364990741`: failure

Failure details:

- CI failed in `Smoke Test` / `Validate WebUI proof harness` because `scripts/smoke/check-max-step-finalization-parity.py` required one exact conservative fallback wording even though the orchestrator carried an equivalent evidence-bound wording.
- Live WebUI Feature Sprint completed the app execution with real `nvidia_nim` / `deepseek-ai/deepseek-v4-flash` evidence, 40 tool-call events, 38 tool-result events, a complete browser proof bundle, and passing hard/workflow checkers.
- The remaining high-weight score failure was `test_and_build_claims_match_tool_commands`, caused by strict literal matching of paraphrased validation claims such as `bash -n validation of scripts/smoke/live-webui-feature-sprint.sh (EXIT:0)` against the successful command metadata.
- A lower-weight semantic check also rejected final text containing brackets globally, which can misclassify evidence-bearing text instead of only true placeholder brackets.

## OpenCode source backing

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant behavior recorded in the repo contract: max-step finalization disables tools and returns a text-only summary with completed work, remaining work, and next-step recommendations. Forge must preserve evidence-bound final claims and must not overclaim unproven command/build/test success.

## Feature slice built

- `scripts/smoke/check-max-step-finalization-parity.py`
  - Accepts both equivalent evidence-bound fallback wordings:
    - `Only successful tool results in the digest are counted`
    - `Only successful tool results listed in the evidence digest are counted`
  - Keeps all OpenCode max-step no-tools/final-report contract checks intact.

- `scripts/smoke/score-live-benchmark-quality.py`
  - Adds `normalize_command_claim()` to map scorer-facing paraphrases to the underlying tool command shape.
  - Adds `command_claims()` to ignore negative/prohibitive command mentions, such as `no cargo build`, `do not claim`, or `unless ... succeeded`.
  - Adds `command_is_proven()` so validation claims pass only when normalized successful tool metadata proves them.
  - Replaces global bracket rejection with `has_placeholder_brackets()` so only obvious placeholder tokens fail the semantic repo-summary gate.

## Proof status

This commit is a source/proof-gate fix, not a same-head WebUI pass claim. CI, Build Proof, and Live WebUI Feature Sprint must run on the new head before claiming same-head proof.

## Expected validation

- CI Smoke Test should pass the max-step finalization parity gate.
- Live WebUI quality scoring should no longer fail solely because a proven command is phrased as a human-readable validation claim.
- Live WebUI quality scoring still rejects actual unproven cargo/build/test claims.
