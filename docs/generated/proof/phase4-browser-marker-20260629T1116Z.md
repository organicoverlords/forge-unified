# Phase 4 browser marker proof fix — 2026-06-29T11:16Z

## Repo / branch / head before patch

- Repo: `organicoverlords/forge-unified`
- PR: #3
- Branch: `mvp/nim-freellmapi-router-20260626`
- Inspected head: `117a0ebe9b11c84a760190a472c02cac05f1869b`

## Same-head workflow state inspected

- Build Proof `28367497968`: success.
- CI `28367497919`: failure in `Smoke Test` / `Validate WebUI proof harness`.
- Live WebUI Feature Sprint `28367497952`: failure in `live-webui-feature-sprint` and `Check full benchmark evidence and quality score`.
- Live artifact inspected at metadata level: `7950149603`, `live-webui-feature-sprint-proof`, digest `sha256:f7d1bce77f63d38d6adb07bdc037ed27a5baee23799f1ccd3f7a8fdd91f247bd`.

## OpenCode source inspected before patch

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant behavior: when max steps are reached, tools are disabled and the final text must summarize work done, remaining tasks, and next recommendations. Forge's quality gate should verify evidence and browser usefulness without requiring one specific edit tool when the benchmark contract accepts any real repo edit tool.

## Fix

`score-live-benchmark-quality.py` previously required the literal browser marker `apply_patch` for useful browser proof. Forge's benchmark and orchestrator accept Phase 4 evidence from `file_edit`, `file_write`, or `apply_patch` outside `.agent_test`, and the hard evidence checker already verifies the actual successful tool result.

The quality scorer now accepts any Phase 4 browser edit marker from:

- `apply_patch`
- `file_edit`
- `file_write`
- `Applied patch`
- `Edited file`
- `Wrote file`

This keeps the hard proof semantics intact while removing a false negative when the model correctly uses `file_edit` or `file_write`.

## Files changed

- `scripts/smoke/score-live-benchmark-quality.py`
- `docs/generated/proof/phase4-browser-marker-20260629T1116Z.md`

## Status

New source-fix commit: `06cf726acc6c834394f6b8f9f76ec8ab1b5188c8`.

Do not claim same-head benchmark proof until CI, Build Proof, and Live WebUI Feature Sprint are green on this commit or a later head containing it.
