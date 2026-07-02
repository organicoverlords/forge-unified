# WebUI tool receipt id proof slice — 2026-06-30T04:52Z

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Baseline head before this slice: `a9e2fa3ed068da87cc3636f673ad33e6c8fa7a53`
- Baseline proof: same-head CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof were green on `a9e2fa3ed068da87cc3636f673ad33e6c8fa7a53`.
- Baseline Live WebUI proof artifact: `7970696828` (`live-webui-feature-sprint-proof`).

## OpenCode source backing used

The slice is source-backed by upstream OpenCode browser-share part rendering paths:

- `packages/web/src/components/share/part.tsx`
  - part ids/anchors and copied-message feedback
  - typed completed/error tool rendering
  - `ToolFooter` duration behavior
- `packages/web/src/components/share/part.module.css`
  - structured part/tool layout and component affordance reference

Reference only; Forge runtime/browser output must not expose upstream branding as user-visible proof text.

## Forge paths changed

- `crates/webui/src/chat_ui_tool_lifecycle.html`
- `PROJECT_STATE.md`
- `docs/generated/proof/webui-tool-receipt-id-20260630T0452Z.md`

## Feature built

The WebUI lifecycle overlay now emits stable browser-visible tool receipts so screenshots and copied proof can unambiguously identify the exact tool card being audited.

Implemented tokens and behavior:

- `tool-receipt-id-visible`
- `copy-tool-receipt`
- visible `.tool-life-receipt` chip
- `data-tool-receipt` on the lifecycle strip and parent tool card
- deterministic receipt from tool id/status/target/ordinal with a short checksum suffix
- `copy tool receipt` button copying receipt plus status/target

## Proof boundary

This is a meaningful app/UI proofability slice, not a docs-only or cosmetic-only change. It improves screenshot auditability of real WebUI/NIM proof artifacts.

Do not claim this new head is same-head proven until fresh CI / Build Proof / Fast WebUI Proof / Live WebUI Feature Sprint / App Build Proof / App Multistep Build Proof runs complete on the exact post-slice head and the WebUI artifact is inspected.
