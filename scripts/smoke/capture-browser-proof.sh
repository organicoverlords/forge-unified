#!/usr/bin/env bash
set -euo pipefail
BASE="$1"
CONV_ID="$2"
MODEL_ID="$3"
PROOF_DIR="$4"
MODE="${5:-model}"
QUERY_SUFFIX="${6:-}"
BROWSER_JSON="$PROOF_DIR/browser-proof.json"
EVENT_JSON="$PROOF_DIR/event-page-proof.json"
WEBUI_PNG="$PROOF_DIR/webui.png"
EVENT_PNG="$PROOF_DIR/event-rail.png"

curl -fsS --connect-timeout 2 --max-time 60 -X POST "$BASE/api/browser-proof" -H 'content-type: application/json' -d "{\"url\":\"$BASE/?conversation=$CONV_ID$QUERY_SUFFIX\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" > "$BROWSER_JSON"
jq -e '.success == true' "$BROWSER_JSON" >/dev/null
jq -r '.screenshot_base64' "$BROWSER_JSON" | base64 -d > "$WEBUI_PNG"
test -s "$WEBUI_PNG"
if [ "$MODE" = "model" ]; then
  for marker in "live-browser-model-proof" "provider-model-visible" "provider: nvidia_nim" "MODEL ROUTE" "LIVE_NIM_BROWSER_PROOF" "$MODEL_ID"; do grep -Fq "$marker" "$BROWSER_JSON"; done
fi
if printf '%s' "$QUERY_SUFFIX" | grep -Fq 'proof=final'; then
  for marker in "Run proof summary" "Final answer" "actions used" "proof-digest-visible" "human-tool-label"; do grep -Fq "$marker" "$BROWSER_JSON"; done
fi
if [ "$MODE" = "tool" ]; then
  for marker in "Write file" "Edit file" "Delete file" "technical details" "human-tool-label"; do grep -Fq "$marker" "$BROWSER_JSON"; done
fi

if curl -fsS --connect-timeout 2 --max-time 60 -X POST "$BASE/api/browser-proof" -H 'content-type: application/json' -d "{\"url\":\"$BASE/events?static=1\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" > "$EVENT_JSON"; then
  if jq -e '.success == true' "$EVENT_JSON" >/dev/null; then
    jq -r '.screenshot_base64' "$EVENT_JSON" | base64 -d > "$EVENT_PNG"
    test -s "$EVENT_PNG" || true
  fi
fi
if ! test -s "$EVENT_PNG"; then
  printf '{"success":false,"note":"secondary event rail screenshot unavailable; primary WebUI screenshot passed"}\n' > "$EVENT_JSON"
fi
