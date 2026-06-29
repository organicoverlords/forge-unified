# WebUI proof part contract gate — 2026-06-29T21:58Z

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open and non-draft, mergeable in PR metadata.
- Selected pre-slice head: `17ab8228c2c1e7291f4847d18d1963115cecd981`.
- Same-head status before this slice:
  - CI `28404596368`: success.
  - Build Proof `28404596364`: success.
  - App Build Proof `28404596398`: success.
  - Fast WebUI Proof `28404596399`: success.
  - Live WebUI Feature Sprint `28404596424`: in progress during inspection, inside `Run natural feature-build prompt through WebUI`.

## OpenCode source backing

Exact upstream OpenCode source paths used as reference:

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
- `anomalyco/opencode:packages/web/src/components/share/part.module.css`

These are the same UI/source anchors already retained in `PROJECT_STATE.md` for completed/share part rendering and visual treatment.

## Feature built

Added deterministic CI guard `scripts/smoke/check-webui-proof-part-contract.py` for the WebUI proof-part presentation contract.

The guard requires:

- visible provider/model proof markers;
- final proof digest and final answer visibility;
- human-readable tool labels for common actions;
- OpenCode-style completed/live tool part markers;
- technical details collapsed out of `proof=final` DOM;
- no primary `raw tool:` marker regression;
- proof trail retention for the two OpenCode source paths and human-label/final-proof rationale.

This is a source-backed parity/proof slice. It does not claim complete OpenCode parity.

## Files changed

- `scripts/smoke/check-webui-proof-part-contract.py`
- `.github/workflows/ci.yml`
- `PROJECT_STATE.md`
- `docs/generated/proof/webui-proof-part-contract-gate-20260629T2158Z.md`

## Claim boundary

Do not accept the latest head until same-head CI / Build Proof / Fast WebUI Proof / Live WebUI Feature Sprint complete with NVIDIA NIM browser proof artifacts on that exact SHA.
