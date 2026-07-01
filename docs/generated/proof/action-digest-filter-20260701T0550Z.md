# WebUI action digest filter proof — 2026-07-01 05:50 EEST

## Selection basis

- Source-of-truth branch: `mvp/nim-freellmapi-router-20260626`.
- Live PR state before the slice: PR #3 open, non-draft, mergeable.
- Inspected head before the slice: `b9006635e25b4b397c1c7e968a25b5715dd2cc1f`.
- Build Proof was green on that head; CI, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof were red.

## Failed-run diagnosis

- CI run `28488018048`, Smoke Test job `84438549893`, failed in `Validate WebUI proof harness`.
- Rust check, tests, clippy, build, audit, deny, and file-size gate jobs were green.
- Smoke failure cause: deterministic state/proof wording guard detected formatter-runtime overclaim wording in `PROJECT_STATE.md`.
- Browser proof workflows still need same-head artifact inspection; earlier red runs show Chrome screenshot capture failure after NIM/WebUI work completed.

## Implementation slice

Implemented filterable action digest summaries for long WebUI tool runs:

- `all`
- `completed`
- `needs attention`
- `running`
- `pending`

Behavior:

- action cards get `data-action-filter-hidden="true"` when excluded by the active filter;
- filter buttons expose `aria-pressed`;
- digest displays `showing N/M actions`;
- `copy action digest` exports the active filtered view.

## OpenCode source backing

Exact upstream source paths inspected and used as behavior anchors:

- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - Used for count-summary semantics: only active/nonzero items should be prominent, with explicit empty/fallback behavior.
- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Used for visible/hidden session part behavior and readable session-level proof surface.

## Forge paths changed

- `crates/webui/src/chat_ui_action_summaries.html`
- `PROJECT_STATE.md`
- `docs/generated/proof/action-digest-filter-20260701T0550Z.md`

## Claim boundary

This is a real WebUI product slice, not docs-only. It does not prove latest-head acceptance. Same-head workflows and browser screenshot artifacts must pass and be inspected before claiming parity or readiness.
