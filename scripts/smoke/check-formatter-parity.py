#!/usr/bin/env python3
"""Guard Forge formatter parity against the source-backed OpenCode formatter catalog.

This checker is intentionally deterministic: it does not execute external formatters.
It validates that Forge's file write/edit formatter metadata preserves the exact
source-backed formatter families and containment contract copied from OpenCode's
format service/catalog.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FILE_OPS = ROOT / "crates" / "engine" / "src" / "tool" / "file_ops.rs"

REQUIRED_FORMATTERS = {
    "rustfmt": ["rs"],
    "gofmt": ["go"],
    "mix": ["ex", "exs", "eex", "heex", "leex", "neex", "sface"],
    "prettier": ["js", "jsx", "ts", "tsx", "json", "yaml", "md", "graphql"],
    "oxfmt": ["js", "jsx", "ts", "tsx"],
    "biome": ["js", "jsx", "ts", "tsx", "json", "yaml", "md", "graphql"],
    "ruff": ["py", "pyi"],
    "uv": ["py", "pyi"],
    "clang-format": ["c", "cc", "cpp", "h", "hpp"],
    "shfmt": ["sh", "bash"],
    "terraform": ["tf", "tfvars"],
    "zig": ["zig", "zon"],
    "dart": ["dart"],
    "ktlint": ["kt", "kts"],
    "rubocop": ["rb", "rake", "gemspec", "ru"],
    "standardrb": ["rb", "rake", "gemspec", "ru"],
    "htmlbeautifier": ["erb", "html.erb"],
    "ocamlformat": ["ml", "mli"],
    "latexindent": ["tex"],
    "gleam": ["gleam"],
    "nixfmt": ["nix"],
    "air": ["R"],
    "pint": ["php"],
    "ormolu": ["hs"],
    "cljfmt": ["clj", "cljs", "cljc", "edn"],
    "dfmt": ["d"],
}

REQUIRED_CONTRACT_SNIPPETS = [
    "packages/opencode/src/format/index.ts",
    "packages/opencode/src/format/formatter.ts",
    "probe matching formatter commands and safely disable unavailable commands",
    "contain formatter spawn/status failures in metadata instead of failing the file tool",
    "resynchronize desired UTF-8 BOM after formatter mutation",
    "formatter_unavailable",
    "formatter_failed_contained",
    "spawn_failed_contained",
]


def main() -> int:
    text = FILE_OPS.read_text(encoding="utf-8")
    errors: list[str] = []

    for name, extensions in REQUIRED_FORMATTERS.items():
        if f'name: "{name}"' not in text:
            errors.append(f"missing formatter family: {name}")
            continue
        spec_match = re.search(
            rf'FormatterSpec \{{ name: "{re.escape(name)}".*?extensions: &\[(.*?)\] \}},',
            text,
        )
        if not spec_match:
            errors.append(f"could not parse formatter spec: {name}")
            continue
        spec_body = spec_match.group(1)
        for ext in extensions:
            if f'"{ext}"' not in spec_body:
                errors.append(f"formatter {name} missing extension {ext}")

    for snippet in REQUIRED_CONTRACT_SNIPPETS:
        if snippet not in text:
            errors.append(f"missing formatter contract/evidence snippet: {snippet}")

    if "opencode_" in text:
        errors.append("runtime formatter metadata must not contain opencode_* keys")

    if errors:
        print("formatter parity check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(
        "formatter parity check passed: Forge file write/edit formatter catalog still matches "
        "the recorded OpenCode formatter source families and containment contract."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
