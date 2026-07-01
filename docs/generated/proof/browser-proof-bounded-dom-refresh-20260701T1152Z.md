# Browser proof bounded DOM refresh — 2026-07-01T11:52Z

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- Source of truth: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`
- PR: #3 into `master`, open and non-draft at inspection time.
- Previous inspected head: `0d1323f50b0381067202548f67af80c5f11717f4`.

## Failure inspected

Same-head workflow state for `0d1323f50b0381067202548f67af80c5f11717f4`:

- CI `28512299366`: success.
- Build Proof `28512299426`: success.
- App Build Proof `28512299392`: success.
- App Multistep Build Proof `28512299402`: success.
- Fast WebUI Proof `28512299407`, job `84515327777`: failed at `capture readable browser proof`; artifact `8007875404` uploaded.
- Live WebUI Feature Sprint `28512299419`, job `84515328205`: failed at `browser proof tool lifecycle` with exit code `124`; artifact `8007834789` uploaded.

The remaining blocker was not the NIM stream or Rust build gates. The failure was the browser-proof capture path consuming the 120s proof budget after the WebUI/NIM stream had already progressed.

## OpenCode source backing

Upstream reference paths used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `partState()` visible-part filtering.
  - `assistantVisible` / `showThinking` delayed visible surface logic.
  - `showAll`, `overflow`, and `visible` bounded disclosure behavior.
  - Session turn assistant/error-card rendering around delayed or failing assistant state.

Relevant source lines inspected in this run include the visible part filtering and session turn memoization/rendering code around `partState`, `assistantVisible`, and the delayed assistant content region.

## Implementation slice

Changed `scripts/smoke/capture-browser-proof.sh`:

- Added `FORGE_BROWSER_PROOF_REFRESH_DOM_WITH_CHROME`.
- GitHub Actions default: `0`; local default: `1`.
- CI now uses the initial `curl` fetch of the exact WebUI URL as the DOM snapshot instead of running a second Chrome `--dump-dom` pass after visual capture.
- Chrome visual proof remains required: accepted proof still needs a readable PNG from Chrome PDF-to-PNG or screenshot capture.
- Reduced per-attempt Chrome budgets:
  - PDF attempt default: 18s.
  - Screenshot attempt default: 16s.
  - DOM refresh attempt default: 12s when explicitly enabled.
- Added proof metadata and diagnostics:
  - `ci_curl_dom_snapshot_default`
  - `chrome_dom_refresh_opt_in`
  - `bounded_visual_attempts`
  - `bounded-dom-refresh` in fallback diagnostics.

## Claim boundary

This is a real harness behavior change and not docs-only. It does not claim same-head browser proof yet. The new head must still complete same-head Fast WebUI Proof / Live WebUI Feature Sprint and artifacts must contain readable browser PNG proof before parity or acceptance is claimed.
