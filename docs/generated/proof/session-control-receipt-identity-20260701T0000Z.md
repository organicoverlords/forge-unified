# Session-control receipt identity proof — 2026-07-01

## Selection

- Repository: `organicoverlords/forge-unified`
- Source-of-truth branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open and non-draft during inspection
- Prior inspected head: `a9a5ddbb49214e175defcae66b3eaac24e58686c`

No newer open PR or recently pushed non-main branch was selected as superseding the target branch.

## Inspected workflow state before this slice

- CI `28475175361`: completed success on `a9a5ddbb49214e175defcae66b3eaac24e58686c`.
- Build Proof `28475175539`: completed success on `a9a5ddbb49214e175defcae66b3eaac24e58686c`.
- Fast WebUI Proof `28475175423`: completed failure; job `84397849907` reached NIM/WebUI streaming and failed at `capture readable browser proof`; artifact `7993376609` was uploaded.
- Live WebUI Feature Sprint `28475175414`: completed failure; job `84397849945` reached NIM model chat stream and browser proof tool lifecycle, then downstream proof check failed with missing full benchmark conversation or stream artifacts; artifact `7993403833` was uploaded.
- App Build Proof `28475175330`: completed failure.
- App Multistep Build Proof `28475175462`: completed failure.

## OpenCode source backing

Exact upstream source paths inspected/used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Stable session and message identity: `sessionID`, `messageID`, `Binary.search(messages, props.messageID, ...)`.
  - Assistant copy target selection through stable part IDs.
  - Visible session-turn status/action surface, turn duration, error unwrap behavior, and proof-visible session UI.
- Existing retained source-backed anchors in this workstream:
  - `anomalyco/opencode:packages/session-ui/src/components/tool-count-summary.tsx`
  - `anomalyco/opencode:packages/session-ui/src/components/message-part.tsx`
  - `anomalyco/opencode:packages/session-ui/src/components/basic-tool.tsx`
  - `anomalyco/opencode:packages/web/src/components/share/part.tsx`

## Feature built

Implemented stable, Forge-local backend session-control receipt identity:

- Added `SESSION_CONTROL_RECEIPT_SEQUENCE` in `crates/webui/src/conversation_controls.rs`.
- Each `control_receipt(...)` now adds:
  - `receipt_id`: `session-control-<conversation_id>-<sequence>`
  - `sequence`: monotonic process-local sequence
  - `source`: `forge.webui.session_controls`
- Preserved existing receipt fields:
  - `type: forge.session_control`
  - `action`
  - `conversation_id`
  - `status`
  - `backend_backed`
  - `created_at`
  - `payload`

This is runtime metadata used by the WebUI copy/event ledger path. It keeps upstream OpenCode paths in docs/proof only, not browser runtime payloads.

## Forge files changed

- `crates/webui/src/conversation_controls.rs`
- `scripts/smoke/check-session-controls-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/session-control-receipt-identity-20260701T0000Z.md`

## Deterministic proof gate updates

Updated `scripts/smoke/check-session-controls-contract.py` to require:

- `SESSION_CONTROL_RECEIPT_SEQUENCE`
- `receipt_id`
- `sequence`
- `forge.webui.session_controls`
- state-file trail tokens for stable session-control receipt identity

## Browser proof boundary

This run did not produce a new passing browser screenshot artifact for the final head. Previous same-head WebUI proof for `a9a5ddbb49214e175defcae66b3eaac24e58686c` failed in browser capture. Therefore this proof does not claim latest-head WebUI screenshot proof or full OpenCode parity.

## Claim boundary

Claimed:

- Source-backed session-control receipt identity slice implemented in runtime WebUI backend code.
- Workflow failures inspected and recorded.
- Proof/state updated.

Not claimed:

- Latest-head same-head workflow pass.
- Latest-head WebUI screenshot proof.
- Complete OpenCode parity or production readiness.
