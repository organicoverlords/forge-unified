# OpenCode tool lifecycle contract gate — 2026-06-30T00:48Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Selected branch: `mvp/nim-freellmapi-router-20260626`
- PR: #3, open/non-draft/mergeable in live PR metadata before this slice.
- Baseline head before this slice: `484189c945d7b0ec90a70300ef960e868ed9a477`
- Baseline same-head proof before this slice: CI `28411907593`, Build Proof `28411907450`, Fast WebUI Proof `28411907448`, Live WebUI Feature Sprint `28411907443`, App Build Proof `28411907553`, and App Multistep Build Proof `28411907592` were all successful.
- Live WebUI artifact inspected/listed: `7968060176` (`live-webui-feature-sprint-proof`) on head `484189c945d7b0ec90a70300ef960e868ed9a477`.

## Built

Added deterministic CI gate `scripts/smoke/check-opencode-tool-lifecycle-contract.py` and wired it into `.github/workflows/ci.yml`.

The gate checks that Forge's durable tool parts preserve the lifecycle contract required by the WebUI proof path:

- pending tool part
- running tool part
- finished part dispatching to completed/error
- same call IDs across request/result-derived parts
- completed output and duration timing
- error output/error metadata
- file attachments for write/edit/patch results
- retained developer proof anchors for OpenCode `ToolPart`/session lifecycle sources

## OpenCode source backing

- `anomalyco/opencode:packages/opencode/src/session/processor.ts` — pending/running/completed/error tool lifecycle and same-call ToolPart state semantics.
- `anomalyco/opencode:packages/schema/src/v1/session.ts` — session part base, ToolPart, ToolState, and FilePart shape.
- `anomalyco/opencode:packages/web/src/components/share/part.tsx` — browser rendering branches for completed/error tools and per-tool result disclosure.

## Forge source paths guarded

- `crates/engine/src/tool_parts.rs`
- `PROJECT_STATE.md`
- `.github/workflows/ci.yml`

## Claim boundary

This is a source-backed lifecycle/proof gate slice. It does not claim complete OpenCode parity. The new latest head requires same-head CI / Build Proof / Fast WebUI / Live WebUI proof before acceptance.
