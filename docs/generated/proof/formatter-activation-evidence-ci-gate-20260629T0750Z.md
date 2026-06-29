# Formatter activation evidence CI gate proof

Date: 2026-06-29

Selected workstream: `organicoverlords/forge-unified` PR #3, branch `mvp/nim-freellmapi-router-20260626`.

Baseline accepted same-head proof before this slice:

- Head: `5e528fe4aa147b565327a7dc3c6ee4ba930a05c9`
- CI: `28354019180` success
- Build Proof: `28354019177` success, artifact `7944553884`
- Live WebUI Feature Sprint: `28354019207` success, artifact `7944644902`

## Source-backed slice

Added `scripts/smoke/check-formatter-activation-evidence.py` and wired it into `.github/workflows/ci.yml` smoke validation. The checker keeps the OpenCode formatter activation source anchors present in the Forge proof trail while verifying that runtime formatter metadata in `crates/engine/src/tool/file_ops.rs` remains Forge-owned and does not reintroduce `opencode_*` keys.

This is not a runtime formatter activation implementation claim. It is a CI proof guard for the activation evidence trail, so future formatter runtime activation work cannot silently drop the upstream-backed config/dependency semantics from state/proof before code parity is implemented.

## OpenCode source paths inspected

- `anomalyco/opencode:packages/opencode/src/format/index.ts`
  - `config.get()` and `cfg.formatter` control whether built-in formatter entries are installed or disabled.
  - `mergeDeep` merges custom formatter config into built-in formatter definitions.
  - `Format.status()` reports formatter names, extensions, and enabled state.
  - `formatFile()` gathers enabled formatter commands by extension and contains formatter spawn/nonzero failures.
- `anomalyco/opencode:packages/opencode/src/format/formatter.ts`
  - Formatter activation uses command discovery and, for several formatters, config/dependency probes via `Filesystem.findUp` and package/config file inspection.
  - Representative activation rules include Prettier/package.json deps, Biome config files, Ruff config/deps, Clang `.clang-format`, OCaml `.ocamlformat`, Pint `composer.json`, and experimental Oxfmt gating.

## Files changed

- `.github/workflows/ci.yml`
- `scripts/smoke/check-formatter-activation-evidence.py`
- `PROJECT_STATE.md`
- `docs/generated/proof/formatter-activation-evidence-ci-gate-20260629T0750Z.md`

## Proof status

The new head created by this slice is not same-head proven until CI, Build Proof, and Live WebUI Feature Sprint complete successfully and produce the expected artifacts. Do not claim latest-head WebUI/NVIDIA NIM parity from this doc alone.
