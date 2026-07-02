# WebUI tool preview evidence controls — 2026-06-30T07:55Z

## Selection basis

- Repo: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open, non-draft, mergeable at API check.
- Selected because the provided target branch remains the active open PR carrying the WebUI / NVIDIA NIM proof work; no newer open PR superseded it.

## Baseline proof before this slice

Same-head baseline before this slice: `bee2dfe3530f8a3b854430c13ad528bfda436c11`.

Workflows on that baseline:

- Live WebUI Feature Sprint `28426015584`: success.
- Fast WebUI Proof `28426015591`: success.
- App Build Proof `28426015585`: success.
- Build Proof `28426015594`: success.
- App Multistep Build Proof `28426015598`: success.
- CI `28426015609`: success.

Live WebUI artifact:

- Artifact ID: `7973188948`
- Artifact name: `live-webui-feature-sprint-proof`
- Digest: `sha256:b60e21a8986162db55fc6df23f2962808c5744f675cbb0c37624975a112defde`

## Feature built

Implemented a WebUI typed-tool preview evidence pane for screenshot-verifiable tool cards.

User-visible controls added inside typed tool cards:

- `show preview`
- `copy preview`

The preview pane is separate from the existing evidence controls:

- `show input`
- `show result`
- `copy target`
- `copy result`
- `copy input`
- `copy diagnostics`

The goal is to make WebUI screenshots show a readable, collapsed preview of what a tool actually did without forcing the primary proof surface to expose raw JSON.

## Forge implementation paths

- `crates/webui/src/chat_ui_enhancements.html`
  - Added `.tool-preview-pane` CSS.
  - Added `previewFrom(kind,state,result,args)`.
  - Added `tool-preview-visible`, `tool-preview-toggle`, and `tool-preview-copy` proof markers.
  - Added `show preview` and `copy preview` controls.
- `scripts/smoke/check-webui-proof-part-contract.py`
  - Added deterministic UI/proof-trail checks for preview pane, preview toggle, and copy preview.
- `PROJECT_STATE.md`
  - Recorded the same-head green baseline and latest slice boundary.

## OpenCode source backing used as reference only

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - `ReadTool`, `WriteTool`, `EditTool`, `BashTool`, `GrepTool`, `GlobTool`, and `TaskTool` split tool title/target/result handling by tool type.
  - `ResultsButton` provides a collapsed/expandable result pattern.
  - `getDiagnostics(...)` and `flattenToolArgs(...)` provide the diagnostics and fallback argument-display pattern.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css`
  - Used as the source anchor for share/session part visual structure.

## Acceptance boundary

This slice is a source-backed WebUI evidence-usability improvement.

It does not claim:

- Complete UI parity.
- Complete OpenCode parity.
- Production readiness.
- Same-head acceptance for the latest commit.

Same-head proof is required on the post-slice head before acceptance.
