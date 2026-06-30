# Full benchmark cargo-claim negation checker repair

Date: 2026-06-30
Branch: `mvp/nim-freellmapi-router-20260626`
Commit: `9a7020a206ec8387ac7e5eaedb79ab6d5345398d`

## Failure inspected

Latest head `545fec357de85718151c4b726a897092bfa5bcca` had five green workflows and one failed workflow:

- Live WebUI Feature Sprint `28464823475` failed.
- Artifact `7989433741` was downloaded and inspected.
- `full-benchmark-checker.json` had one failed check: `claimed_cargo_tests_are_tool_proven`.

## Root cause

The benchmark final answer correctly said:

- `UNKNOWN: Whether the full test suite passes — no cargo test was executed`

The checker treated the word `passes` plus Markdown-code-formatted `cargo test` as a success claim because the negation filter did not normalize Markdown punctuation before matching phrases such as `no cargo test`.

## Fix

Updated `scripts/smoke/check-full-agentic-benchmark.py`:

- Added `prose_for_claim_detection()` to strip Markdown/code punctuation before claim detection.
- Expanded negation phrases for exact cargo commands.
- Moved the broad `all tests pass` pattern to per-line detection after negation filtering.
- Preserved strict behavior: explicit non-negated `cargo test` success claims still require matching ShellCommand evidence.

## Local proof

The patched checker was run against the failed artifact before commit:

```text
python -m py_compile check-full-agentic-benchmark.py
python check-full-agentic-benchmark.py full-benchmark-conversation.json full-benchmark-stream.sse check-out.json
```

Result:

```json
{"passed": true, "failed_checks": []}
```

## Claim boundary

This repair fixes a deterministic checker false positive. Same-head workflows still need to complete on the new head before acceptance.
