#!/usr/bin/env bash
set -euo pipefail

cat >&2 <<'MSG'
::error::REJECTED / NOT PROVEN: live-webui-feature-sprint must be restored as a real browser proof.
Required proof:
- start Forge WebUI
- send a normal chat prompt through /api/conversations/:id/chat/stream
- reject provider local, local_shortcut, event: benchmark-phase, and natural_language_webui_agent_benchmark
- require run-finish provider nvidia_nim and a non-empty model
- capture webui.png from /?conversation=<id>
- require screenshot DOM to show provider: nvidia_nim, model:<actual model>, MODEL ROUTE, and the assistant response
- upload webui.png and event-rail.png
MSG
exit 1
