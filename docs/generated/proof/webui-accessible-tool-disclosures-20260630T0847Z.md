# WebUI accessible tool disclosures proof — 2026-06-30T08:47Z

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626` from `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected work remains PR #3 because it is the active open, non-draft, mergeable PR for the requested branch and no newer open PR superseded it during this check.
- Baseline before this slice: `62932c83ef2a88cee3bbb090208e3e82ea3f5d52`.
- Same-head baseline proof before this slice:
  - CI `28429136936`: success.
  - Build Proof `28429136942`: success.
  - App Build Proof `28429136986`: success.
  - App Multistep Build Proof `28429136999`: success.
  - Fast WebUI Proof `28429136996`: success.
  - Live WebUI Feature Sprint `28429137007`: success.
  - Live WebUI artifact: `7974499647` / `live-webui-feature-sprint-proof`, created 2026-06-30T08:06:41Z.

## Feature slice

Forge WebUI typed tool cards already exposed separate target, input, preview, diagnostics, and result controls. This slice makes the evidence-disclosure controls explicit and machine/a11y-visible:

- `show input`, `show preview`, and `show result` now go through one `disclosureButton(...)` helper.
- Each disclosure binds to a concrete pane id using `aria-controls`.
- Each disclosure exposes open/closed state through `aria-expanded`.
- Each disclosure also mirrors the state through `data-state=open|closed` for browser proof and screenshot inspection.
- The CI smoke gate now requires `tool-toggle-aria-expanded`, `tool-toggle-state-visible`, `aria-expanded`, `aria-controls`, and durable proof-trail tokens for accessible disclosure state.

## OpenCode source backing

Used as developer reference only; no Forge runtime branding is exposed.

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - `ResultsButton` holds disclosure state and toggles visible tool result content.
  - Tool renderers expose named targets, previews, diagnostics, counts, and result panes.
  - `TaskTool`, `ReadTool`, `WriteTool`, `EditTool`, `GrepTool`, `GlobTool`, `BashTool`, and `FallbackTool` define the source-backed behavior shape.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css`
  - Share/session part visual contract source anchor for tool/result presentation.

## Forge files changed

- `crates/webui/src/chat_ui_enhancements.html`
- `scripts/smoke/check-webui-proof-part-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/webui-accessible-tool-disclosures-20260630T0847Z.md`

## Claim boundary

This is a source-backed WebUI evidence-usability slice. It does not claim full OpenCode parity, production readiness, or same-head acceptance for the new head until CI, Build Proof, App Build Proof, App Multistep Build Proof, Fast WebUI Proof, and Live WebUI Feature Sprint complete on the exact new head and their browser artifacts/screenshots are inspected.
