# Forge Unified — Current State

Updated: 2026-06-30

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`
- Previous selected head `545fec357de85718151c4b726a897092bfa5bcca` had CI, Build Proof, Fast WebUI Proof, App Build Proof, and App Multistep Build Proof green, but Live WebUI Feature Sprint `28464823475` failed.
- Inspected failed Live artifact `7989433741`: the full benchmark run used provider `nvidia_nim` and model `deepseek-ai/deepseek-v4-flash`; the only failed checker item was `claimed_cargo_tests_are_tool_proven`.
- Root cause: `scripts/smoke/check-full-agentic-benchmark.py` treated `UNKNOWN: Whether the full test suite passes — no cargo test was executed` as a cargo-test success claim when `cargo test` was Markdown-code-formatted.
- Latest implementation/proof slice: cargo command claim detector now normalizes Markdown punctuation and applies negation filtering before cargo-test/cargo-build success claim checks.
- New proof doc: `docs/generated/proof/full-benchmark-cargo-claim-negation-20260630T1815Z.md`.
- Do not claim the latest head containing this slice is same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on that exact head and artifacts/screenshots are inspected.

## Latest implementation changes

- Kept backend-backed session operations in `crates/engine/src/agent.rs`: `retry_source`, `fork_conversation`, `revert_last_turn`, and `session_control_receipt`.
- Kept backend route handlers in `crates/webui/src/conversation_controls.rs` so checkpoint, fork, revert-latest-turn, and retry-source all return structured receipt payloads.
- Kept browser control bundle `crates/webui/src/chat_ui_session_controls.html` with visible receipt strip, `copy session receipt`, `forge:session-control` events, event ledger, event rows, `data-session-control-event`, event copy, disclosure, count summary, status filters, diff summary, duration summary, overflow toggle, and hidden older receipt row.
- Updated `scripts/smoke/check-full-agentic-benchmark.py` so negated/future cargo command mentions do not require impossible tool evidence.
- Verified the patched checker locally against the failed artifact before committing: the same `full-benchmark-conversation.json` and `full-benchmark-stream.sse` now pass with zero failed checks.

## Proof requirements retained

- Same-head workflow proof is mandatory before acceptance.
- Browser artifacts must show provider/model route, session turn grouping, readable tool cards, final answer/proof summary, and no visible unwanted source-reference branding.
- Current required UI tokens include `backend-session-controls`, `backend-checkpoint-action`, `backend-fork-action`, `backend-revert-action`, `backend-retry-source-action`, `backend-session-control-status`, `backend-session-control-receipt`, `copy-session-control-receipt`, `session-control-event-ledger`, `backend-session-control-ledger`, `backend-session-control-event-row`, `copy-session-control-event`, `data-session-control-event`, `session-control-event-disclosure`, `backend-session-control-event-detail`, `show-session-control-event`, `aria-expanded`, `aria-controls`, `session-control-count-summary`, `backend-session-control-summary`, `backend-session-control-count`, `session-control-filter`, `session-control-filter-all`, `session-control-filter-ok`, `session-control-filter-error`, `aria-pressed`, `session-control-diff-summary`, `backend-session-control-diff-summary`, `backend-session-control-diff-chip`, `session-control-diff-before`, `session-control-diff-after`, `session-control-diff-removed`, `session-control-duration-summary`, `backend-session-control-duration-summary`, `backend-session-control-duration-chip`, `session-control-duration-ms`, `session-control-started-at`, `session-control-completed-at`, `session-control-ledger-overflow`, `backend-session-control-overflow-toggle`, `session-control-show-all`, `session-control-show-less`, `session-control-visible-count`, `session-control-hidden-overflow-row`, and `forge-local-control-receipt`.
- This slice fixes proof accuracy for cargo command claim validation. It does not claim full parity, production readiness, or same-head acceptance until workflows finish.

## Compatibility proof trail retained for deterministic gates

### Browser proof part contract

- Source anchors: `packages/session-ui/src/components/session-turn.tsx`, `packages/session-ui/src/components/message-part.tsx`, `packages/session-ui/src/components/basic-tool.tsx`, `packages/session-ui/src/components/tool-count-summary.tsx`, `packages/web/src/components/share/part.tsx`, `packages/web/src/components/share/part.module.css`.
- Required trail tokens: `proof-final`, session turn, assistant parts, copy/retry, changed files, collapsed technical details, turn receipt grouping, stable session receipts, timeline action groups, file diff summary, typed tool cards, tool targets, result toggles, flattened tool input, diagnostics, count summary, tool lifecycle strip, tool state timeline, copy tool anchor, duration footer, stable tool receipt ids, copy tool receipt, lifecycle event ledger, copy tool event, tool event JSON, input toggle, diagnostic copy, target copy, preview pane, preview toggle, copy preview, accessible disclosure state, `aria-expanded`, `aria-controls`, backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, session control event ledger, copy session control event, session-control event disclosure, event detail disclosure, show event, session-control count summary, status filters, session-control diff summary, before/after/removed message chips, session-control duration summary, start/completion/duration chips, session-control ledger overflow, show older/show less overflow toggle, visible ledger count, hidden older receipt row, and `data-session-control-event`.
- Forge implementation paths under guard: `crates/webui/src/chat_ui.rs`, `crates/webui/src/chat_ui.html`, `crates/webui/src/chat_ui_enhancements.html`, `crates/webui/src/chat_ui_tool_lifecycle.html`, `crates/webui/src/chat_ui_session_controls.html`, and `crates/webui/src/conversation_controls.rs`.

### Durable ToolPart lifecycle contract

- Source anchors: `anomalyco/opencode:packages/opencode/src/session/processor.ts`, `anomalyco/opencode:packages/schema/src/v1/session.ts`, `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`.
- Required trail tokens: `tool lifecycle`, `ToolPart`, `ToolStatePending`, `ToolStateRunning`, `ToolStateCompleted`, `ToolStateError`, `callID`, `attachments`, `max steps`, text-only finalization.
- Forge implementation paths under guard: `crates/engine/src/tool_parts.rs`, `scripts/smoke/check-opencode-tool-lifecycle-contract.py`, and `scripts/smoke/full-agentic-benchmark-prompt.txt`.

### Backend session controls contract

- Source anchor: `packages/session-ui/src/components/tool-count-summary.tsx` for count summaries that hide zero-count items and expose active count state; `packages/web/src/components/share/part.tsx` for result/detail toggles with show/hide controls, `aria-expanded`, and `aria-controls`; `packages/session-ui/src/components/session-turn.tsx` for visible session actions, per-turn status, retry affordance, action history semantics, changed-file diff summaries, overflow `showAll` toggles for long changed-file lists, hidden overflow-more rows, and turn duration calculation.
- Required behavior tokens: backend-backed session controls, checkpoint, fork, revert latest turn, retry source, Forge-local session control receipts, copy session receipt, session control event ledger, copy session control event, session-control event disclosure, visible event JSON detail, session-control count summary, status filters, session-control diff summary, before/after/removed message chips, session-control duration summary, start/completion/duration chips, session-control ledger overflow, show older/show less overflow toggle, visible ledger count, hidden older receipt row, `data-session-control-event`, and `forge:session-control` browser events.
- Forge implementation paths under guard: `crates/engine/src/agent.rs`, `crates/webui/src/conversation_controls.rs`, `crates/webui/src/chat_ui_session_controls.html`, `scripts/smoke/check-session-controls-contract.py`, and `scripts/smoke/check-full-agentic-benchmark.py`.
