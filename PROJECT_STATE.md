# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous selected head `2045bb9c42fa31e1eba1c2fc0e11216d0f70fb3d` had CI and Build Proof green but Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof failed in browser proof capture / downstream missing full-benchmark artifacts.
- Inspected failed Live WebUI Feature Sprint run `28473962831`, job `84393631052`; artifact `7992869759` was uploaded but the browser proof step exited before full benchmark artifacts could be validated.
- Inspected failed Fast WebUI Proof run `28473962844`, job `84393670263`; it reached NIM/WebUI streaming and failed exactly at `capture readable browser proof` after the 16s capture path.
- Latest implementation/proof slice: browser proof rendered-wait budget, extending Chrome screenshot timeout from the 10s/16s path to a 30s Chrome budget and 45s process budget with metadata, plus DOM capture budget constants.
- New proof doc: `docs/generated/proof/browser-proof-render-wait-budget-20260630T2047Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Hardened `crates/engine/src/tool/browser.rs` with runner-stable Chrome flags: `--no-sandbox`, `--disable-dev-shm-usage`, `--disable-background-networking`, `--disable-extensions`, `--disable-sync`, `--hide-scrollbars`, `--mute-audio`, and `--run-all-compositor-stages-before-draw`.
- Added PNG readability validation so a successful Chrome process must also produce a non-empty PNG screenshot.
- Added diagnosable browser proof failure output so screenshot failures return structured `BrowserProofResult { success: false, error, console_logs }` metadata instead of blank artifacts.
- Extended browser screenshot wait to match slower hosted-runner rendering: `SCREENSHOT_CHROME_TIMEOUT_MS=30000`, `SCREENSHOT_VIRTUAL_TIME_BUDGET_MS=15000`, and `SCREENSHOT_BROWSER_TIMEOUT_SECONDS=45`.
- Added DOM capture budget constants: `DOM_CHROME_TIMEOUT_MS=12000`, `DOM_VIRTUAL_TIME_BUDGET_MS=5000`, and `DOM_BROWSER_TIMEOUT_SECONDS=18`.
- Kept backend-backed session operations in `crates/engine/src/agent.rs`: `retry_source`, `fork_conversation`, `revert_last_turn`, and `session_control_receipt`.
- Kept backend route handlers in `crates/webui/src/conversation_controls.rs` so checkpoint, fork, revert latest turn, and retry source all return structured receipt payloads.
- Kept browser control bundle `crates/webui/src/chat_ui_session_controls.html` with visible receipt strip, `copy session receipt`, `copy all events`, `copy latest error`, `forge:session-control` events, event ledger, event rows, `data-session-control-event`, event copy, disclosure, count summary, status filters, diff summary, duration summary, overflow toggle, hidden older receipt row, session-control ledger export, and session-control error card.
- Updated `scripts/smoke/check-session-controls-contract.py` so the deterministic gate requires the latest session-control error card and copy affordance.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `backend-session-controls`, `backend-checkpoint-action`, `backend-fork-action`, `backend-revert-action`, `backend-retry-source-action`, `backend-session-control-status`, `backend-session-control-receipt`, `copy-session-control-receipt`, `session-control-ledger-export`, `copy-session-control-ledger`, `session-control-error-card`, `backend-session-control-error-card`, `copy-session-control-error`, `latest-session-control-error`, `copy latest error`, `session-control-event-ledger`, `backend-session-control-ledger`, `backend-session-control-event-row`, `copy-session-control-event`, `data-session-control-event`, `session-control-event-disclosure`, `backend-session-control-event-detail`, `show-session-control-event`, `aria-expanded`, `aria-controls`, `session-control-count-summary`, `backend-session-control-summary`, `backend-session-control-count`, `session-control-filter`, `session-control-filter-all`, `session-control-filter-ok`, `session-control-filter-error`, `aria-pressed`, `session-control-diff-summary`, `backend-session-control-diff-summary`, `backend-session-control-diff-chip`, `session-control-diff-before`, `session-control-diff-after`, `session-control-diff-removed`, `session-control-duration-summary`, `backend-session-control-duration-summary`, `backend-session-control-duration-chip`, `session-control-duration-ms`, `session-control-started-at`, `session-control-completed-at`, `session-control-ledger-overflow`, `backend-session-control-overflow-toggle`, `session-control-show-all`, `session-control-show-less`, `session-control-visible-count`, `session-control-hidden-overflow-row`, and `forge-local-control-receipt`.
- This slice fixes the browser-proof screenshot capture timeout path. It does not claim full parity, production readiness, or same-head acceptance until workflows finish.

## Compatibility proof trail retained for deterministic gates

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, lifecycle event ledger, copy tool event, tool event JSON, input toggle, diagnostic copy, target copy, preview pane, preview toggle, copy preview, accessible disclosure state, `aria-expanded`, `aria-controls`, backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, session control event ledger, copy session control event, session-control event disclosure, event detail disclosure, show event, session-control count summary, status filters, session-control diff summary, before/after/removed message chips, session-control duration summary, start/completion/duration chips, session-control ledger overflow, show older/show less overflow toggle, visible ledger count, hidden older receipt row, session-control ledger export, copy all events, session-control error card, latest session-control error, copy latest error, and `data-session-control-event`.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, `crates/webui/src/chat_ui_tool_lifecycle.html`, `crates/webui/src/chat_ui_session_controls.html`, and `crates/webui/src/conversation_controls.rs`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### Backend session controls contract

- Source anchor: `packages/session-ui/src/components/tool-count-summary.tsx` for count summaries that hide zero-count items and expose active count state; `packages/web/src/components/share/part.tsx` for result/detail toggles with show/hide controls, `aria-expanded`, and `aria-controls`; `packages/session-ui/src/components/session-turn.tsx` for visible session actions, per-turn status, retry affordance, assistant copy affordance, action history semantics, changed-file diff summaries, overflow `showAll` toggles for long changed-file lists, hidden overflow-more rows, turn duration calculation, readable error unwrap behavior, and visible turn-level error card rendering.
- Required behavior tokens: backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, copy session receipt, session control event ledger, copy session control event, session-control event disclosure, visible event JSON detail, session-control count summary, status filters, session-control diff summary, before/after/removed message chips, session-control duration summary, start/completion/duration chips, session-control ledger overflow, show older/show less overflow toggle, visible ledger count, hidden older receipt row, session-control ledger export, copy all events, session-control error card, latest session-control error, copy latest error, `data-session-control-event`, and `forge:session-control` browser events.
- Forge implementation paths under guard: `crates/engine/src/agent.rs`, `crates/webui/src/conversation_controls.rs`, `crates/webui/src/chat_ui_session_controls.html`, `scripts/smoke/check-session-controls-contract.py`, and `scripts/smoke/check-full-agentic-benchmark.py`.

### Browser proof capture contract

- Source anchor: `packages/session-ui/src/components/session-turn.tsx` because browser proof must capture the readable session UI that exposes final answer, session actions, and error cards. This slice specifically follows the upstream delayed-rendering pattern around `requestAnimationFrame` and `shown` state before proof capture.
- Required behavior tokens: `BROWSER_PROOF_SOURCE`, `CHROME_PROOF_FLAGS`, `--no-sandbox`, `--disable-dev-shm-usage`, `--run-all-compositor-stages-before-draw`, `diagnosable_browser_failure`, PNG signature validation, non-empty screenshot artifacts, `SCREENSHOT_CHROME_TIMEOUT_MS`, `SCREENSHOT_VIRTUAL_TIME_BUDGET_MS`, `SCREENSHOT_BROWSER_TIMEOUT_SECONDS`, `DOM_CHROME_TIMEOUT_MS`, and `DOM_BROWSER_TIMEOUT_SECONDS`.
- Forge implementation paths under guard: `crates/engine/src/tool/browser.rs`, `scripts/smoke/fast-webui-proof.sh`, `scripts/smoke/app-build-one-file.sh`, and `scripts/smoke/live-webui-feature-sprint.sh`.
