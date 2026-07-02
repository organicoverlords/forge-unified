# Proof-final raw JSON DOM scrub

Date: 2026-06-29
Branch: `mvp/nim-freellmapi-router-20260626`

## Selection

Selected PR #3 / `mvp/nim-freellmapi-router-20260626` because it remains the newest meaningful open PR for `organicoverlords/forge-unified` and targets `master`.

## Failure inspected

Fast WebUI Proof run `28401287933`, job `84152913786`, used real NVIDIA NIM evidence:

- provider: `nvidia_nim`
- model: `deepseek-ai/deepseek-v4-flash`
- screenshot path: `forge-proof/fast-webui-proof/webui.png`
- artifact ID: `7963968150`

The run passed provider/model/run/screenshot/readability checks but failed `raw_json_not_primary_result` because proof DOM still contained implementation-first raw text / JSON (`raw tool:` and escaped JSON in hidden technical details).

## OpenCode source backing

Exact upstream source paths used:

- `anomalyco/opencode:packages/web/src/components/share/part.tsx`
- `anomalyco/opencode:packages/web/src/components/share/part.module.css`

Relevant OpenCode behavior: completed tool parts are rendered through tool-specific UI components such as `GrepTool`, `GlobTool`, `ReadTool`, `WriteTool`, `EditTool`, `BashTool`, `TodoWriteTool`, `WebFetchTool`, and `TaskTool`, with a footer/status presentation. The user-facing proof surface is componentized and readable instead of exposing raw tool JSON as the primary result.

## Forge change

Updated `crates/webui/src/chat_ui.rs`:

- replaces `raw tool: <name>` with human-readable provider-executed action text;
- keeps readable tool labels and result summaries visible;
- only attaches technical `<details><pre>...</pre></details>` outside `proof=final` mode;
- avoids embedding hidden escaped JSON in final browser proof DOM;
- keeps final proof markers: `Run proof summary`, `Final answer`, `human-tool-label`, `provider-model-visible`, and `proof-digest-visible`.

## Claim boundary

This is a WebUI proof-presentation parity slice. It does not claim full OpenCode UI parity or full app parity. Same-head CI / Build Proof / Live WebUI proof must still pass on the final head before accepting this slice.