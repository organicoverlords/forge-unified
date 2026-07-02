# Browser proof CI screenshot fallback guard — 2026-07-01T10:47Z

## Selection basis

- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open, non-draft, mergeable before this slice.
- Prior inspected head: `dc5161acaa22bac5854ddf8c68739b078dfbb4d1`.

## Failed workflow evidence inspected

- CI `28508811307`: success.
- Build Proof `28508811326`: success.
- App Build Proof `28508811305`: success.
- App Multistep Build Proof `28508811363`: success.
- Fast WebUI Proof `28508811321`, job `84503780970`: failed after live NIM/WebUI stream while capturing readable browser proof. Chrome `--screenshot` segfaulted at `scripts/smoke/capture-browser-proof.sh` line 113. Artifact: `8006358899`.
- Live WebUI Feature Sprint `28508811327`, job `84503780844`: failed after NIM conversation and tool lifecycle conversation while capturing browser proof. Chrome `--screenshot` segfaulted at the same path.

## Feature / proof-path slice built

Changed `scripts/smoke/capture-browser-proof.sh` so GitHub Actions `pdf-first` browser proof no longer falls through to the known-bad direct Chrome `--screenshot` path after PDF capture fails unless `FORGE_BROWSER_PROOF_ALLOW_SCREENSHOT_AFTER_PDF_FAIL=1` is explicitly set.

This is a source/proof-path implementation slice, not docs-only and not cosmetic-only. It changes runtime proof harness behavior used by Fast WebUI Proof and Live WebUI Feature Sprint.

## OpenCode source backing

Upstream source paths inspected and used as reference:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `partState`
  - `assistantVisible`
  - `showThinking`
  - `MAX_FILES`
  - `showAll`
  - `overflow`
  - `visible`
  - `session-turn-diffs-more`
  - error-card rendering

Reference rationale: OpenCode keeps proof-relevant session-turn UI visible, compact, delayed-render-safe, and diagnosable. Forge’s browser proof harness must fail quickly and diagnostically instead of spending the proof budget on a known broken screenshot fallback.

## Files changed

- `scripts/smoke/capture-browser-proof.sh`
- `PROJECT_STATE.md`
- `docs/generated/proof/browser-proof-ci-screenshot-fallback-disabled-20260701T1047Z.md`

## Claim boundary

Do not claim same-head acceptance, production readiness, OpenCode parity, or browser screenshot proof until same-head workflows complete and artifacts contain inspected readable PNG browser proof from NVIDIA NIM WebUI natural-language prompts.
