# WebUI tool lifecycle overlay proof note

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`
Baseline before slice: `9e0def730d21564ad098fb9da111b03f6ed59cde`

## Selection basis

The provided target branch remains PR #3. The PR is open, non-draft, and mergeable. No newer open PR superseded it for the active WebUI/NIM proof work.

The baseline before this slice was same-head green on `9e0def730d21564ad098fb9da111b03f6ed59cde`:

- CI `28416964487`: success
- Build Proof `28416964489`: success
- Live WebUI Feature Sprint `28416964485`: success
- Fast WebUI Proof `28416964498`: success
- App Multistep Build Proof `28416964511`: success
- App Build Proof `28416964537`: success
- Live WebUI artifact `7969964940` (`live-webui-feature-sprint-proof`)

## Feature slice

Implemented a browser-side WebUI tool lifecycle overlay:

- new `crates/webui/src/chat_ui_tool_lifecycle.html`
- included from `crates/webui/src/chat_ui.rs`
- visible `tool-lifecycle-strip` proof parts on tool cards
- `pending`, `running`, `completed`, and `error` state timeline pills
- target and duration extraction from tool state metadata
- `copy tool link` and `copy status` actions for exact browser-proof references
- updated `scripts/smoke/check-webui-proof-part-contract.py` so CI requires these markers

## OpenCode source backing

OpenCode source paths used as developer-only references:

- `packages/web/src/components/share/part.tsx` — part anchors, completed/error tool part rendering, tool target/result components, and duration footer behavior.
- `packages/web/src/components/share/part.module.css` — component/disclosure structure reference.
- `packages/session-ui/src/components/session-turn.tsx`
- `packages/session-ui/src/components/message-part.tsx`
- `packages/session-ui/src/components/basic-tool.tsx`
- `packages/session-ui/src/components/tool-count-summary.tsx`

## Guard tokens retained

Required proof trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer.

## Claim boundary

This is a source-backed WebUI tool-card lifecycle/readability slice. It does not claim complete OpenCode parity, production readiness, or same-head acceptance for the latest post-slice head. Same-head CI / Build Proof / Fast WebUI / Live WebUI / App proof workflows must run and artifacts/screenshots must be inspected before accepting the latest head.
