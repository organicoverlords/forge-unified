# Formatter activation proof trail gate — 2026-06-29T17:45Z

## Selection

- Repo: `organicoverlords/forge-unified`.
- Selected branch: `mvp/nim-freellmapi-router-20260626`.
- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`.
- PR: #3, open, non-draft, mergeable in metadata before this slice.
- Pre-slice head: `a9517aad7cfa6dfcb6b62794b13529a70f38e5ad`.

## Failed workflow inspected

- CI run: `28390284128`.
- Failed job: `Smoke Test`, job `84115340620`.
- Failed step: `Validate WebUI proof harness`.
- Failure text: `formatter activation evidence check failed` because activation evidence phrases were missing from `PROJECT_STATE.md`.
- Same pre-slice head had Build Proof `28390284139` success and Live WebUI Feature Sprint `28390284127` success.

## Source-backed OpenCode parity slice

Updated `scripts/smoke/check-formatter-activation-evidence.py` so the gate verifies the durable proof trail, not only one volatile state file. The proof trail is `PROJECT_STATE.md` plus `docs/generated/proof/*.md`.

This is tied to OpenCode formatter behavior, not a cosmetic docs update. The gate continues to require Forge runtime source evidence in `crates/engine/src/tool/file_ops.rs` and still fails if Forge runtime metadata contains `opencode_*` keys.

## OpenCode source backing

- `anomalyco/opencode:packages/opencode/src/format/index.ts`
  - Formatter service lifecycle.
  - `config.get()` / `cfg.formatter` activation control.
  - command caching via `commands[item.name]`.
  - extension matching and `Format.status` shape.
  - contained formatter spawn/status failures.
- `anomalyco/opencode:packages/opencode/src/format/formatter.ts`
  - built-in formatter catalog.
  - representative extensions.
  - command semantics.
  - `Filesystem.findUp` and dependency/config-aware enablement for formatters such as prettier, biome, clang-format, ruff, ocamlformat, and pint.
  - plain command probing for formatters such as gofmt, mix, zig, ktlint, dart, shfmt, nixfmt, rustfmt, ormolu, cljfmt, and dfmt.

## Required formatter activation evidence phrases

- configuration/dependency-aware formatter activation.
- formatter service, extension matching, command probing/caching, contained formatter execution, status shape, and configuration/dependency-aware formatter activation.
- built-in formatter catalog, representative extensions, command semantics, and config/dependency-aware formatter enablement.

## Expected proof behavior

- CI Smoke Test should pass the formatter activation proof gate once rerun on the post-slice head.
- Build Proof and Live WebUI Feature Sprint still need same-head success before the latest head is considered proven.
- No full OpenCode parity claim is made here.
