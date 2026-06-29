# Readable WebUI proof cards — 2026-06-29T19:55Z

User rejected the previous screenshot proof because the process screenshots were hard to read:

- visible cards used raw tool names such as `file_read` and `batch_parallel` as primary labels;
- final benchmark screenshot showed mostly the benchmark prompt instead of proof/final-answer evidence;
- process screenshots looked like raw diagnostic dumps instead of a readable build timeline.

## Fix applied

- `crates/webui/src/chat_ui.rs` now maps raw tool names to human labels such as `Read file`, `Write file`, `Edit file`, `Run command`, `Run tools in parallel`, and `Delegate subtask`.
- Raw tool names remain available only as technical metadata / data attributes for checker compatibility.
- Tool cards now show a clear status badge, concise result text, file chips, and collapsed technical details.
- `proof=final` now renders a top `Run proof summary` panel with provider, model, tool-result count, actions used, files touched/inspected, and the final answer.
- `scripts/smoke/capture-browser-proof.sh` now fails the browser proof if readable proof markers are missing.

## Acceptance boundary

This is a UX/proof-presentation fix. It does not change model routing or tool execution semantics. The head containing this fix still needs same-head CI, Build Proof, and Live WebUI Feature Sprint proof before acceptance.
