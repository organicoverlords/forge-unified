# Browser proof path/read guards — 2026-07-01 07:54Z

## Selection basis

- Source of truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- PR: #3, open, non-draft, mergeable.
- Head inspected before this slice: `eb7131fa07c078f083b366d0b592f3ac910a5e25`.
- Same-head workflow state before this slice: Build Proof, App Build Proof, and App Multistep Build Proof were successful; CI, Fast WebUI Proof, and Live WebUI Feature Sprint were failing.

## Failed workflow/log inspection

Fast WebUI Proof run `28492222206`, job `84451141847`, failed during browser proof capture.

Relevant log findings:

- NIM/WebUI stream reached browser capture: `capture readable browser proof`.
- The capture harness emitted missing PNG path errors for `forge-proof/fast-webui-proof/webui.png`.
- Chrome fallback attempt then segfaulted, and the harness exited without a readable PNG.

This means the browser proof path still was not acceptable; the app/router proof work before screenshot capture may have completed, but same-head screenshot proof remained unproven.

## OpenCode source backing

Reference source paths used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`
  - Rendered assistant turn surface.
  - Duration/error/copy/diff proof surfaces.
  - `showAll`, `overflow`, `visible`, and `session-turn-diffs-more` delayed/conditional rendering patterns.

The Forge slice keeps the same principle: proof should be attached to a rendered turn surface and must not be accepted until the rendered browser artifact exists.

## Forge implementation

Changed path:

- `scripts/smoke/capture-browser-proof.sh`

Behavior added:

- Create screenshot, DOM, JSON, and Chrome profile parent directories before browser capture attempts.
- Add `png_size()` so missing screenshot files are recorded as `0` bytes without shell redirection errors.
- Add JSON/diagnostic metadata:
  - `screenshot_path_parent_guard`
  - `png_size_redirection_guard`
  - `path_parent_guard=true`
  - `png_size_redirection_guard=true`
- Preserve the acceptance boundary: browser proof still requires a valid readable PNG; DOM or JSON fallback alone does not pass.

## Claim boundary

This slice is not a parity claim and not a production-readiness claim.

It only removes a concrete harness failure mode found in the live logs. Latest-head browser screenshot proof must still pass and be inspected before claiming acceptance.
