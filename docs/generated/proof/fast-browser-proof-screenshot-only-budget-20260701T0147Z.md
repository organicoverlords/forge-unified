# Fast browser proof screenshot-only budget — 2026-07-01T01:47+03:00

## Selection

- Repo: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open, non-draft, mergeable before this slice.
- Starting head inspected: `3cb90110bb42529d6386b6bf9834b48f342df660`.

## Live failure inspected

- Fast WebUI Proof run: `28478458720`, job `84408901801`, conclusion `failure`.
- CI run on same head: `28478458714`, conclusion `success`.
- Build Proof run on same head: `28478458730`, conclusion `success`.
- Fast log reached `run fast NIM/WebUI stream`, then `capture readable browser proof`.
- Failure line: `curl: (28) Operation timed out after 60002 milliseconds with 0 bytes received` during `POST /api/browser-proof`.
- Artifact uploaded: `7994661225` / `fast-webui-proof`.

## OpenCode source backing

Reference paths used from upstream OpenCode:

- `packages/session-ui/src/components/session-turn.tsx`
  - `AssistantParts` receives turn duration and visible assistant parts after session state resolves.
  - Diff views are deferred behind `requestAnimationFrame` and `shown` state.
  - Error cards remain visible at the turn level.

Forge mapping:

- Fast proof must capture the readable final WebUI/session state reliably after NIM streaming.
- The screenshot proof is the acceptance artifact for this workflow.
- DOM capture is a second Chrome pass and should not be required in the fast screenshot path when the endpoint caller has a bounded CI budget.

## Implementation

Changed `scripts/smoke/fast-webui-proof.sh`:

- `POST /api/browser-proof` now uses `capture_dom:false` for Fast WebUI proof.
- Caller budget for the proof POST increased from 60s to 90s.
- Added an inline note documenting why Fast proof captures screenshot only: DOM capture has its own Chrome pass and can push the endpoint beyond the caller budget on hosted runners after NIM streaming.
- Kept all screenshot checks: browser tool success, non-empty `screenshot_base64`, PNG decode, UI marker checks, provider/model checks, and no visible unwanted source-reference branding.

## Claim boundary

This is a proof-path reliability slice, not a full parity claim.

Do not mark this latest head accepted until same-head CI, Build Proof, Fast WebUI Proof, App Build Proof, App Multistep Build Proof, and Live WebUI Feature Sprint complete and their artifacts/screenshots are inspected.
