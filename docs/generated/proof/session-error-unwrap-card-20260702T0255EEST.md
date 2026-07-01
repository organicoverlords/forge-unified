# Session error unwrap card — 2026-07-02 02:55 EEST

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- Selection basis: target URL points to this branch; open PR inspection showed PR #3 remains the current meaningful app-change PR and no newer open PR supersedes it.
- Baseline same-head proven before this slice: `77fc4e0be1a96e1bd3ebd51df83bcad961769198`

## Live workflow baseline inspected

Same-head workflows for `77fc4e0be1a96e1bd3ebd51df83bcad961769198`:

- Live WebUI Feature Sprint: `28552966164` — success
- CI: `28552966090` — success
- Build Proof: `28552966115` — success
- Fast WebUI Proof: `28552966159` — success
- App Build Proof: `28552966093` — success
- App Multistep Build Proof: `28552966101` — success

Live WebUI artifact inspected:

- `8024766752` / `live-webui-feature-sprint-proof`
- Job: `84654156102`
- Live steps succeeded, including natural feature-build prompt, full benchmark evidence/quality check, and proof upload.

## Feature built

Added a browser-visible session error unwrap card:

- New file: `crates/webui/src/chat_ui_error_unwrap.html`
- Loader update: `crates/webui/src/chat_ui.rs`

Behavior:

- Listens for `forge:session-control` error receipts.
- Extracts nested provider/API error text from raw strings, double-encoded JSON strings, `{ error: { type, message, code } }`, `{ message }`, and `{ error: string }` shapes.
- Renders a readable error card in the session header while preserving raw session-control receipt/event JSON elsewhere.
- Adds `copy readable error` so the user can copy the cleaned failure text without digging through raw JSON.

This is an app/runtime UI feature, not docs-only or cosmetic-only.

## OpenCode source backing

Upstream paths used exactly:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `unwrap()` JSON/provider error extraction logic.
  - Error card rendering under `data-slot="session-turn-diffs"` / assistant turn surface region.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - Existing action/copy affordance style already used by Forge's session controls.

Forge keeps independent branding and implementation; the source paths are recorded only as reference anchors.

## Proof markers added

- `session-error-unwrap-card`
- `opencode-error-unwrap-parity`
- `opencode-error-card-parity`
- `readable-session-control-error-card`
- `latest-session-control-error-unwrapped`
- `copy-session-control-error-unwrapped`
- `packages/session-ui/src/components/session-turn.tsx`

## Claim boundary

Do not claim same-head proof for the post-slice head until the GitHub Actions workflows for that exact head finish and artifacts/screenshots are inspected. The baseline head `77fc4e0...` is proven; the new implementation head must prove itself independently.
