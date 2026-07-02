# Browser proof Chrome fallback proof

Date: 2026-07-01
Branch: `mvp/nim-freellmapi-router-20260626`
PR: #3

## Selection basis

The provided source-of-truth URL points to `organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
PR #3 is still the selected work because it is open, non-draft, mergeable, targets `master`, and its head branch is `mvp/nim-freellmapi-router-20260626`.

## Pre-slice live state

Inspected head before this slice: `b03c7365a625e093677edff080ac7e9372259512`.

Same-head workflow state for that head:

- Build Proof `28485868412`: success.
- Live WebUI Feature Sprint `28485868425`: failure.
- Fast WebUI Proof `28485868406`: failure.
- App Build Proof `28485868440`: failure.
- CI `28485868402`: failure.
- App Multistep Build Proof `28485868418`: failure.

Inspected Live WebUI failed job:

- Run: `28485868425`.
- Job: `84432079012`.
- The job reached WebUI startup, NIM conversation creation, model chat stream, tool lifecycle conversation creation, and browser proof capture for tool lifecycle.
- It then failed before full benchmark artifacts were created, and the checker step wrote `missing_full_benchmark_artifacts`.
- Artifact uploaded: `7997412194` / `live-webui-feature-sprint-proof`.

## OpenCode source backing

Used upstream OpenCode source paths as behavior references only. These paths are recorded in docs/proof, not exposed as Forge runtime branding.

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Source-backed behavior: the browser proof must capture the readable session turn surface, including assistant content, session status, changed-file summary, copy/retry affordances, duration metadata, and visible error card behavior.
  - Relevant source areas reviewed: `partState`, `heading`, `SessionTurn`, `working`, `turnDurationMs`, assistant copy target, changed-file overflow, and error card rendering.

## Feature built

Built one proof-harness product slice that materially affects browser proof reliability:

- `scripts/smoke/capture-browser-proof.sh`

New behavior:

- Keeps real Chromium screenshot capture and PNG signature validation.
- Keeps the existing DBus-off default.
- Adds reusable Chrome common flag generation for screenshot and DOM paths.
- First tries the requested headless mode from `FORGE_CHROME_HEADLESS` or `new`.
- If the primary screenshot attempt leaves no readable PNG, retries with `--headless=old`, `--single-process`, and `--disable-software-rasterizer`.
- If DOM dump fails with requested headless mode, retries DOM dump with old headless mode.
- Adds explicit diagnostics to `browser-chrome.log`:
  - requested headless mode;
  - fallback attempt label;
  - screenshot return code;
  - DOM return code;
  - PNG byte count;
  - DBus wrapping state.
- Adds proof JSON metadata:
  - `chrome_retry_fallbacks`;
  - `headless_old_retry`;
  - `single_process_retry`.

## Why this slice

The inspected failure pattern was not a NIM provider failure and not a Rust build failure. The run reached NIM/WebUI stream work, then failed at browser capture, which prevented later full benchmark evidence from being produced. This slice attacks that exact failure mode instead of adding unrelated UI or docs-only work.

## Files changed

- `scripts/smoke/capture-browser-proof.sh`
- `PROJECT_STATE.md`
- `docs/generated/proof/browser-proof-chrome-fallback-20260701T0447Z.md`

## Claim boundary

Not claimed yet:

- Same-head green workflows.
- Full OpenCode parity.
- Production readiness.
- Latest-head browser screenshot proof.

Acceptance still requires CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof to complete on the final exact head, with artifacts/screenshots inspected.
