# Live benchmark final-report proof contract

Date: 2026-06-28

## Selection

- Repository: `organicoverlords/forge-unified`
- Source-of-truth branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open and non-draft
- Inspected same-head before this change: `e296cbbc99c40895df82df63524b9b406b7b3b30`

## Failed proof inspected

- Live WebUI Feature Sprint run: `28328598367`
- Job: `83922751023`
- Provider/model evidence: `nvidia_nim` / `deepseek-ai/deepseek-v4-flash`
- Workflow evidence checker: passed.
- Full benchmark checker: failed because Phase 3 lacked dedicated `FileWrite` proof for `.agent_test/repo_summary.md`, `.agent_test/investigation.md`, and `.agent_test/action_plan.json`, and the final answer claimed a build/check without a matching successful cargo build/check tool result.

## Parity slice

Forge now gives the model a stricter OpenCode-style proof discipline before final reporting:

- Required temporary benchmark files must be created with dedicated `file_write` tool calls, not patch or shell shortcuts.
- Dedicated `file_read` and `file_delete` evidence is required before final reporting.
- Final reporting may not claim build/check/test/file success unless the recorded tool result proves the exact claim.
- Fallback final reports now include the benchmark checker’s exact summary labels and explicit uncertainty rather than implying success.

## OpenCode source backing recorded

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — processor-backed tool lifecycle, completed/failed tool result handling, final message construction after tool evidence.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — ToolPart/ToolState/FilePart evidence shape used as the downstream proof model.

## Files changed

- `crates/engine/src/orchestrator.rs`
- `PROJECT_STATE.md`
- `docs/generated/proof/live-benchmark-final-report-proof-contract-20260628T1655Z.md`

## Validation

Remote workflows are expected to run for the new head after this commit. Do not claim parity until same-head CI, Build Proof, and Live WebUI Feature Sprint produce green results with a same-head WebUI/NVIDIA NIM screenshot artifact.
