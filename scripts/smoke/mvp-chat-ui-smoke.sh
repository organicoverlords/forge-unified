#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_SMOKE_PORT:-3310}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_SMOKE_PROOF_DIR:-$ROOT/.forge-proof/webui-smoke}"
mkdir -p "$PROOF_DIR"
LOG="$PROOF_DIR/server.log"
STREAM_OUT="$PROOF_DIR/chat-stream.sse"
PROOF_JSON="$PROOF_DIR/browser-proof.json"
PROOF_PNG="$PROOF_DIR/webui.png"

cargo build --workspace

cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$LOG" 2>&1 &
PID=$!
cleanup() {
  kill "$PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT

for _ in $(seq 1 60); do
  if curl -fsS "$BASE/api/health" >/dev/null; then
    break
  fi
  sleep 0.5
done

curl -fsS "$BASE/" | grep -q "Forge Unified"
curl -fsS "$BASE/api/health" | grep -q '"status":"ok"'

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  -d '{"title":"natural prompt smoke"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"

test -n "$CONV_ID"
curl -fsS "$BASE/api/conversations/$CONV_ID" | grep -q "natural prompt smoke"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  -d '{"message":"Using only the normal chat interface, inspect the current app state and propose the smallest next feature needed to make fallback routing visible in the WebUI.","max_rounds":1}' \
  > "$STREAM_OUT" || true

grep -q "event: run-start" "$STREAM_OUT"
grep -q "event:" "$STREAM_OUT"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/snapshot" \
  -H 'content-type: application/json' \
  -d '{}' | grep -q "snapshot_saved"

curl -fsS -X POST "$BASE/api/browser-proof" \
  -H 'content-type: application/json' \
  -d "{\"url\":\"$BASE/\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" \
  > "$PROOF_JSON" || true

if jq -e '.success == true' "$PROOF_JSON" >/dev/null 2>&1; then
  jq -r '.screenshot_base64' "$PROOF_JSON" | base64 -d > "$PROOF_PNG" || true
fi

echo "MVP chat UI + SSE smoke passed: $BASE conversation=$CONV_ID proof_dir=$PROOF_DIR"
