# Live benchmark answer contract repair — 2026-06-28T14:46Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open and non-draft
- Source-of-truth URL: `https://github.com/organicoverlords/forge-unified/tree/mvp/nim-freellmapi-router-20260626`

## Failure inspected

- Head inspected before this repair: `edf8350e38365332f61f47ab0715c42036b5b3e8`
- Live WebUI Feature Sprint run: `28325465129`
- Job: `83914593454`
- The WebUI/NVIDIA NIM run reached the full benchmark and produced real artifacts.
- Provider/model evidence: `nvidia_nim`, `deepseek-ai/deepseek-v4-flash`.
- Tool evidence was present: 77 `tool-call` events, 71 `tool-result` events, 40 persisted tool results.

## Checker failures that drove this repair

The run failed because the agent answer missed proof-contract details, not because startup or NIM failed:

- Missing explicit `Cargo.toml` file read plus repo map pairing.
- Missing final confidence labels `VERIFIED`, `LIKELY`, and `UNKNOWN`.
- Missing validation command evidence.
- Missing final risk/rollback wording.
- Founder report was 183 words, over the 180-word limit.
- Technical report was missing required sections.
- Final summary labels were incomplete.

## Repair landed

Updated `scripts/smoke/full-agentic-benchmark-prompt.txt` with a proof-critical checklist before the phase instructions. The checklist explicitly requires:

- real `file_read` evidence for `Cargo.toml`;
- repo map evidence from `repo_info` or `file_list`;
- one real validation shell command such as `cargo check`, `cargo test`, `cargo build`, `cargo fmt --check`, `cargo clippy`, or `bash -n`;
- exact final labels required by the checker;
- Founder report below 180 words, aiming under 120 words;
- Technical report sections for evidence, assumptions, failed hypotheses, confidence, and rollback strategy.

This is a proof-facing behavior change, not a docs-only update: it changes the natural-language prompt actually sent through the WebUI during Live WebUI Feature Sprint.

## OpenCode source backing

Reference retained in developer/proof docs only:

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — tool-call lifecycle is organized around explicit `updateToolCall`, `completeToolCall`, and `failToolCall` state transitions, with structured output metadata and attachments.
- Forge mirrors that discipline here by making the benchmark prompt require tool-backed evidence before final reporting instead of allowing final prose to outrun tool state.

## Proof status

Not proven on the repaired head yet. The next required proof is a same-head green Live WebUI Feature Sprint artifact containing `full-benchmark-webui.png`, `full-benchmark-browser-proof.json`, `full-benchmark-stream.sse`, `full-benchmark-conversation.json`, `full-benchmark-checker.json` with `passed: true`, and `opencode-workflow-checker.json` with `passed: true`.
