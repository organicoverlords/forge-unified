# Final evidence digest path/command repair — 2026-06-28T17:45Z

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft into `master`
- Selection basis: target URL points at this branch and no newer open PR superseded it.

## Live state before this patch

- Starting HEAD observed: `6c69fe1e11c495b058253bc5305e64c3979ed842`
- Same-head workflows observed for that starting head:
  - Build Proof `28330704599`: in progress
  - CI `28330704602`: in progress
  - Live WebUI Feature Sprint `28330704615`: in progress

## Failure class addressed

Prior failed NIM/WebUI runs reached the real provider/tool path, but final-report proof was fragile because the forced-final evidence digest summarized tool results as kind/success/error/output only. That was insufficient for exact proof claims such as:

- which file path was written/read/deleted;
- which shell command actually ran;
- whether a build/check/test claim was supported by a matching successful command.

## Source-backed OpenCode reference

Upstream source paths used as reference material:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `completeToolCall` stores `output`, `metadata`, `title`, `time`, and `attachments` in the completed ToolPart state.
  - `failToolCall` stores error state instead of treating failure as invisible.
  - `toolResultOutput` normalizes tool result values into structured `title`, `metadata`, `output`, and optional attachments.

These paths are recorded here for developer proof only. They are not provider-facing runtime metadata.

## Implementation

Changed `crates/engine/src/orchestrator.rs`:

- Added `result_evidence_line()` to emit explicit evidence rows:
  - `kind`
  - `success`
  - `path`
  - `command`
  - `error`
  - bounded output
- Added `result_metadata_path()` and `result_metadata_command()` helpers to read direct metadata first and fall back to provider input metadata.
- Increased the forced-final digest window from 24 to 36 rows so long NIM/tool loops retain more recent file and validation evidence.
- Added run metadata marker `forge_final_evidence_digest = "path/command-aware evidence digest"`.

## Validation status

No same-head WebUI/NVIDIA NIM screenshot proof exists yet for this patch. Current-head parity is not proven until a green Live WebUI Feature Sprint artifact includes:

- `full-benchmark-webui.png`
- `full-benchmark-browser-proof.json`
- `full-benchmark-stream.sse`
- `full-benchmark-conversation.json`
- `full-benchmark-checker.json` with `passed: true`
- `opencode-workflow-checker.json` with `passed: true`
