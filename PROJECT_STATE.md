# Forge Unified — Current State

Updated: 2026-07-01

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Selected live head before this slice: `3f4c83415ee8d5bbd28e02dcfab3a33fadf20439`.
- PR state verified live: open, non-draft, mergeable.
- Same-head workflow state before this product slice: previous proof-focused head had not achieved same-head acceptance; repeated failures were concentrated in browser proof capture rather than basic build/CI.
- Latest implementation slice: real WebUI human action summaries. Added `crates/webui/src/chat_ui_action_summaries.html` and wired it into `crates/webui/src/chat_ui.rs` so every visible tool card gets a user-facing action summary with action, target, outcome, next step, and copy controls.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Added `crates/webui/src/chat_ui_action_summaries.html` as a browser-only product enhancement loaded by the real bundled chat UI, not by a test harness.
- Each `.tool` card now receives a visible `human-action-summary` section with:
  - human action label, for example inspected repo, wrote file, edited file, ran command, updated plan, or delegated subtask;
  - action target extracted from file pills, command/pattern/path input, metadata, typed-tool target, or visible subtitle;
  - explicit outcome state: completed, needs attention, running, or pending;
  - next-step guidance such as review diff, check command output, continue plan item, or read subtask result;
  - copy controls for the action summary and result.
- Added proof/DOM hooks for the real UI surface: `human-action-summary`, `action-outcome-visible`, `action-target-visible`, `action-next-step-visible`, `readable-tool-summary`, `tool-name-not-primary`, and `tool-json-collapsed`.
- Wired the new product enhancement into `crates/webui/src/chat_ui.rs` with `include_str!("chat_ui_action_summaries.html")` between the typed-tool renderer and lifecycle/session controls.
- This directly addresses the UX problem where tool cards were too tool-name/JSON oriented and did not clearly tell a non-coder what Forge did, what it touched, whether it succeeded, and what to inspect next.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI/backend tokens include `backend-session-controls`, `backend-checkpoint-action`, `backend-fork-action`, `backend-revert-action`, `backend-retry-source-action`, `backend-session-control-status`, `backend-session-control-receipt`, `copy-session-control-receipt`, `session-control-ledger-export`, `copy-session-control-ledger`, `session-control-error-card`, `backend-session-control-error-card`, `copy-session-control-error`, `latest-session-control-error`, `copy latest error`, `session-control-event-ledger`, `backend-session-control-ledger`, `backend-session-control-event-row`, `copy-session-control-event`, `data-session-control-event`, `session-control-event-disclosure`, `backend-session-control-event-detail`, `show-session-control-event`, `aria-expanded`, `aria-controls`, `session-control-count-summary`, `backend-session-control-summary`, `backend-session-control-count`, `session-control-filter`, `session-control-filter-all`, `session-control-filter-ok`, `session-control-filter-error`, `aria-pressed`, `session-control-diff-summary`, `backend-session-control-diff-summary`, `backend-session-control-diff-chip`, `session-control-diff-before`, `session-control-diff-after`, `session-control-diff-removed`, `session-control-duration-summary`, `backend-session-control-duration-summary`, `backend-session-control-duration-chip`, `session-control-duration-ms`, `session-control-started-at`, `session-control-completed-at`, `session-control-ledger-overflow`, `backend-session-control-overflow-toggle`, `session-control-show-all`, `session-control-show-less`, `session-control-visible-count`, `session-control-hidden-overflow-row`, `forge-local-control-receipt`, `stable session-control receipt identity`, `receipt_id`, `sequence`, and `forge.webui.session_controls`.
- This slice improves the actual WebUI usability. It does not claim full parity, production readiness, or same-head acceptance until workflows finish and artifacts are inspected.

## Compatibility proof trail retained for deterministic gates

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, lifecycle event ledger, copy tool event, tool event JSON, input toggle, diagnostic copy, target copy, preview pane, preview toggle, copy preview, accessible disclosure state, `aria-expanded`, `aria-controls`, backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, session control event ledger, copy session control event, session-control event disclosure, event detail disclosure, show event, session-control count summary, status filters, session-control diff summary, before/after/removed message chips, session-control duration summary, start/completion/duration chips, session-control ledger overflow, show older/show less overflow toggle, visible ledger count, hidden older receipt row, session-control ledger export, copy all events, session-control error card, latest session-control error, copy latest error, stable session-control receipt identity, receipt_id, sequence, and `data-session-control-event`.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, `crates/webui/src/chat_ui_action_summaries.html`, `crates/webui/src/chat_ui_tool_lifecycle.html`, `crates/webui/src/chat_ui_session_controls.html`, and `crates/webui/src/conversation_controls.rs`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### Backend session controls contract

- Source anchor: `packages/session-ui/src/components/tool-count-summary.tsx` for count summaries that hide zero-count items and expose active count state; `packages/web/src/components/share/part.tsx` for result/detail toggles with show/hide controls, `aria-expanded`, and `aria-controls`; `packages/session-ui/src/components/session-turn.tsx` for visible session actions, per-turn status, retry affordance, assistant copy affordance, action history semantics, changed-file diff summaries, overflow `showAll` toggles for long changed-file lists, hidden overflow-more rows, turn duration calculation, readable error unwrap behavior, visible turn-level error card rendering, stable message IDs, binary message lookup, and assistant part copy targets.
- Required behavior tokens: backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, stable session-control receipt identity, receipt_id, sequence, copy session receipt, session control event ledger, copy session control event, session-control event disclosure, visible event JSON detail, session-control count summary, status filters, session-control diff summary, before/after/removed message chips, session-control duration summary, start/completion/duration chips, session-control ledger overflow, show older/show less overflow toggle, visible ledger count, hidden older receipt row, session-control ledger export, copy all events, session-control error card, latest session-control error, copy latest error, `data-session-control-event`, and `forge:session-control` browser events.
- Forge implementation paths under guard: `crates/engine/src/agent.rs`, `crates/webui/src/conversation_controls.rs`, `crates/webui/src/chat_ui_session_controls.html`, `scripts/smoke/check-session-controls-contract.py`, and `scripts/smoke/check-full-agentic-benchmark.py`.

### Browser proof capture contract

- Source anchor: `packages/session-ui/src/components/session-turn.tsx` because browser proof must capture the readable session UI that exposes final answer, session actions, and error cards.
- Required behavior tokens: `BROWSER_PROOF_SOURCE`, `CHROME_PROOF_FLAGS`, `--no-sandbox`, `--disable-dev-shm-usage`, `--run-all-compositor-stages-before-draw`, `diagnosable_browser_failure`, PNG signature validation, non-empty screenshot artifacts, `SCREENSHOT_CHROME_TIMEOUT_MS`, `SCREENSHOT_VIRTUAL_TIME_BUDGET_MS`, `SCREENSHOT_BROWSER_TIMEOUT_SECONDS`, `DOM_CHROME_TIMEOUT_MS`, `DOM_BROWSER_TIMEOUT_SECONDS`, `capture_dom:false` for `fast-webui-proof.sh`, `FORGE_CHROME_USE_DBUS`, `FORGE_CHROME_HEADLESS`, `chrome_dbus_default_disabled`, and `browser-chrome.log` diagnostics.
- Forge implementation paths under guard: `crates/engine/src/tool/browser.rs`, `scripts/smoke/capture-browser-proof.sh`, `scripts/smoke/fast-webui-proof.sh`, `scripts/smoke/app-build-one-file.sh`, and `scripts/smoke/live-webui-feature-sprint.sh`.
