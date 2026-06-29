# Max-step finalization parity CI gate — 2026-06-29T09:46Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft/mergeable at live read
- Starting live PR head observed before this slice: `c396280c36d2940be3d2025cbcb3bc1250332e3a`
- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Live workflow state before this slice

For head `c396280c36d2940be3d2025cbcb3bc1250332e3a`:

- Build Proof `28363086374`: success
- CI `28363086372`: in progress at read time
- Live WebUI Feature Sprint `28363086396`: in progress at read time

No same-head WebUI/NVIDIA NIM proof was available for this head at the time this slice was made.

## OpenCode source backing

Inspected upstream source:

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant OpenCode behavior recorded from that file:

- Max-step finalization disables tools until next user input.
- The assistant must provide a text-only response.
- The response must summarize work done so far.
- The response must list remaining tasks.
- The response must recommend what should happen next.

## Slice built

Added deterministic CI coverage for Forge's max-step/evidence-ready finalization contract:

- New script: `scripts/smoke/check-max-step-finalization-parity.py`
- CI integration: `.github/workflows/ci.yml`

The gate checks that `crates/engine/src/orchestrator.rs` continues to:

- record the OpenCode max-step source path;
- disable tools/tool choice for forced finalization;
- require a final Markdown report rather than tool calls or JSON-only output;
- prevent unproven file/test/build/fix claims;
- require Founder report, Technical report, `VERIFIED`, `LIKELY`, `UNKNOWN`, confidence, unresolved risks, and rollback strategy;
- keep a conservative fallback final report;
- retain evidence-ready benchmark finalization metadata.

## Proof command added to CI

```bash
python3 -m py_compile scripts/smoke/check-max-step-finalization-parity.py
python3 scripts/smoke/check-max-step-finalization-parity.py
```

## Files changed

- `scripts/smoke/check-max-step-finalization-parity.py`
- `.github/workflows/ci.yml`
- `docs/generated/proof/max-step-finalization-parity-ci-gate-20260629T0946Z.md`
- `PROJECT_STATE.md`
- PR body updated after commit

## Proof status

This slice adds a source-backed CI guard. It does not itself prove live WebUI/NVIDIA NIM parity.

Required proof before claiming latest-head parity:

- CI green on the final head.
- Build Proof green on the final head.
- Live WebUI Feature Sprint green on the final head.
- Live artifact includes natural-language prompt, browser screenshot PNG, stream/transcript, conversation JSON, checker JSON, and NVIDIA NIM provider/model/tool evidence.
