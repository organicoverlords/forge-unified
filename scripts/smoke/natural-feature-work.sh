#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_NATURAL_PORT:-3330}"
BASE="http://127.0.0.1:${PORT}"
OUT_DIR="${FORGE_NATURAL_OUT:-$ROOT/.forge-proof/natural-feature-work}"
PROMPT="${FORGE_NATURAL_PROMPT:-Inspect the current app through the normal chat interface and propose the smallest next feature slice that makes model fallback routing visible and useful in the WebUI. Keep the answer practical and implementation-focused.}"
TITLE="${FORGE_NATURAL_TITLE:-natural feature work}"
mkdir -p "$OUT_DIR"

SERVER_LOG="$OUT_DIR/server.log"
STREAM_OUT="$OUT_DIR/chat-stream.sse"
CONVERSATION_JSON="$OUT_DIR/conversation.json"
PROOF_JSON="$OUT_DIR/browser-proof.json"
SCREENSHOT="$OUT_DIR/webui.png"
SUMMARY="$OUT_DIR/summary.json"

cargo build --workspace

cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; }
trap cleanup EXIT

for _ in $(seq 1 80); do
  curl -fsS "$BASE/api/health" >/dev/null && break
  sleep 0.5
done

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  -d "$(jq -n --arg title "$TITLE" '{title:$title}')" \
  | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"

test -n "$CONV_ID"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  -d "$(jq -n --arg message "$PROMPT" '{message:$message,max_rounds:3}')" \
  > "$STREAM_OUT" || true

grep -q "event: run-start" "$STREAM_OUT"
grep -q "event:" "$STREAM_OUT"

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"

curl -fsS -X POST "$BASE/api/browser-proof" \
  -H 'content-type: application/json' \
  -d "$(jq -n --arg url "$BASE/" '{url:$url,width:1440,height:1000,capture_dom:true}')" \
  > "$PROOF_JSON" || true

if jq -e '.success == true and (.screenshot_base64 | length > 1000)' "$PROOF_JSON" >/dev/null 2>&1; then
  jq -r '.screenshot_base64' "$PROOF_JSON" | base64 -d > "$SCREENSHOT" || true
fi

jq -n \
  --arg base "$BASE" \
  --arg conversation_id "$CONV_ID" \
  --arg prompt "$PROMPT" \
  --arg out_dir "$OUT_DIR" \
  --arg screenshot "$SCREENSHOT" \
  '{base:$base, conversation_id:$conversation_id, prompt:$prompt, out_dir:$out_dir, screenshot:$screenshot, normal_webui_path:true}' \
  > "$SUMMARY"

cat "$SUMMARY"
