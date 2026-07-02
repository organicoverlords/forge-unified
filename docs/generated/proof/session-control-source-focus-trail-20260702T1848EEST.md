# Session-control source focus trail proof

Date: 2026-07-02 18:48 EEST

## Selection

- Repo: `organicoverlords/forge-unified`
- Source URL branch: `mvp/nim-freellmapi-router-20260626`
- Selected PR: #3, open, non-draft, mergeable
- Previous selected head: `7fde684aa832215ed8c394f8777493ff0b21db59`

## Workflow state inspected before this slice

Same-head workflow state for `7fde684aa832215ed8c394f8777493ff0b21db59`:

- CI `28599198550`: success
- Build Proof `28599198852`: success
- Fast WebUI Proof `28599198562`: success
- Live WebUI Feature Sprint `28599198824`: failure
- App Build Proof `28599198617`: success
- App Multistep Build Proof `28599198861`: success

Live WebUI failure inspected:

- Run: `28599198824`
- Job: `84802576143`
- Provider/model: NVIDIA NIM `deepseek-ai/deepseek-v4-flash`
- Result: WebUI prompt path compiled and ran with tool calls/results and browser proof, but the quality gate failed on final test-name reporting.
- Failed jobs were retried for run `28599198824` before this slice.

## Source backing

Used upstream source path:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Source-backed behavior adapted:

- Sticky expanded context around a focused turn/diff item.
- Trigger affordance for focused interaction.
- Path and meta slots kept visible near the focused item.
- Deferred detail rendering pattern preserved from the diff detail path.

## Forge slice built

Built: WebUI session-control source focus trail.

Implementation paths:

- Added `crates/webui/src/chat_ui_session_control_source_focus_trail.html`.
- Updated `crates/webui/src/chat_ui.rs` to include the helper.

Behavior:

- When a selected source-map receipt is active, the WebUI renders a sticky source-focus trail above the session-control ledger.
- The trail shows source, action, and focused receipt id/sequence, preserving context after `jump to receipt source` scrolls away from the selected panel.
- The trail adds `return to selected receipt`, which scrolls and focuses the selected receipt panel.
- Existing source map, source jump, filtering, selected receipt, copy, and lazy-detail behavior is preserved.

Proof markers added:

- `session-control-source-focus-trail`
- `session-control-source-focus-return`
- `opencode-session-turn-sticky-accordion-header-shape`
- `opencode-session-turn-diff-path-shape`
- `opencode-session-turn-diff-meta-shape`
- `crates/webui/src/chat_ui_session_control_source_focus_trail.html`

## Claim boundary

This source-backed slice is committed, but the new head is not same-head browser/NIM proven until exact-head workflows complete and the Live WebUI artifact/screenshots are inspected.
