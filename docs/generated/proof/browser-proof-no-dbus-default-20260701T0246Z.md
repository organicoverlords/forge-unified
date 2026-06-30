# Browser proof no-DBus default slice — 2026-07-01T02:46+03:00

## Selection basis

- Source of truth branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open, non-draft, mergeable.
- Selected head before this slice: `51be0610923761f6f14e372f057246e1e456361d`.
- Same-head status before this slice:
  - CI `28483064782`: success.
  - Build Proof `28483064781`: success.
  - Fast WebUI Proof `28483064812`: failure.
  - Live WebUI Feature Sprint `28483064793`: failure.
  - App Build Proof `28483064783`: failure.
  - App Multistep Build Proof `28483064777`: failure.

## Failed workflow inspection

- Fast WebUI Proof run `28483064812`, job `84423541492`, reached `run fast NIM/WebUI stream`, then failed during `capture readable browser proof`.
- Fast artifact: `7996408354` / `fast-webui-proof`.
- Log timing shows capture started at `23:43:31` and the job failed at `23:45:06`, matching the script's 60s screenshot plus 35s DOM capture budget.
- The failed run uploaded a proof artifact with 381 files; the workflow itself failed after browser capture, not before NIM/WebUI streaming.

## OpenCode source backing

Used upstream `anomalyco/opencode` branch `dev` paths as behavior reference:

- `packages/session-ui/src/components/session-turn.tsx`
  - `requestAnimationFrame` delayed diff reveal and `shown` state around rendered proof surfaces.
  - `SessionRetry`, assistant copy affordance, `turnDurationMs`, error-card rendering, and session-turn structure.
  - Exact relevant areas inspected: imports/session structure, assistant visible state, duration/error handling, delayed diff rendering, and turn-level error card.

Forge remains independently branded; OpenCode source paths are recorded here as reference only, not exposed in runtime UI.

## Implementation slice

Changed `scripts/smoke/capture-browser-proof.sh`:

- Disabled `dbus-run-session` as the default Chrome wrapper for proof capture.
- Added opt-in DBus wrapping through `FORGE_CHROME_USE_DBUS=1` for local diagnosis.
- Sanitized Chrome environment with DBus variables removed and `NO_AT_BRIDGE=1`.
- Added `XDG_RUNTIME_DIR` fallback and `--disable-ipc-flooding-protection`.
- Defaulted to `--headless=new` while allowing `FORGE_CHROME_HEADLESS` override.
- Added explicit browser proof diagnostics to `browser-chrome.log`:
  - Chrome path.
  - headless mode.
  - DBus wrapping state.
  - screenshot/DOM return codes.
  - PNG byte count.
- Added failure-tail printing when the primary browser proof JSON is not successful.
- Preserved strict proof checks for PNG validity, readable UI markers, provider/model evidence, and final-answer/tool-card/session markers.

## Claim boundary

This slice is a source-backed browser-proof reliability fix. It does not claim complete OpenCode parity, production readiness, or same-head WebUI screenshot proof until same-head workflows finish and artifacts/screenshots are inspected.
