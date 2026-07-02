# Source-backed formatter catalog completion — 2026-06-29T05:55Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open / non-draft / mergeable at selection time.
- Baseline HEAD before this slice: `caa80651d173c96703dbb5df37d23a2ca1762eca`.
- Baseline same-head proof already accepted in PR body:
  - CI `28349510016`: success.
  - Build Proof `28349509993`: success.
  - Live WebUI Feature Sprint `28349510045`: success.
  - Live WebUI artifact `7942987277`, `live-webui-feature-sprint-proof`, digest `sha256:b38e189f46a037213b7293d9c43e563b10f4bf200e03cdf6df4d5639fd71e660`.

## OpenCode source backing

Exact upstream paths used:

- `anomalyco/opencode:packages/opencode/src/format/index.ts`
  - Formatter service lifecycle.
  - Extension matching.
  - Command discovery/probing.
  - Contained formatter execution and non-fatal formatter failure behavior.
- `anomalyco/opencode:packages/opencode/src/format/formatter.ts`
  - Built-in formatter catalog and command semantics.
  - Added coverage from upstream entries: `mix`, `oxfmt`, `air`, `pint`, `ormolu`, `cljfmt`, `dfmt`.

## Code slice

Updated `crates/engine/src/tool/file_ops.rs` to complete the prior formatter parity slice by adding missing OpenCode formatter families:

- Elixir / Phoenix templates: `mix format` for `ex`, `exs`, `eex`, `heex`, `leex`, `neex`, `sface`.
- Experimental JS/TS formatter: `oxfmt` for JS/TS module extensions.
- R: `air format` for `R`.
- PHP: `./vendor/bin/pint` for `php`.
- Haskell: `ormolu -i` for `hs`.
- Clojure / EDN: `cljfmt fix --quiet` for `clj`, `cljs`, `cljc`, `edn`.
- D: `dfmt -i` for `d`.

Retained behavior:

- Formatter command absence remains contained as `formatter_unavailable` metadata.
- Formatter spawn/status failure remains contained and does not fail the file write/edit tool.
- Formatter mutation is followed by UTF-8 BOM resynchronization.
- Runtime metadata remains Forge-owned; exact upstream source paths are recorded in docs/proof only.

## Proof status

This commit is source-backed and pushed to the PR branch. It is not same-head WebUI/NIM proven until the new GitHub Actions runs complete successfully on the resulting head.
