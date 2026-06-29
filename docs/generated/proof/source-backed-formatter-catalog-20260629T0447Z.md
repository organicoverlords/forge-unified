# Source-backed formatter catalog parity slice — 2026-06-29T04:47Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft/mergeable at run start.
- Baseline latest accepted same-head proof before this slice: `75296872eea904634c0eb565f7fe917de97443a0` with CI `28348220256`, Build Proof `28348220266`, Live WebUI Feature Sprint `28348220251`, artifact `7942541939`.

## Feature built

Expanded Forge file write/edit formatting behavior from Rust-only `rustfmt` support into a source-backed formatter catalog in `crates/engine/src/tool/file_ops.rs`.

The catalog now covers formatter families mirrored from OpenCode formatter sources:

- `rustfmt` for Rust.
- `gofmt` for Go.
- `prettier` and `biome` for common JS/TS/web/data/doc formats.
- `ruff` and `uv` for Python.
- `clang-format` for C/C++ headers/sources.
- `shfmt` for shell.
- `terraform fmt`, `zig fmt`, `dart format`, `ktlint`, `rubocop`, `standardrb`, `htmlbeautifier`, `ocamlformat`, `latexindent`, `gleam format`, and `nixfmt`.

Formatter execution remains contained: unavailable formatter commands and failed formatter exits are recorded in tool metadata and do not make file write/edit fail. BOM resync remains after formatter mutation.

## OpenCode source backing

Exact upstream paths inspected/used:

- `anomalyco/opencode:packages/opencode/src/format/index.ts`
  - Formatter service lifecycle.
  - Extension-to-formatter matching.
  - Command probing/caching.
  - Running all enabled matching formatters while containing failures.
- `anomalyco/opencode:packages/opencode/src/format/formatter.ts`
  - Built-in formatter catalog, extension lists, and command semantics.

## Files changed

- `crates/engine/src/tool/file_ops.rs`
- `docs/generated/proof/source-backed-formatter-catalog-20260629T0447Z.md`

## Proof status

This commit is not same-head WebUI/NIM proven at creation time. Required follow-up proof:

- CI success.
- Build Proof success.
- Live WebUI Feature Sprint success using NVIDIA NIM only.
- Live WebUI artifact with browser screenshots and checker JSON for this new head.
