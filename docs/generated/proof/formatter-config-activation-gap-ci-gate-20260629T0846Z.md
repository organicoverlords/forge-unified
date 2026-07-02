# Formatter config/dependency activation gap guard — 2026-06-29T08:46Z

Repo: `organicoverlords/forge-unified`
Branch: `mvp/nim-freellmapi-router-20260626`

## Source-of-truth selection

The target URL points to `mvp/nim-freellmapi-router-20260626`. PR #3 remains the active open PR for this workstream; no newer open PR was found that supersedes this branch.

## OpenCode source backing

Inspected upstream OpenCode source paths:

- `anomalyco/opencode:packages/opencode/src/format/index.ts`
- `anomalyco/opencode:packages/opencode/src/format/formatter.ts`

Relevant upstream behaviors recorded from those files:

- Formatter enablement is not only extension matching.
- `prettier` activation checks `package.json` dependencies/devDependencies before resolving the npm binary.
- `biome` activation checks `biome.json` / `biome.jsonc` before resolving the npm binary.
- `ruff` activation checks `pyproject.toml`, `ruff.toml`, `.ruff.toml`, and Python dependency files.
- `clang-format` activation checks `.clang-format`.
- `ocamlformat` activation checks `.ocamlformat`.
- `pint` activation checks `composer.json` for `laravel/pint`.

## Slice built

Added a deterministic CI smoke guard:

- `scripts/smoke/check-formatter-config-activation-gap.py`
- `.github/workflows/ci.yml` now compiles and runs the guard in the smoke-test job.

The guard keeps the current limitation explicit: source-backed formatter catalog support exists, but config/dependency-aware formatter activation is still a known runtime gap until probes such as `package.json`, `pyproject.toml`, `biome.json`, `composer.json`, `.clang-format`, `.ocamlformat`, or equivalent activation probes exist in `crates/engine/src/tool/file_ops.rs`.

## Why this is not docs-only

This changes executable CI behavior. A future proof/state edit that overstates formatter activation support before the runtime probes exist will fail CI.

## Not claimed

This does not implement config/dependency-aware formatter activation. The next real runtime slice is still to add those probes to Forge-owned file formatting behavior while keeping runtime metadata independent from upstream OpenCode names.
