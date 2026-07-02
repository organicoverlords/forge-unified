# WebUI tool lifecycle event ledger proof — 2026-06-30T09:45Z

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626` from `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected work remains PR #3 because it is the active open, non-draft, mergeable PR for the requested branch and no newer open PR superseded it during this check.
- Baseline before this slice: `d97a1851189bd243afc2c499e6a03a6293b7ee5c`.
- Same-head baseline proof before this slice:
  - CI `28432259806`: success.
  - Build Proof `28432259855`: success.
  - App Build Proof `28432259822`: success.
  - App Multistep Build Proof `28432259808`: success.
  - Fast WebUI Proof `28432259807`: success.
  - Live WebUI Feature Sprint `28432259837`: success.
  - Live WebUI artifact: `7975940477` / `live-webui-feature-sprint-proof`, created 2026-06-30T09:08:58Z.

## Feature slice

Forge WebUI tool lifecycle cards already showed stable receipt ids, status/timing, and copyable anchors. This slice adds a structured lifecycle event ledger per visible tool card:

- Each lifecycle strip now stores `data-tool-status`, `data-tool-target`, and `data-tool-event`.
- The event payload is JSON with `type`, `receipt`, `status`, `target`, `duration`, and `index`.
- The visible strip shows the JSON event in a compact event pill for screenshot/DOM proof.
- A new `copy tool event` control copies the exact event payload.
- Each enhanced tool dispatches a bubbling `forge:tool-lifecycle` `CustomEvent` with the same receipt/status/target/duration/index details.

This is a functional WebUI semantics slice: browser proof can now inspect or copy the exact lifecycle event for a tool card instead of relying only on visual text.

## OpenCode source backing

Used as developer reference only; no Forge runtime branding is exposed.

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - Source anchor for ToolPart lifecycle transitions and durable tool events.
- `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - Source anchor for typed pending/running/completed/error tool states.
- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - Source anchor for rendering tool state/result evidence into visible session parts.

## Forge files changed

- `crates/webui/src/chat_ui_tool_lifecycle.html`
- `PROJECT_STATE.md`
- `docs/generated/proof/webui-tool-lifecycle-event-ledger-20260630T0945Z.md`

## Claim boundary

This is a source-backed WebUI lifecycle/event evidence slice. It does not claim full OpenCode parity, production readiness, or same-head acceptance for the new head until CI, Build Proof, App Build Proof, App Multistep Build Proof, Fast WebUI Proof, and Live WebUI Feature Sprint complete on the exact new head and artifacts/screenshots are inspected.
