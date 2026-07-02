# Full benchmark exact-command claim checker — 2026-06-28T22:45Z

## Selection basis

- Source-of-truth branch: `mvp/nim-freellmapi-router-20260626`.
- Matching PR: #3, open/non-draft.
- Prior latest head: `e551b3a924e539d1c522589bcede5f847306100c`.

## Inspected failed workflow

- Live WebUI Feature Sprint run: `28337141161`.
- Job: `83945215615`.
- Provider/model evidence in failed run: `nvidia_nim` / `deepseek-ai/deepseek-v4-flash`.
- The benchmark reached 38 `tool-call` events and 38 `tool-result` events and the OpenCode workflow checker passed.
- Failure was isolated to `check-full-agentic-benchmark.py` checks `claimed_cargo_tests_are_tool_proven` and `claimed_build_check_is_tool_proven`.

## Feature slice

`check-full-agentic-benchmark.py` now treats final-answer test/build claims as exact-command claims:

- The required final summary label `tests run` is no longer interpreted as a `cargo test` success claim by itself.
- `cargo test` proof is required only when final prose explicitly claims `cargo test` or all cargo tests passed.
- `cargo check` / `cargo build` proof is required only when final prose explicitly claims those commands or compilation success.
- The checker still fails real overclaims when the corresponding exact successful tool command is absent.

## OpenCode source backing

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — ToolPart completion/failure records exact tool result output and state before finalization.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — ToolState/FilePart structure anchors the exact result envelope shape.

Forge parity rule used here: evidence acceptance must come from exact tool-result metadata (`kind`, `command`, `path`, `success`), not broad inference from report prose labels.

## Validation status

- Static syntax validation is delegated to CI / Build Proof / Live WebUI on the pushed head.
- Same-head WebUI/NIM proof is not claimed until the workflows for the new head complete and publish artifacts.
