# WebUI TodoWrite-style plan status part — 2026-06-30T01:46Z

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open and non-draft, mergeable in PR metadata.
- Selected pre-slice head: `6f43a985165a5b53db06e8c76ed2513c135d94ed`.
- Same-head status before this slice:
  - CI `28406505084`: success.
  - Build Proof `28406505066`: success.
  - App Build Proof `28406505063`: success.
  - App Multistep Build Proof `28406505077`: success.
  - Fast WebUI Proof `28406505108`: success.
  - Live WebUI Feature Sprint `28406505078`: success.
  - Live WebUI artifact: `7966282221`, digest `sha256:e20fd3f3cbc2661c77901a5eec3031baf21db8018cefbe2e8cfc43a324a4d92d`.

## OpenCode source backing

Exact upstream OpenCode source paths used as reference:

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
- `anomalyco/opencode:packages/web/src/components/share/part.module.css`

Source behavior used:

- `TodoWriteTool` handles todo statuses `pending`, `in_progress`, and `completed`, sorts active work before pending and completed items, and switches the human title between creating/updating/completing plan.
- `part.module.css` contains dedicated `[data-component="todos"]` and `[data-status="pending|in_progress|completed"]` presentation rules instead of rendering todo JSON as the primary result.

## Feature built

Implemented a WebUI plan-status part for Forge `todo_write` tool results:

- `crates/webui/src/chat_ui.rs`
  - added `todoItems`, `todoSummary`, and `addTodoStatus` helpers;
  - renders `todo-status-summary` and `todo-counts` chips for plan state counts;
  - summarizes `todo_write` as `Plan updated: ...` instead of falling through to generic collapsed JSON text.
- `scripts/smoke/check-webui-proof-part-contract.py`
  - now enforces the TodoWriteTool-style plan status rendering contract;
  - requires `Update plan`, `todo-status-summary`, `todo-counts`, `Plan updated`, and todo status proof trail evidence.
- `PROJECT_STATE.md`
  - records the same-head green proof baseline before this slice and the new TodoWrite-style WebUI behavior.

## Files changed

- `crates/webui/src/chat_ui.rs`
- `scripts/smoke/check-webui-proof-part-contract.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/webui-todo-plan-status-part-20260630T0146Z.md`

## Claim boundary

This slice improves one WebUI part-rendering behavior using OpenCode as source reference. It does not claim complete OpenCode parity.

The latest head containing this slice is not accepted until same-head CI / Build Proof / Fast WebUI Proof / Live WebUI Feature Sprint complete with NVIDIA NIM browser proof artifacts on that exact SHA.
