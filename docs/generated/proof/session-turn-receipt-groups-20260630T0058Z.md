# Session turn receipt grouping proof gate

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`

## Slice

Forge WebUI now has a separate reviewable browser enhancement bundle at:

- `crates/webui/src/chat_ui_enhancements.html`

`crates/webui/src/chat_ui.rs` loads it beside the main browser UI via `include_str!("chat_ui_enhancements.html")` so this proof UX is not hidden inside a giant Rust raw string.

## User-visible behavior

The browser proof UI adds turn receipt grouping and stable session receipts:

- `timeline-file-diff-groups`
- `timeline-action-groups`
- `turn-receipt-toolbar`
- `file-diff-summary-visible`
- `stable-session-receipts`
- visible text: `Receipts grouped by turn`
- visible actions: `copy timeline`, `retry latest prompt`, `copy latest files`, `copy receipts`, `copy files`

Each enhanced session turn summarizes visible evidence already present in the browser:

- file receipts
- tool cards
- status chips

The right-side Session timeline gets copy/retry actions that operate on visible turn/file/tool evidence. The enhancement does not claim backend fork/revert semantics; those remain a later backend slice.

## Source backing

OpenCode source anchors retained as developer-only reference material:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx` — session turn grouping and changed-file grouping.
- `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx` — assistant/message part rendering and copy affordances.
- `anomalyco/opencode:packages/session-ui/src/components/basic-tool.tsx` — tool card trigger/status/collapsed technical details.
- `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx` — grouped tool activity/count summary behavior.

## Deterministic guards

Updated deterministic guards:

- `scripts/smoke/check-webui-proof-part-contract.py` now validates `chat_ui_enhancements.html`, required receipt grouping hooks, timeline action hooks, and proof trail tokens: turn receipt grouping, stable session receipts, timeline action groups, and file diff summary.
- `scripts/smoke/fast-webui-proof.sh` now checks static and browser-captured proof markers for receipt groups and timeline actions.

## Claim boundary

This is a browser UI/proof readability slice. It improves real visible proof review but does not claim full OpenCode parity, production readiness, backend fork/revert, or complete session checkpoint semantics.
