# Live benchmark quality label matching — 2026-06-29T15:15Z

## Repository

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3
- Source-fix head: `2d7f91619e700713d4554b967b6ccf47c70c3964`
- State/doc head after this proof note: `775ca107d1fcebf89727e91a16f09d5a60d7adfd` or later

## Failed run inspected first

- Prior head: `65fee6348197ee973af21809e97f9d1cc5cb966e`
- CI `28381036299`: success
- Build Proof `28381036315`: success
- Live WebUI Feature Sprint `28381036286`: failure
- Live artifact: `7955936640`, `live-webui-feature-sprint-proof`, digest `sha256:59d7889e98b26db786ccd9261c0b6d4fe9144270595cc1dea0a2e08838a32c4f`
- Failed job: `84083393327`

## OpenCode source inspected before patch

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant upstream behavior: when maximum steps are reached, tools are disabled and the assistant must provide text only, summarizing work done so far, remaining tasks, and next recommendations. The source constrains behavior and content semantics, not exact heading capitalization.

## Source-backed patch

Updated `scripts/smoke/score-live-benchmark-quality.py`:

- final-report label scoring now matches required labels case-insensitively;
- score evidence records `matching: case_insensitive`;
- hard checkers remain strict for actual evidence, tool use, NIM route, browser proof, and command provenance.

## Status

This head is not yet same-head proven. Required proof remains CI, Build Proof, and Live WebUI Feature Sprint green on `775ca107d1fcebf89727e91a16f09d5a60d7adfd` or a later head containing this fix.
