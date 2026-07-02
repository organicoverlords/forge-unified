# Live WebUI proof manifest gate — 2026-06-29T15:46Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- Selection basis: target URL remains the source of truth; PR #3 is still the meaningful open app-change PR for this branch.
- Pre-slice verified current head: `59f9d4a71625d0dfe7125df9c816b8f47930fce5`
- Pre-slice PR state: #3 open, non-draft, mergeable in metadata.

## Same-head proof before this slice

The actual current branch head before this slice was fully green:

- CI `28382878597`: success.
- Build Proof `28382878610`: success; artifact `7956402464`, digest `sha256:c5381969921f8c71f6a18b56ec9280630ab6e1d06a5c2845bc5d0845282ce64b`.
- Live WebUI Feature Sprint `28382878593`: success; artifact `7956715745`, digest `sha256:5ce3895a333ba27b5a1ddc09c07b01587a9f0fe1c76d72d6cbc587abadc3f5f9`.

The green Live WebUI artifact is the current same-head browser/NVIDIA NIM proof baseline for `59f9d4a71625d0dfe7125df9c816b8f47930fce5`.

## Source-backed parity slice

Built a deterministic Live WebUI proof manifest gate so a future green Live WebUI workflow cannot omit the browser-proof screenshot/JSON/stream/conversation/checker/quality artifacts while still looking green.

Files changed:

- `scripts/smoke/check-live-webui-proof-manifest.py`
  - Adds `quality-score.json` and `live-webui-proof-manifest.json` to the required proof bundle.
  - Requires the final browser PNG to be a non-empty PNG screenshot.
  - Requires browser JSON markers for the natural full benchmark prompt, Phase 1/2, Founder report, Technical report, and `.agent_test` artifacts.
  - Requires real NIM provider/model evidence and tool-call/tool-result depth.
  - Records OpenCode source backing in the emitted manifest.
- `.github/workflows/live-webui-feature-sprint.yml`
  - Runs the manifest gate after hard benchmark checker, OpenCode workflow checker, and quality scorer.
  - Uploads `live-webui-proof-manifest.json` inside `live-webui-feature-sprint-proof`.

## OpenCode source backing

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`
  - Relevant behavior: when maximum steps are reached, tools are disabled; the agent must respond with text only, summarize completed work, list remaining tasks, and recommend next steps.
  - Forge mapping: the proof artifact must preserve browser-visible final-report evidence after the no-tools finalization path, not just hidden checker status.

## Expected workflow state after push

New commits:

- `df2a0fa12925de3d90c6ebd54afa07736112db4e` — manifest checker update.
- `f280faf790c229e90d6e3ce2eefe10be47c029ec` — Live WebUI workflow gate wiring.

The latest head after this proof doc is not same-head WebUI/NIM proven yet. CI, Build Proof, and Live WebUI Feature Sprint need to complete on this head or a later head containing the manifest gate before claiming latest-head proof.
