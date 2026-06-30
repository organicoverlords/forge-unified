# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Current selected baseline before this slice: `a9be8be583ac1d5c81b675659fc413c6d1d07bbe`
- Latest same-head green proof before this slice: all required workflows passed on `484189c945d7b0ec90a70300ef960e868ed9a477`.
- Latest implementation/proof slice: browser session turn receipt grouping in `crates/webui/src/chat_ui_enhancements.html`, loaded by `crates/webui/src/chat_ui.rs`, with deterministic WebUI and Fast WebUI proof gates updated.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_enhancements.html` as a reviewable browser enhancement bundle.
- Updated `crates/webui/src/chat_ui.rs` to load both browser UI files with `concat!(include_str!(...))`.
- Added visible per-turn receipt grouping: `Receipts grouped by turn`, file receipt count, tool card count, and status chip count.
- Added Session timeline actions: `copy timeline`, `retry latest prompt`, and `copy latest files`.
- Added per-turn receipt actions: `copy receipts` and `copy files`.
- Updated `scripts/smoke/check-webui-proof-part-contract.py` to validate the enhancement bundle and proof trail tokens.
- Updated `scripts/smoke/fast-webui-proof.sh` to require the new static and browser-captured proof markers.
- Added proof doc `docs/generated/proof/session-turn-receipt-groups-20260630T0058Z.md`.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `timeline-file-diff-groups`, `timeline-action-groups`, `turn-receipt-toolbar`, `file-diff-summary-visible`, and `stable-session-receipts`.
- This slice improves browser proof readability. It does not claim full parity, production readiness, or backend fork/revert/session checkpoint semantics.

## Developer source anchors retained

- Browser session-turn grouping, message/assistant part rendering, basic tool-card rendering, and grouped tool-count summary behavior are the upstream UI concepts used as reference material.
- Forge implementation paths under guard now include `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, and `crates/webui/src/chat_ui_enhancements.html`.
