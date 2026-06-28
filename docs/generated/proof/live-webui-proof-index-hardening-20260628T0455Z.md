# Live WebUI proof index hardening — 2026-06-28T04:55Z

Status: attempted but not landed by connector because the existing failing shell script requires a full-file replacement and the connector blocked the write payload.

Selection:
- Repo: organicoverlords/forge-unified
- Branch: mvp/nim-freellmapi-router-20260626
- PR: #3 open, non-draft
- Inspected HEAD before attempted repair: 3222eed283544541c266aed526089363d97df2d0

Failure inspected:
- Live WebUI Feature Sprint run 28310632058 failed after compiling forge-app.
- Job 83874774660 failed at scripts/smoke/live-webui-feature-sprint.sh with: unexpected EOF while looking for matching double quote.
- The follow-up checker failed because full-benchmark-conversation.json and full-benchmark-stream.sse were not produced.
- Uploaded artifact 7930850569 is failure evidence only, not WebUI/NIM proof.

Intended repair:
- Replace the final fragile one-line proof-status echo with a Python-backed live-proof-index.json writer and multi-line live-proof-status.txt writer.
- Keep NIM-only gates for provider local, local_shortcut, benchmark-phase, and provider-error.
- Preserve browser screenshot requirements for tool-lifecycle-webui.png and full-benchmark-webui.png.

OpenCode source paths used:
- packages/opencode/src/session/processor.ts: Handle.completeToolCall, updateToolCall, failToolCall, ensureToolCall.
- packages/opencode/src/session/processor.ts: toolResultOutput normalized FilePart attachments.
- packages/opencode/src/session/processor.ts: EventV2Bridge-backed workflow/event proof references.

Proof state:
- Same-head live WebUI/NVIDIA NIM proof is not available yet.
- Do not claim parity from this run.
