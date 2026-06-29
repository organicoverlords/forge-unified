# Natural WebUI finished-stream transport proof

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`
Base before slice: `9a8f0157e75f2017200764abbcf10ac4ba4d1bf9`

## Live failure inspected

Live WebUI Feature Sprint run `28409705410`, job `84179868033`, failed in the natural feature-build proof step after the full benchmark had already passed.

Observed evidence from the job log:

- Full benchmark WebUI/NVIDIA NIM proof passed with provider `nvidia_nim` and model `deepseek-ai/deepseek-v4-flash`.
- Full benchmark checker passed with 24 tool-call events and 24 tool-result events.
- Natural feature-build proof used provider `nvidia_nim` and model `deepseek-ai/deepseek-v4-flash`.
- Natural feature-build proof had 17 tool-call events and 17 tool-result events.
- Natural feature-build browser proof and screenshot checks passed.
- The only failed natural feature-build check was `stream_exit_code_zero`, because curl returned exit code `28` after receiving stream data even though the run-finish marker and proof checks were present.

## Slice

Updated `scripts/smoke/natural-feature-work.sh` so the natural WebUI proof distinguishes transport timeout from model/tool failure:

- Replaced strict `stream_exit_code_zero` with `stream_transport_completed_or_run_finished`.
- Accepts stream exit code `0` normally.
- Accepts curl timeout codes `28` / `124` only when the SSE stream contains `event: run-finish`.
- Keeps provider/model, tool calls/results, edit markers, browser proof, screenshot, validation, and final-answer gates intact.
- Records `stream_exit_code` and `stream_transport_ok` in the generated proof summary.
- Removes the post-check hard fail that rejected a passed proof only because curl timed out after completion.

## OpenCode source backing

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts` — evidence-bound finalization and explicit final-summary behavior when the run has enough evidence.
- `anomalyco/opencode:packages/opencode/src/cli/cmd/run/turn-summary.ts` — concise turn/run summary carrying status metadata without treating transport framing as the only proof of completion.
- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool lifecycle completion/failure state semantics; Forge proof must key off recorded run/tool states, not raw transport exit alone.

## Claim boundary

This patch does not claim full OpenCode parity. It fixes one proof harness behavior so a completed WebUI/NIM natural-language run is not rejected solely because curl timed out after the app emitted completion and browser proof succeeded.

Latest head after this doc still needs same-head CI / Build Proof / Fast WebUI / Live WebUI proof before acceptance.
