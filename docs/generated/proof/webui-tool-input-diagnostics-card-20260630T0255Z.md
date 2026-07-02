# WebUI tool input diagnostics card proof — 2026-06-30T02:55Z

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open / non-draft / mergeable
- Source-of-truth target URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

No newer open PR superseded PR #3 for this run. The branch remains the active WebUI/NVIDIA NIM proof branch.

## Baseline inspected before this slice

Same-head proof passed on pre-slice head `1a351d9914935395d212fc2ff6be42c28b223222`:

- App Build Proof `28414920902`: success
- Fast WebUI Proof `28414920943`: success
- App Multistep Build Proof `28414920894`: success
- Build Proof `28414920904`: success
- CI `28414920900`: success
- Live WebUI Feature Sprint `28414920920`: success

Live WebUI proof artifact:

- `7969229350` — `live-webui-feature-sprint-proof`

## Feature slice implemented

Added browser-visible typed tool-card detail summaries in `crates/webui/src/chat_ui_enhancements.html`:

- flattened public tool input fields using `tool-args-grid`
- visible result count / match count summaries using `tool-result-count`
- diagnostic extraction into `tool-diagnostic-list`
- `copy input` action beside existing `show result` and `copy result`
- browser proof markers: `tool-args-visible`, `tool-diagnostics-visible`, `tool-count-summary-visible`
- runtime scrubber removes or replaces `opencode_*` and `anomalyco/opencode:*` source-reference strings from browser-visible tool summaries

This keeps upstream OpenCode paths as developer proof references only; the Forge runtime UI remains Forge-branded.

## OpenCode source backing

Exact upstream paths used as reference:

- `packages/web/src/components/share/part.tsx`
  - `GrepTool` / `GlobTool`: count summaries and result disclosure
  - `ReadTool` / `WriteTool` / `EditTool`: visible file targets, previews, diffs, diagnostics
  - `TaskTool`: task target/input/output structure
  - `FallbackTool` / `flattenToolArgs`: readable flattened tool arguments
  - `ResultsButton`: collapsed result disclosure
  - `ToolFooter`: duration visibility
- `packages/web/src/components/share/part.module.css`
  - component/disclosure structure reference

## Deterministic gate updated

Updated `scripts/smoke/check-webui-proof-part-contract.py` to require:

- `tool-args-visible`
- `tool-diagnostics-visible`
- `tool-count-summary-visible`
- `tool-args-grid`
- `tool-diagnostic-list`
- `tool-result-count`
- `copy input`
- `flattened tool input`
- `diagnostics`
- `count summary`

## Claim boundary

This is one WebUI proof/readability parity slice. It does not claim complete OpenCode parity, production readiness, or same-head acceptance until GitHub workflows complete on the final pushed head and browser artifacts/screenshots are inspected.
