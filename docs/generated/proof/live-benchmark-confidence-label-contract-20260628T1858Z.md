# Live benchmark confidence-label contract repair — 2026-06-28T18:58Z

## Selection basis

- Source-of-truth target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- Matching PR: #3, open, non-draft, mergeable.
- No newer open PR or recently pushed non-main app branch superseded the target branch during this run.

## Inspected live state

Same-head workflow set for `77e331114181a9c0bf92c5b3e405e6d0385ae36a`:

- CI `28330840313`: success.
- Build Proof `28330840323`: success.
- Live WebUI Feature Sprint `28330840331`: failure.
- Live WebUI job: `83928605127`.
- Uploaded artifact: `7937155412`, `live-webui-feature-sprint-proof`, digest `sha256:ff760f222324abb2af13331adb893e4ff89381bfd3eec5cbf75bcae210316ad1`.

The failed live run was not a provider or tool-loop failure. It used real `nvidia_nim` with model `deepseek-ai/deepseek-v4-flash`, emitted 28 `tool-call` events and 28 `tool-result` events, passed the workflow checker, and failed only `phase2_confidence_labels_in_answer` because the final answer omitted the exact `VERIFIED`, `LIKELY`, and `UNKNOWN` labels.

## Repair

Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` so the natural-language WebUI benchmark prompt now requires the final answer to begin with an exact confidence block:

```text
confidence
- VERIFIED: evidence directly proven by successful tool results.
- LIKELY: supported conclusion that is not fully proven.
- UNKNOWN: remaining uncertainty.
```

The prompt also keeps the Technical report confidence labels and adds a final self-check instructing the model to rewrite its final answer before sending if any of the three exact uppercase labels are missing.

## OpenCode source backing

Reference paths used:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `completeToolCall`
  - `failToolCall`
  - `toolResultOutput`
  - `handleEvent` tool-result handling
- `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - ToolPart / ToolState / FilePart shape retained as the runtime evidence model.

Relevant behavior copied in Forge terms: final prose must be derived from completed tool-result state, not from ungrounded claims. The checker remains strict; the repair strengthens the natural-language contract so the live model is more likely to emit the same confidence semantics that the evidence checker requires.

## Changed files

- `scripts/smoke/full-agentic-benchmark-prompt.txt`
- `docs/generated/proof/live-benchmark-confidence-label-contract-20260628T1858Z.md`
- `PROJECT_STATE.md` in the follow-up state update.

## Proof status

Current-head same-head WebUI/NVIDIA NIM proof is still pending after this patch. Do not claim parity until the new Live WebUI Feature Sprint artifact contains:

- `full-benchmark-webui.png`
- `full-benchmark-browser-proof.json`
- `full-benchmark-stream.sse`
- `full-benchmark-conversation.json`
- `full-benchmark-checker.json` with `passed: true`
- `opencode-workflow-checker.json` with `passed: true`
