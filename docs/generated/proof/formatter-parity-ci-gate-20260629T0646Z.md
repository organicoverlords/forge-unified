# Formatter parity CI gate proof — 2026-06-29T06:46Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft/mergeable at inspection time
- Starting head: `567f42ac4cbfa99705ddeea81276990a6ca79ff5`
- Same-head proof already cleared for starting head:
  - CI: `28351599331` success
  - Build Proof: `28351599361` success
  - Live WebUI Feature Sprint: `28351599332` success after rerun
  - Live WebUI artifact: `7944041593`, `live-webui-feature-sprint-proof`, digest `sha256:3f2adbc9bb2d08a6af83825af4b3a73d670238b725727999a4cfd33348759ce3`
  - Build Proof artifact: `7943661980`, `build-proof`, digest `sha256:a794a45fc73556c0da69f178aa94d7db19f4887ae9f670474a06643a05f3b200`

## Source backing

Inspected upstream OpenCode sources:

- `anomalyco/opencode:packages/opencode/src/format/index.ts`
  - Formatter service lifecycle
  - Extension matching
  - Command probing/caching
  - Contained formatter execution and failed-format logging
  - `Format.status()` shape
- `anomalyco/opencode:packages/opencode/src/format/formatter.ts`
  - Built-in formatter families and command semantics
  - Config/dependency-aware activation examples for Prettier, Biome, Ruff, uv, clang-format, OCaml, Pint, and oxfmt

## Implemented slice

Added a deterministic CI checker:

- New file: `scripts/smoke/check-formatter-parity.py`
- CI integration: `.github/workflows/ci.yml` smoke-test step now compiles and runs it.

The checker gates the Forge formatter runtime against the recorded source-backed contract by verifying:

- All upstream formatter families copied into `crates/engine/src/tool/file_ops.rs` remain present.
- Representative extensions for each formatter family remain mapped.
- The Forge formatter contract still records the exact OpenCode source paths used as developer/proof anchors.
- Formatter unavailability, non-zero formatter exits, and spawn failures stay contained in metadata.
- Runtime formatter metadata does not reintroduce `opencode_*` keys.

## Changed files

- `scripts/smoke/check-formatter-parity.py`
- `.github/workflows/ci.yml`
- `docs/generated/proof/formatter-parity-ci-gate-20260629T0646Z.md`

## Proof status

This commit adds a hard CI regression gate. It does not by itself prove a new WebUI/NIM run.

Latest accepted same-head WebUI/NIM proof before this slice remains the starting head `567f42ac4cbfa99705ddeea81276990a6ca79ff5` until CI / Build Proof / Live WebUI Feature Sprint complete on the newer formatter-parity-gate head.
