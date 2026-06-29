# Runtime file tool metadata independence — 2026-06-29T02:50Z

## Selection

- Repo: `organicoverlords/forge-unified`
- Branch: `mvp/nim-freellmapi-router-20260626`
- Base checkpoint: PR #3 head `c12789a7b7c59ba7bfe0ba22118892396356fc7c`
- Prior same-head workflows for that checkpoint: CI `28343848325`, Build Proof `28343848326`, Live WebUI Feature Sprint `28343848306`.
- Prior Live WebUI artifact: `7941120525`, digest `sha256:7f9758084a3701b759191daad22b04fae6eed8b259429be665a174e5aaa9c5c5`.

## Source backing inspected

Reference source paths used:

- `anomalyco/opencode:packages/opencode/src/tool/apply_patch.ts`
- `anomalyco/opencode:packages/opencode/src/tool/write.ts`
- `anomalyco/opencode:packages/opencode/src/tool/edit.ts`
- `anomalyco/opencode:packages/opencode/src/tool/read.ts`
- `anomalyco/opencode:packages/opencode/src/lsp/lsp.ts`
- `anomalyco/opencode:packages/opencode/src/lsp/diagnostic.ts`
- `anomalyco/opencode:packages/opencode/src/event-v2-bridge.ts`
- `anomalyco/opencode:packages/opencode/src/server/routes/instance/httpapi/handlers/event.ts`
- `anomalyco/opencode:packages/core/src/file-mutation.ts`

## Slice built

Runtime file-tool metadata was changed so Forge keeps reference paths in proof/docs only, not provider-facing or WebUI runtime tool metadata.

Changed code:

- `crates/engine/src/tool/file_ops.rs`: replaced upstream-branded runtime metadata with Forge-owned `forge_formatter_contract` and `forge_file_tool_contract` while keeping BOM preservation, formatter containment, file events, watcher updates, LSP diagnostic metadata, and stale-edit recovery.
- `crates/engine/src/tool/patch_events.rs`: replaced upstream-branded nested event keys and source strings with Forge-owned event/watcher/LSP contracts; added `forge_event_contract()`; updated tests.

## Proof status

- Code commits: `814ea07169997e9412e18538d32d809eb3dfee93`, `5e3ac6be0259bce666383dd231e1b06fcee9b413`.
- Same-head CI / Build Proof / Live WebUI Feature Sprint were not yet returned by the workflow API when this note was written.

## Do not overclaim

Do not claim this latest head is same-head WebUI/NIM proven until new workflow runs finish green and upload a Live WebUI artifact. Prior accepted proof remains PR head `c12789a7b7c59ba7bfe0ba22118892396356fc7c`, artifact `7941120525`.
