# Browser proof PDF visual fallback — 2026-07-01

## Selection

- Repository: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- Selection basis: target URL points to this branch; PR #3 is still open, non-draft, and mergeable; no newer open PR was found that supersedes this target.
- Previous selected HEAD: `e747756aa27bb07b193f9d0f38518800c6907f5f`

## Failure inspected

- Fast WebUI Proof `28502173116`, job `84481844125`, failed after NIM/WebUI stream completed and Chrome screenshot capture segfaulted in `scripts/smoke/capture-browser-proof.sh`.
- Live WebUI Feature Sprint `28502173203`, job `84481844230`, failed on the same Chrome screenshot capture segfault before full benchmark artifacts and natural-feature screenshot could be produced.
- Artifact IDs from failed same-head runs:
  - Fast WebUI: `8003545705`
  - Live WebUI Feature Sprint: `8003557972`

## Source-backed implementation slice

Built a browser proof harness fallback that remains browser-rendered and produces a PNG proof artifact when direct Chrome `--screenshot` paths fail on GitHub-hosted runners:

- Primary: existing direct Chrome `--screenshot` path.
- Secondary: existing `--headless=old --single-process --disable-software-rasterizer` path.
- New fallback: Chrome `--print-to-pdf` renders the same WebUI URL to a browser PDF, then `pdftoppm` or ImageMagick converts the first rendered page to the required PNG proof file.
- The JSON proof metadata now records `chrome_print_pdf_visual_fallback` and `browser_rendered_pdf_to_png_fallback` when this fallback path is used.
- Success still requires a readable PNG and the existing provider/UI marker checks. DOM-only or JSON-only proof is still not accepted.

## OpenCode source backing

Exact upstream source paths used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - `partState` hides non-useful/internal tool surface and keeps useful proof-visible tool/message parts.
  - `MAX_FILES`, `showAll`, `overflow`, and `visible` keep long session artifacts compact but inspectable.
  - `session-turn-diffs-more` and error card rendering provide the reference for proof surfaces that remain visible and diagnosable instead of silently dropping failed render state.

## Forge paths changed

- `scripts/smoke/capture-browser-proof.sh`
- `PROJECT_STATE.md`
- `docs/generated/proof/browser-proof-pdf-visual-fallback-20260701T1155Z.md`

## Claim boundary

This slice is not same-head proven until CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof complete on the new exact head and the uploaded artifacts contain readable PNG screenshots from natural-language WebUI prompts using NVIDIA NIM only.
