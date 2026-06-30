# WebUI tool evidence controls — 2026-06-30T06:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Selected because the provided target branch remains the active open PR workstream and no newer open PR superseded it.
- Baseline before this slice: `f0d1fd6818b67f8d41d3885ed616002e90c5868b`.

## Baseline proof inspected

Same-head green baseline on `f0d1fd6818b67f8d41d3885ed616002e90c5868b`:

- App Build Proof `28423428922`: success.
- Build Proof `28423428937`: success.
- App Multistep Build Proof `28423428921`: success.
- Fast WebUI Proof `28423428944`: success.
- CI `28423428951`: success.
- Live WebUI Feature Sprint `28423428936`: success.
- Live WebUI artifact: `7972230611` / `live-webui-feature-sprint-proof`.

## Feature built

The WebUI typed tool-card renderer now exposes evidence controls that are useful in screenshot/browser proof without making raw JSON the primary UI:

- `show input` / `hide input` toggle for flattened public input.
- `copy target` for the exact visible target/path/pattern/command summary.
- `copy diagnostics` for error diagnostics extracted from tool metadata.
- Existing `show result`, `copy result`, and `copy input` remain available.
- New proof hooks: `tool-input-toggle`, `tool-diagnostic-copy`, `tool-target-copy`.

## OpenCode source backing

Used as source/reference material only; Forge remains independently branded.

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - Tool-specific renderers for `read`, `write`, `edit`, `bash`, `grep`, `glob`, `task`, and fallback tools.
  - `ResultsButton` for collapsible results.
  - `getDiagnostics` for file/line/column diagnostics.
  - `flattenToolArgs` for nested fallback tool input display.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css`
  - Share/session part visual contract anchor.

## Forge paths changed

- `crates/webui/src/chat_ui_enhancements.html`
- `PROJECT_STATE.md`
- `docs/generated/proof/webui-tool-evidence-controls-20260630T0650Z.md`

## Claim boundary

This is a source-backed WebUI evidence-usability slice. It does not claim full OpenCode parity, production readiness, or same-head acceptance for the new head until CI/Build/WebUI workflows complete on the exact latest commit and artifacts/screenshots are inspected.
