# Live WebUI proof shell rewrite and independence gate — 2026-06-28T09:55Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft into `master`
- Prior inspected HEAD: `a1dbb80d8a64e8c5abfe8b99a171dff397c419cd`
- Prior Live WebUI run: `28317071564`, failed in job `83892223737`

## Failure inspected

The same-head Live WebUI Feature Sprint job built `forge-app`, then failed before benchmark artifacts were produced:

```text
scripts/smoke/live-webui-feature-sprint.sh: line 262: unexpected EOF while looking for matching `"'
```

That means the prior artifact is failure evidence only, not WebUI/NVIDIA NIM screenshot proof.

## Change made

Replaced `scripts/smoke/live-webui-feature-sprint.sh` with a quote-safe proof harness:

- Adds a self `bash -n` lint pass through `FORGE_LIVE_WEBUI_LINTED`.
- Replaces fragile one-line marker lists with multi-line `for marker in ... do` blocks.
- Keeps the NIM-only checks for provider/model, provider-error rejection, and local/scripted shortcut rejection.
- Keeps the full six-phase natural-language benchmark gate, checker JSON, and browser screenshot artifact generation.
- Adds tool catalog independence guards rejecting `packages/opencode/` and `opencode_` markers in the provider-visible `/api/tools` response.
- Changes the tool-lifecycle proof prompt to Forge identity while retaining behavior-level lifecycle assertions.

## Source backing

Behavior reference used from upstream OpenCode:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts`
  - `updateToolCall`
  - `completeToolCall`
  - `failToolCall`
  - `ensureToolCall`
  - provider-executed ToolPart metadata handling

Forge runtime/proof output should keep these as development references only, not provider-visible app identity.

## Status

- Commit after script rewrite: `bea3ffeab7979b92d10b99add047e421ed3564af`
- Same-head workflows were not complete at the time this note was written.
- Do not claim current-head parity until Live WebUI Feature Sprint produces same-head browser screenshots and checker artifacts.
