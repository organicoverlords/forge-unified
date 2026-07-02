# Session-control source focus keyboard proof

Date: 2026-07-02
Branch: `mvp/nim-freellmapi-router-20260626`
Slice commit: `781acf8e57351f84499ef8df156085579fec1b92`

## Selection basis

- Source-of-truth URL branch remains `mvp/nim-freellmapi-router-20260626`.
- PR #3 remains open, non-draft, mergeable, and is still the meaningful app-change PR for this branch.
- Previous head `473f2986676feecfd0ddfebc6c0cd4b646756aae` was inspected as same-head green before this slice.

## Previous same-head proof inspected

Previous head `473f2986676feecfd0ddfebc6c0cd4b646756aae` had all required workflows complete successfully:

- CI: `28614040714`
- Build Proof: `28614040665`
- Fast WebUI Proof: `28614040656`
- Live WebUI Feature Sprint: `28614040690`
- App Build Proof: `28614040694`
- App Multistep Build Proof: `28614040669`

Live WebUI proof details:

- Run: `28614040690`
- Job: `84853287573`
- Artifact: `8049090416` (`live-webui-feature-sprint-proof`)
- Artifact digest: `sha256:a2ecd6e6cd8e9eaba92f8b7eba61bae212e916393000fe955423e6f9198f03c4`
- The job steps included `Run natural feature-build prompt through WebUI`, `Check full benchmark evidence and quality score`, and `Upload feature sprint proof`.

## OpenCode source backing

Exact upstream source path used:

- `anomalyco/opencode:packages/session-ui/src/components/session-turn.tsx`

Relevant OpenCode shapes used as source backing:

- `createAutoScroll`, `autoScroll.handleInteraction`, and `toggleAll()` calling `autoScroll.pause()` for explicit user interaction before view/focus changes.
- `showAll`, `overflow`, and `visible` for explicit user-controlled traversal state.
- `StickyAccordionHeader` and `Accordion.Trigger` for persistent focused controls.
- `data-slot="session-turn-diff-trigger"`, `data-slot="session-turn-diff-path"`, and `data-slot="session-turn-diff-meta"` for path/meta/action affordances.
- `showAssistantCopyPartID` and `assistantCopyPartID` for stable copy-target behavior.

## Forge implementation

Updated:

- `crates/webui/src/chat_ui_session_control_source_focus_trail.html`

Feature added:

- Added keyboard command map to the sticky source-focus trail:
  - `[` = previous receipt in same source/action group
  - `]` = next receipt in same source/action group
  - `f` = pause/resume focus follow
  - `c` = copy focus context
  - `r` = return to selected receipt panel
- Commands are disabled while typing in inputs, textareas, selects, contenteditable fields, or role=textbox surfaces.
- Commands reuse the existing visible buttons via `data-source-focus-command`, so keyboard and click paths stay aligned.
- Copy still pauses live focus before copying, preserving the anti-drift inspection guard.
- Added browser proof hooks: `session-control-source-focus-keyboard`, `session-control-source-focus-command-map`, and `opencode-session-turn-keyboard-interaction-shape`.

## Claim boundary

This commit updates real WebUI behavior, but this exact new head is not yet same-head browser/NIM proven. Do not claim parity for the new keyboard slice until the same-head CI, Build Proof, Fast WebUI Proof, Live WebUI Feature Sprint, App Build Proof, and App Multistep Build Proof runs complete successfully and the Live WebUI artifact/screenshots are inspected.
