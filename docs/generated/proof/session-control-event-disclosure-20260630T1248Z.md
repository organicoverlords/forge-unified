# Session-control event disclosure proof — 2026-06-30T12:48Z

## Selection

- Repository: `organicoverlords/forge-unified`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open, non-draft, mergeable into `master`.
- Baseline before this slice: `b1efb91cbf1bcb4522fcf9543660f4cbaff21311`.
- Baseline same-head proof: CI `28442226731`, Build Proof `28442226780`, Fast WebUI Proof `28442226796`, App Build Proof `28442226776`, App Multistep Build Proof `28442226736`, and Live WebUI Feature Sprint `28442226849` passed on the baseline head.
- Baseline Live WebUI artifact: `7979896103`.
- Baseline Fast WebUI artifact: `7979638101`.

## Slice built

Implemented Forge WebUI session-control event disclosure for backend-backed session control receipts.

- Each session-control ledger row now includes a `show event` / `hide event` disclosure.
- The disclosure exposes visible JSON detail for the exact Forge-local control receipt.
- Disclosure buttons carry `aria-expanded` and `aria-controls`.
- The existing `copy event`, `copy session receipt`, `data-session-control-event`, and `forge:session-control` behavior remains intact.
- Runtime UI continues to avoid embedding upstream OpenCode source paths in browser payloads.

## OpenCode source backing

Exact upstream reference paths inspected:

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
  - `ResultsButton` show/hide disclosure pattern.
  - `ReadTool`, `WriteTool`, `TaskTool`, and `FallbackTool` result/detail panes.
  - Tool-title/target/result grouping.
  - Flattened args and visible copy/detail behavior used as the reference shape for Forge proof controls.
- `packages/session-ui/src/components/session-turn.tsx`
  - Existing durable proof anchor for session action/status/retry semantics retained from the prior backend session-control slices.

## Forge paths changed

- `crates/webui/src/chat_ui_session_controls.html`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-event-disclosure-20260630T1248Z.md`

## Proof status

- Deterministic contract gate updated to require:
  - `session-control-event-disclosure`
  - `backend-session-control-event-detail`
  - `show-session-control-event`
  - `aria-expanded`
  - `aria-controls`
- Same-head GitHub workflow proof is required for the final commit containing this slice.
- Do not claim parity or same-head WebUI proof until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete successfully on the final head and artifacts/screenshots are inspected.

## Browser proof prompt requirement

The live WebUI proof must use NVIDIA NIM only and a natural-language prompt through the browser UI. Required visible evidence:

- Forge-branded WebUI.
- `nvidia_nim` provider route and model route.
- Backend session controls.
- Session-control event ledger.
- Session-control event disclosure/detail pane.
- Copyable event receipt.

## Claim boundary

This slice is a meaningful OpenCode-backed UI behavior parity increment for visible/copyable disclosure of session-control event receipts. It does not prove complete OpenCode parity or production readiness.
