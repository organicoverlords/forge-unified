# Browser Proof Chrome Capture Hardening — 2026-06-30T2010Z

## Problem observed

Head `c09eae1503046bc2e3afbe2fbf69d30a53adcb06` failed browser-proof workflows even though the NIM model/tool loops completed.

Observed failed artifacts:

- Fast WebUI Proof: model answered the fast marker, but `browser-proof.json` was empty and no screenshot was produced.
- App Build Proof: file write succeeded and the target marker was present, but `browser_success` and `screenshot_png` failed because `browser-proof.json` was empty.
- Live WebUI Feature Sprint and App Multistep Build Proof failed on the same browser screenshot capture layer.

This was not a model/tool-call failure; it was a browser proof capture failure.

## Fix

`crates/engine/src/tool/browser.rs` now hardens the browser proof path:

- Centralizes Chrome launch flags in `CHROME_PROOF_FLAGS`.
- Adds runner-stable Chrome flags:
  - `--no-sandbox`
  - `--disable-dev-shm-usage`
  - `--disable-background-networking`
  - `--disable-extensions`
  - `--disable-sync`
  - `--hide-scrollbars`
  - `--mute-audio`
  - `--run-all-compositor-stages-before-draw`
- Validates that a successful Chrome process produced a non-empty PNG screenshot.
- Returns structured `BrowserProofResult { success: false, error, console_logs }` metadata for failed Chrome captures, so failed artifacts are diagnosable instead of blank.
- Records `diagnosable_browser_failure` metadata for failure-path proof.

## Source-backed UI target

Source anchor retained: `packages/session-ui/src/components/session-turn.tsx`.

The browser proof tool must capture the readable session UI: final answer, session timeline, session actions, tool cards, and error cards. The capture path is Forge-local and does not expose source-reference branding in the browser UI.

## Claim boundary

This commit fixes the browser-proof capture mechanism and improves diagnostics. It does not by itself prove the latest head green. Same-head workflows and browser artifacts must still pass and be inspected on the exact post-fix head.
