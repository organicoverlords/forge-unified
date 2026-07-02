# Browser proof CI PDF-first visual strategy — 2026-07-01

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- Selection basis: target URL points to this branch; PR #3 is still open, non-draft, and mergeable; no newer open PR was found that supersedes this target.
- Previous selected HEAD inspected: `688a9fff5ec8c4f5b65c9b231942acd7e6522dca`

## Failure inspected

- Fast WebUI Proof `28505554865`, job `84493016643`, failed after static UI checks, provider tool catalog checks, live NIM/WebUI stream, and then browser proof capture.
- The failure line was Chrome `--screenshot` segmentation fault in `scripts/smoke/capture-browser-proof.sh`, followed by job failure before a readable PNG proof could be accepted.
- Live WebUI Feature Sprint `28505554882`, job `84493016717`, failed on the same Chrome `--screenshot` segmentation fault after creating the NIM conversation and reaching browser proof for the tool lifecycle.
- Artifact IDs from the failed same-head runs:
  - Fast WebUI Proof: `8005012897`
  - Live WebUI Feature Sprint: artifact available from the run upload step, not accepted as same-head screenshot proof because capture failed before readable PNG validation.

## Source-backed implementation slice

Built a browser proof capture strategy that avoids spending most of the GitHub Actions budget on a known-crashing Chrome `--screenshot` path:

- `FORGE_BROWSER_PROOF_STRATEGY` now controls capture order.
- On GitHub Actions, the default strategy is `pdf-first`.
- `pdf-first` renders the exact WebUI URL with Chrome `--print-to-pdf`, converts the browser-rendered PDF to the required PNG, and then runs the same marker validation path.
- Direct screenshot attempts remain available as fallback and as the default local/non-CI strategy.
- Direct screenshot timeout is reduced from 45s to 25s, and screenshot virtual-time budget is reduced so segmentation faults cannot consume the full proof job budget before fallback paths run.
- Proof JSON metadata records `ci_pdf_first_strategy` and `screenshot_segmentation_fault_timeout_guard`.

## OpenCode source backing

Exact upstream source paths used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `partState` and assistant visible/working state keep useful proof-relevant parts visible and suppress non-useful internal noise.
  - `showThinking`, `assistantVisible`, and delayed assistant rendering guided the requirement that Forge proof capture tolerate delayed browser-rendered UI state.
  - `MAX_FILES`, `showAll`, `overflow`, `visible`, and `session-turn-diffs-more` guided compact-but-inspectable proof surfaces.
  - Error card rendering guided preserving diagnostic output rather than silently dropping failed render/capture state.

## Forge paths changed

- `scripts/smoke/capture-browser-proof.sh`
- `PROJECT_STATE.md`
- `docs/generated/proof/browser-proof-ci-pdf-first-20260701T1245Z.md`

## Claim boundary

This slice is not same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the new exact head and the uploaded artifacts contain readable PNG screenshots from natural-language WebUI prompts using NVIDIA NIM only.
