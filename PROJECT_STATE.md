# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current implementation head before this note: `65792222e44aeba501c0e72bb8bbf8dcb2769110`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this slice: `b9006635e25b4b397c1c7e968a25b5715dd2cc1f` had Build Proof success; CI, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof were red.
- Latest implementation slice: filterable WebUI action digest summaries. Long tool runs now expose digest filters for `all`, `completed`, `needs attention`, `running`, and `pending`; matching cards remain visible; nonmatching cards are hidden from the current scan view; the digest shows `showing visible/total actions`; `copy action digest` exports the active filter view.
- Do not claim this head is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Updated `crates/webui/src/chat_ui_action_summaries.html` as an executable WebUI product slice, not documentation only.
- Added action-digest filter buttons with `aria-pressed` state: `all`, `completed`, `needs attention`, `running`, and `pending`.
- Added per-card filtering via `data-action-filter-hidden` so long runs can be scanned by outcome without raw JSON hunting.
- Added a visible `showing N/M actions` count and made `copy action digest` respect the active filter.

## Source-backed contracts

- Action digest source anchors: `packages/session-ui/src/components/tool-count-summary.tsx` and `packages/session-ui/src/components/session-turn.tsx`.
- Forge implementation paths: `crates/webui/src/chat_ui_action_summaries.html`, `scripts/smoke/check-webui-proof-part-contract.py`, and browser proof scripts.
- Required action digest tokens: `human action summaries`, `action digest summary`, `count summary`, `action-digest-filter`, `action-digest-filter-all`, `action-digest-filter-ok`, `action-digest-filter-error`, `action-digest-filter-running`, `action-digest-filter-pending`, `action-digest-visible-count`, `aria-pressed`, and active-filter `copy action digest` output.
- Formatter runtime gap remains explicit: Forge must not claim completed runtime formatter behavior for config-aware or dependency-aware formatting until runtime probes exist.
- Browser proof gap remains explicit: repeated red runs show Chrome screenshot capture failures after NIM/WebUI work completed; same-head screenshot proof is still required before acceptance.
