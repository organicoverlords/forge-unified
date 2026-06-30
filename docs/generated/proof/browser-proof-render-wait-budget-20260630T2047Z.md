# Browser proof render wait budget — 2026-06-30T20:47Z

## Selection basis

- Source-of-truth target branch: `mvp/nim-freellmapi-router-20260626`.
- Selected PR: #3, `mvp router slice`, open, non-draft, mergeable, base `master`.
- Previous exact head inspected: `2045bb9c42fa31e1eba1c2fc0e11216d0f70fb3d`.

## Failed workflow evidence inspected

- CI run `28473962799`: success on `2045bb9c42fa31e1eba1c2fc0e11216d0f70fb3d`.
- Build Proof run `28473962850`: success on `2045bb9c42fa31e1eba1c2fc0e11216d0f70fb3d`.
- Live WebUI Feature Sprint run `28473962831`, job `84393631052`: failed after `browser proof tool lifecycle`; artifact `7992869759` uploaded. The log showed the workflow reached NIM conversation creation, model chat stream, tool lifecycle conversation, then failed during browser proof / downstream full-benchmark validation.
- Fast WebUI Proof run `28473962844`, job `84393670263`: failed after `capture readable browser proof`; artifact `7992885875` uploaded. The elapsed log span from capture start to failure matched the old 16s Forge browser-proof process budget.

## OpenCode source backing

Reference repository: `anomalyco/opencode`, default branch `dev`.

Exact source paths used:

- `packages/session-ui/src/components/session-turn.tsx`
  - `requestAnimationFrame` delayed diff rendering around `shown` state.
  - `turnDurationMs` and readable session-turn proof surface.
  - visible error-card rendering for proofable failure state.
- `packages/session-ui/src/components/message-part.tsx`
- `packages/session-ui/src/components/basic-tool.tsx`
- `packages/session-ui/src/components/tool-count-summary.tsx`

Reasoning: Forge's browser proof must capture the rendered WebUI state after session-turn and tool surfaces have had enough time to become visible. The observed failure was not model routing or compile failure; it was the browser capture budget ending before a readable PNG proof was written.

## Feature slice built

Implemented in `crates/engine/src/tool/browser.rs`:

- Added explicit screenshot budget constants:
  - `SCREENSHOT_CHROME_TIMEOUT_MS=30000`
  - `SCREENSHOT_VIRTUAL_TIME_BUDGET_MS=15000`
  - `SCREENSHOT_BROWSER_TIMEOUT_SECONDS=45`
- Added explicit DOM budget constants:
  - `DOM_CHROME_TIMEOUT_MS=12000`
  - `DOM_VIRTUAL_TIME_BUDGET_MS=5000`
  - `DOM_BROWSER_TIMEOUT_SECONDS=18`
- Replaced the old hardcoded Chrome `--timeout=10000`, virtual-time budget `5000`, and Forge process timeout `16s` values.
- Exposed the new timing values in browser-proof success/failure metadata so proof artifacts explain the capture budget used.
- Preserved existing Chrome hardening: DBus env scrubbing, isolated profile, `--headless=chrome`, `--no-sandbox`, `--disable-dev-shm-usage`, `--run-all-compositor-stages-before-draw`, and PNG signature validation.

## Files changed

- `crates/engine/src/tool/browser.rs`
- `PROJECT_STATE.md`
- `docs/generated/proof/browser-proof-render-wait-budget-20260630T2047Z.md`

## Claim boundary

This is a targeted browser-proof capture reliability slice. It does not claim OpenCode parity, production readiness, or same-head WebUI proof until the exact new head completes all required workflows and the screenshot artifacts are inspected.
