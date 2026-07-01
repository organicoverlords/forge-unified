# Action digest pinned-actions proof

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection basis

The target branch remains the selected work branch. PR #3 is open, non-draft, and mergeable. Live state before the pinned-rail slice showed head `9ae7a8f70abaaba13195ee0fca7a5a67b2ce20f1` with Build Proof, App Build Proof, and App Multistep Build Proof passing; CI, Fast WebUI Proof, and Live WebUI Feature Sprint were red.

## Failed workflow inspection

CI run `28499337013`, Smoke Test job `84472706650`, failed in `Validate WebUI proof harness`. Rust check, clippy/build, tests, doc tests, deny, file-size, and security audit jobs passed. The failing proof-harness check was the stale state phrase `state:session-control ledger search`.

Fast WebUI Proof run `28499337002` and Live WebUI Feature Sprint run `28499337009` were also red before this slice. The current proof boundary remains that same-head browser screenshot PNG proof is not accepted until a later exact-head artifact is inspected.

## Source-backed OpenCode anchors

- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - Used for copy/action button semantics via `MessageActionButton` and `writeClipboard` fallback behavior.
- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Used for compact visible/overflow turn-surface behavior via `MAX_FILES`, `showAll`, `overflow`, and `visible`.
- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - Used for count-list and visible/fallback summary behavior through non-zero count rendering and stable summary slots.

## Forge slice built

Implemented and extended WebUI action digest pinned-action controls:

- New bundled file: `crates/webui/src/chat_ui_action_pins.html`.
- Each generated human action summary can be pinned/unpinned with a visible `pin action` / `unpin action` control.
- Pinned cards expose `data-action-pinned="true"` and a visible highlighted state.
- The action digest shows a live pinned count, `N pinned actions`.
- The action digest exposes `copy pinned actions`, which copies pinned actions first and falls back to currently visible actions if none are pinned.
- Added a pinned action rail above the action list so pinned actions remain visible without scrolling the full digest.
- Rail items scroll/focus back to the source action card.
- The rail shows the first four pinned actions and reports overflow with `pinned-action-rail-overflow`.
- Proof tokens include `action-digest-pin-visible`, `pinned-action-summary`, `copy-pinned-actions`, `action-pin-count`, `message-action-button-parity`, `pinned-action-rail`, `pinned-action-rail-list`, and `pinned-action-rail-overflow`.

## Files changed

- `crates/webui/src/chat_ui_action_pins.html`
- `crates/webui/src/chat_ui.rs`
- `PROJECT_STATE.md`
- `docs/generated/proof/action-digest-pinned-actions-20260701T0950Z.md`

## Proof boundary

This commit is not same-head proven yet. GitHub Actions need to run on the new final head, and WebUI browser screenshot proof must be inspected before claiming latest-head parity or screenshot proof.
