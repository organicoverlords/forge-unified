# OpenCode normalized tool attachments parity slice

Date: 2026-06-28
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection

The selected work remains `organicoverlords/forge-unified` branch `mvp/nim-freellmapi-router-20260626` because it is the target URL branch and PR #3 is open and non-draft.

## Failed workflow inspected

Previous HEAD `5ea11d17efaf019c42749dc2d97aa913fce221da` had failed same-head workflows:

- Live WebUI Feature Sprint `28309348172`: failure.
- CI `28309348173`: failure.
- Build Proof `28309348175`: failure.

The Live WebUI log showed `scripts/smoke/live-webui-feature-sprint.sh: line 127: syntax error near unexpected token '('` after `forge-app` compiled, so the full benchmark artifacts were missing.

## Harness repair

`scripts/smoke/live-webui-feature-sprint.sh` was hardened to avoid fragile inline shell/JQ predicates around proof assertions:

- Added Python-backed `assert_conversation_nim` for provider/model/message validation.
- Added Python-backed `assert_tool_catalog` for `/api/tools` validation.
- Kept the hard rejection gates for `provider: local`, `local_shortcut`, and `event: benchmark-phase`.
- Kept real NIM-only requirements for live-model, tool-lifecycle, and full six-phase benchmark conversations.
- Added proof markers for the new normalized attachment metadata.

## OpenCode source backing

Upstream source inspected:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `toolResultOutput(...)` returns `{ title, metadata, output, attachments }`.
  - `completeToolCall(...)` stores `attachments` in completed ToolPart state.
  - The tool-result handler normalizes image file attachments and stores surviving `SessionV1.FilePart` values.

Exact source anchors:

- `packages/opencode/src/session/processor.ts:toolResultOutput`
- `packages/opencode/src/session/processor.ts:completeToolCall`
- `packages/opencode/src/session/processor.ts` tool-result handler attachment normalization / `filter(isFilePart)`

## Forge implementation

`crates/engine/src/orchestrator.rs` now adds an OpenCode-style attachment envelope to provider-selected file/patch tool results when concrete file paths are known.

New metadata:

- `attachments`: OpenCode-like FilePart objects with `type: file`, `mime`, `filename`, `url`, `source`, and upstream source marker.
- `opencode_normalized_attachments: true`
- `opencode_tool_attachments_source: packages/opencode/src/session/processor.ts:toolResultOutput normalized FilePart attachments`
- `opencode_tool_output_shape.attachments` is true when attachments are present.

This is still a metadata/shape parity slice. It does not implement full OpenCode image resizing or database-backed FilePart persistence.

## Files changed

- `crates/engine/src/orchestrator.rs`
- `scripts/smoke/live-webui-feature-sprint.sh`
- `docs/generated/proof/opencode-normalized-tool-attachments-20260628T0354Z.md`
- `OPENCODE-PARITY.md`
- `PROJECT_STATE.md`

## Current status

After the implementation commits, current HEAD became `e9797fdb69034017608fde1ab4eb5cce4798b75c` before docs updates. New workflows were queued/in progress:

- CI `28310588664`
- Build Proof `28310588667`
- Live WebUI Feature Sprint `28310588666`

Do not claim current-head live parity until those workflows complete successfully and the Live WebUI artifact contains same-head NVIDIA NIM browser screenshots.