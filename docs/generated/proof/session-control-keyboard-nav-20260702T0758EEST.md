# Proof — session control keyboard navigation

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`

## Live selection basis

- Source of truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected PR: #3, `mvp router slice`, because it remains open, non-draft, mergeable, and is the current meaningful app-change branch for this work.
- Previous same-head checked: `74c8d18e93fdbe9d147ada948241754efc65a498`.

## Previous same-head workflow status

- Build Proof `28563994299`: success.
- App Multistep Build Proof `28563994337`: success.
- Fast WebUI Proof `28563994321`: success.
- App Build Proof `28563994303`: success.
- CI `28563994308`: failure.
- Live WebUI Feature Sprint `28563994300`: failure.

## Failures inspected

- CI Smoke Test job `84687418579` failed in `Validate WebUI proof harness` because `PROJECT_STATE.md` no longer preserved the session-control proof markers required by the smoke harness.
- Live WebUI Feature Sprint job `84687418580` failed after NVIDIA NIM/browser proof progress. The log reported `provider: nvidia_nim`, `model: deepseek-ai/deepseek-v4-flash`, browser proof passed, and final quality failed on omitted final answer/report content.
- Live artifact inspected: `8028743238` (`live-webui-feature-sprint-proof`).

## OpenCode source backing

Exact upstream paths used as reference:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `data-slot="session-turn-content"`
  - `data-slot="session-turn-message-container"`
  - `autoScroll.handleInteraction`
  - `showAll`, `toggleAll`, `visible`, and overflow behavior

The Forge implementation keeps Forge independent and does not expose OpenCode branding.

## Feature built

Added WebUI keyboard navigation for the session-control event ledger:

- `j` / `ArrowDown`: move focused event down.
- `k` / `ArrowUp`: move focused event up.
- `e`: jump to next error event.
- `Enter`: open/hide the focused event.
- `c`: copy the focused event receipt.
- `/`: focus the session-control ledger search box.

Proof hooks added:

- `session-control-keyboard-nav`
- `session-control-keyboard-focus`
- `session-control-copy-focused-event`
- `session-control-next-error`
- `session-control-focus-search`
- `session-control-open-focused-event`
- `opencode-session-turn-keyboard-interaction`

## Files changed

- `crates/webui/src/chat_ui_session_control_keyboard.html`
- `crates/webui/src/chat_ui.rs`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-keyboard-nav-20260702T0758EEST.md`

## Claim boundary

This is a source-backed app-code parity slice. It is not same-head browser/NIM proven until the workflow set completes on the final pushed head and artifacts/screenshots are inspected.
