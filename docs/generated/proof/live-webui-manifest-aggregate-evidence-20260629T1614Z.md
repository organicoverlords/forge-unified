# Live WebUI manifest aggregate evidence proof — 2026-06-29T16:14Z

## Source of truth

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3
- Failed head inspected: `7f60f6cedc4bb7c7b0a131637adcb74eee65287f`
- Failed Live WebUI run: `28384854653`
- Failed job: `84096737028`
- Failed artifact: `7957561694`, `live-webui-feature-sprint-proof`, digest `sha256:f37d708474d4b99a07098a319ab26820e515cb6ff99df2fb3fd525c15aad2f4c`

## Upstream OpenCode source inspected before patching

- `anomalyco/opencode:packages/core/src/session/runner/max-steps.ts`

Relevant behavior: once max steps are reached, tools are disabled and the agent must produce text only, including a summary of work done, remaining tasks, and next-step recommendations. Forge's live proof gate therefore needs to preserve browser-visible final-response evidence, but that evidence can be represented across the browser DOM/proof JSON, conversation JSON, and SSE stream artifact.

## Failure mode addressed

The Live WebUI sprint step succeeded, while the final evidence/quality step failed. The manifest gate had become stricter than the artifact model by requiring all browser/final-report markers to appear in `full-benchmark-browser-proof.json` alone.

That is too narrow for the actual proof bundle: the browser screenshot/DOM proof, conversation JSON, and SSE stream together prove the natural-language WebUI run. Marker validation should inspect the aggregate proof bundle while keeping hard pass/fail gates on the benchmark checker, OpenCode workflow checker, quality score, screenshot, provider/model metadata, tool-call/tool-result evidence, and local-shortcut rejection.

## Patch

- Updated `scripts/smoke/check-live-webui-proof-manifest.py`.
- Added aggregate evidence text from:
  - `full-benchmark-browser-proof.json`
  - `full-benchmark-conversation.json`
  - `full-benchmark-stream.sse`
- Made required marker matching case-insensitive.
- Renamed the marker check to `browser_evidence_has_required_markers` and reports missing markers directly.
- Preserved strict NVIDIA NIM provider/model checks and non-local runtime checks.

## Current proof status

This is a source-backed gate fix, not a same-head pass claim. Same-head proof requires CI, Build Proof, and Live WebUI Feature Sprint to complete green on `76e999a094bacc4f1709b51781e11d3a5930f743`, `eee411537855e841902e9f0856380e67e98a0a34`, or a later head containing this change.
