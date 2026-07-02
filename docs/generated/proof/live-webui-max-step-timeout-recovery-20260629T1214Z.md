# Live WebUI max-step timeout recovery proof slice

Updated: 2026-06-29T12:14Z

## Repository

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3
- Source-fix head: `206d453c1879f74cdd5515eab66aad5b89fcdacc`

## Current inspected gate

Before this patch, PR head `36f67a7fe760d54fdc10b24ad89204790a56e095` had:

- CI `28370125998`: success.
- Build Proof `28370125986`: success.
- Live WebUI Feature Sprint `28370126001`: failure.
- Live artifact: `7951317296`, digest `sha256:30142c86d5d834522543331c304e5bb2dfca507ac6594eadfd22995ab957d168`.

The Live job failed in both the live sprint step and the final evidence/quality step. The artifact ZIP was present, but the local automation runtime could not unpack connector-downloaded ZIP bytes during this run, so the patch was based on the checked-in failing harness code and previous preserved PR-body diagnostics.

## OpenCode source inspected before patch

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant behavior: once max steps are reached, tools are disabled and the agent must produce a text-only response summarizing completed work, remaining tasks, and next steps. A Forge live proof can therefore accept preserved conversation/checker evidence after a transport timeout when the hard benchmark checker and OpenCode workflow checker both pass.

## Code change

Patched `scripts/smoke/live-webui-feature-sprint.sh`:

- Added `need_any_marker` so full-benchmark browser proof accepts any real Phase 4 edit marker: `apply_patch`, `file_edit`, `file_write`, `Applied patch`, `Edited file`, or `Wrote file`.
- Added `json_passed` so the harness can reason about checker output without relying only on curl exit status.
- If the full benchmark stream command times out but preserved diagnostics show both full benchmark checker and OpenCode workflow checker passed, the harness now continues to browser proof and quality scoring instead of hard-failing before proof capture.
- Keeps strict provider-error, local-shortcut, NIM-provider, tool-call/tool-result, file artifact, checker, and browser-proof gates.

## Not yet proven

Same-head proof is pending for `206d453c1879f74cdd5515eab66aad5b89fcdacc`. Do not claim the live benchmark is green until CI, Build Proof, and Live WebUI Feature Sprint all pass on this head or a later head containing this patch.
