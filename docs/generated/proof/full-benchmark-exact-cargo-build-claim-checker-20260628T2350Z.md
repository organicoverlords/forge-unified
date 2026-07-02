# Full Benchmark Exact Cargo Build/Check Claim Checker

Date: 2026-06-28
Branch: `mvp/nim-freellmapi-router-20260626`
Repo: `organicoverlords/forge-unified`

## Selection basis

The source-of-truth URL points to `mvp/nim-freellmapi-router-20260626`. PR #3 is the matching open, non-draft PR into `master`; no newer superseding app branch was selected.

## Inspected failed proof

Same-head workflows for `0235e75f9218f1ad45b14a5d08b177fe4b13fed1`:

- Build Proof `28339416253`: success.
- CI `28339416255`: success.
- Live WebUI Feature Sprint `28339416256`: failure.
- Artifact `7939661071`: `live-webui-feature-sprint-proof`, digest `sha256:fdb810e6bc9d6900fe9fd6d54caf0695492f76f02b2f305ee1f30db5c71e40c4`.

The failed Live WebUI job used real NVIDIA NIM (`nvidia_nim`, `deepseek-ai/deepseek-v4-flash`) and produced tool-call/tool-result evidence, but `scripts/smoke/check-full-agentic-benchmark.py` failed `claimed_build_check_is_tool_proven`.

## Built slice

Updated `scripts/smoke/check-full-agentic-benchmark.py` so build/check overclaim detection is exact-command based:

- `cargo check ... passed/success/green/succeeds` still requires a successful `ShellCommand` with `cargo check`.
- `cargo build ... passed/success/green/succeeds` still requires a successful `ShellCommand` with `cargo build`.
- Generic phrases like `build config`, `build proof`, `compilation not run`, or `validation passed` no longer create a synthetic cargo build/check claim.

This keeps the checker strict against unproven cargo build/check claims while avoiding false positives from required benchmark labels or report prose.

## OpenCode source backing

Reference paths used from upstream OpenCode:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `completeToolCall`
  - `failToolCall`
  - `toolResultOutput`
- `anomalyco/opencode:packages/schema/src/v1/session.ts`
  - `ToolPart`
  - `ToolState`

The relevant upstream behavior is that completed/failed tool state and output metadata are authoritative evidence. This Forge checker now follows the same principle: prose only triggers cargo-specific proof requirements when it names the exact cargo command.

## Proof status

Not same-head proven yet. New workflows must pass on the new branch head before claiming Live WebUI/NVIDIA NIM parity for this slice.
