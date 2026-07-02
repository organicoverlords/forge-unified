# WebUI natural feature-build gate — 2026-06-29T19:15Z

The Live WebUI Feature Sprint workflow now includes a dedicated natural-language feature-build proof path.

## Behavior added

- `scripts/smoke/natural-feature-work.sh` starts Forge WebUI, creates a conversation, sends a normal chat prompt, captures browser proof, and writes `summary.json` plus `summary.md`.
- The default prompt requires a repo-local edit, a generated proof note, a shell syntax validation command, and a final human-readable summary.
- `.github/workflows/live-webui-feature-sprint.yml` now runs this natural feature-build prompt and requires `natural-feature-work/summary.json` to pass.

## Acceptance boundary

This proves the WebUI can execute a natural feature-build prompt in GitHub Actions and leave artifacts under `forge-proof/live-webui-feature-sprint/natural-feature-work/`. It does not by itself prove complete runtime parity.

Latest branch head still needs same-head CI, Build Proof, and Live WebUI Feature Sprint results before it can be accepted.
