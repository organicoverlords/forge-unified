# WebUI typed tool card renderer proof

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`
Baseline selected head before slice: `208f24329f361e87d82842129ed83b469df36ca5`

## Selection basis

- Target URL branch remained the active work: `mvp/nim-freellmapi-router-20260626`.
- PR #3 remained open, non-draft, mergeable, and no newer open PR superseded it.
- Same-head baseline proof on `208f24329f361e87d82842129ed83b469df36ca5` was green across CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof.
- Live WebUI artifact inspected/listed: `7968597454` (`live-webui-feature-sprint-proof`) from run `28413421182`.

## Source backing used

Developer reference only; no Forge runtime branding claim:

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - `Part` branches typed rendering by part/tool kind.
  - `ReadTool`, `WriteTool`, `EditTool`, `BashTool`, `GlobTool`, `GrepTool`, `TaskTool`, and `FallbackTool` expose action-specific titles, targets, outputs, and error/result handling.
  - `ResultsButton` exposes hide/show result behavior.
  - `ToolFooter` records duration visibility for slower tool work.
- `anomalyco/opencode:packages/web/src/components/share/part.module.css`
  - Used as UI-structure reference for stable part/tool component markers and collapsed result disclosure.

## Forge slice implemented

- Updated `crates/webui/src/chat_ui_enhancements.html` with a browser-side typed tool renderer.
- The renderer augments existing tool cards with:
  - `typed-tool-renderer`
  - `tool-target-visible`
  - `tool-result-toggle`
  - `tool-duration-visible`
  - action-specific readable labels: `Read preview`, `Write target`, `Diff target`, `Command output`, `Matched paths`, `Matched lines`
  - `show result` / `hide result` and `copy result` controls
- Updated `scripts/smoke/check-webui-proof-part-contract.py` to gate these browser-visible proof markers and the exact OpenCode source anchors.

## Claim boundary

This is a source-backed WebUI tool-card UX slice. It does not claim full OpenCode parity, production readiness, or same-head live proof after this new commit until workflows complete on the latest head and artifacts/screenshots are inspected.

Required proof trail tokens retained: typed tool cards, tool targets, result toggles, session turn, assistant parts, changed files, stable session receipts, timeline action groups, file diff summary, collapsed technical details, proof-final.
