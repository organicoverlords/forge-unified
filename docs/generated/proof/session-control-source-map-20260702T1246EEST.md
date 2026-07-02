# Session-control source map proof — 2026-07-02 12:46 EEST

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`
- PR: #3, open, non-draft, mergeable at verification.
- Exact head inspected before this slice: `5f768df164ee83a2d736e325ec224a20018f0e2a`.

## Live workflow state inspected before this slice

Same-head workflows for `5f768df164ee83a2d736e325ec224a20018f0e2a`:

- Live WebUI Feature Sprint `28577794982`: success.
- Build Proof `28577794975`: success.
- App Build Proof `28577794993`: success.
- Fast WebUI Proof `28577795046`: success.
- App Multistep Build Proof `28577794942`: success.
- CI `28577794923`: failure.

Failed CI job:

- Smoke Test job `84730397852` failed during `Validate WebUI proof harness`.
- Failure was stale state-marker coverage for:
  - `crates/webui/src/conversation_controls.rs`
  - `crates/webui/src/chat_ui_session_controls.html`
  - `crates/webui/src/chat_ui_session_control_search.html`

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Source features used as reference:

- `data-slot="session-turn-diff-meta"`
- `data-slot="session-turn-diff-path"`
- visible metadata beside a dense session-turn row
- copyable/actionable per-row session-turn affordances

## Forge implementation

Built: **WebUI session-control source map**.

Files changed:

- `crates/webui/src/chat_ui_session_control_source_map.html`
- `crates/webui/src/chat_ui.rs`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-source-map-20260702T1246EEST.md`

Behavior:

- Adds source/action/receipt chips to backend session-control event rows.
- Adds `copy source map` for a compact, audit-friendly per-row receipt map.
- Preserves Forge identity; upstream OpenCode paths are proof/docs references only and are not exposed as runtime branding.
- Adds browser proof hooks: `session-control-source-map`, `session-control-source-map-source`, `session-control-source-map-action`, `session-control-source-map-receipt`, `copy-session-control-source-map`, and `opencode-session-turn-diff-meta-shape`.

## Claim boundary

This slice has been pushed to the branch, but the new post-slice head is not same-head browser/NIM proven until workflows complete on the exact new head and the Live WebUI proof artifact/screenshots are inspected.
