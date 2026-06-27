#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
mkdir -p "$PROOF_DIR"
SERVER_LOG="$PROOF_DIR/server.log"
STATUS_OUT="$PROOF_DIR/git-status.txt"
CONVERSATION_JSON="$PROOF_DIR/live-model-conversation.json"
MODEL_STREAM="$PROOF_DIR/live-model-stream.sse"
PROMPT_FILE="$PROOF_DIR/live-model-prompt.txt"
REQUEST_JSON="$PROOF_DIR/live-model-request.json"

cargo build --workspace
RUST_BACKTRACE=1 cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; git status --short > "$STATUS_OUT" 2>/dev/null || true; }
trap cleanup EXIT

for attempt in $(seq 1 120); do
  if ! kill -0 "$PID" >/dev/null 2>&1; then tail -n 220 "$SERVER_LOG" >&2 || true; exit 1; fi
  if curl -fsS --connect-timeout 2 --max-time 10 "$BASE/api/health" -o "$PROOF_DIR/health.json"; then break; fi
  sleep 1
done
curl -fsS "$BASE/" -o "$PROOF_DIR/index.html"
grep -Fq "Forge Unified" "$PROOF_DIR/index.html"
grep -Fq "provider-model-visible" "$PROOF_DIR/index.html"
grep -Fq "live-browser-model-proof" "$PROOF_DIR/index.html"

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" -H 'content-type: application/json' -d '{"title":"live NIM browser model proof"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$CONV_ID"
cat > "$PROMPT_FILE" <<'PROMPT'
Reply with LIVE_NIM_BROWSER_PROOF and one short sentence.
PROMPT
jq -Rs '{message: ., max_rounds: 2}' "$PROMPT_FILE" > "$REQUEST_JSON"
curl -fsS --connect-timeout 2 --max-time 240 -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$REQUEST_JSON" > "$MODEL_STREAM"

if grep -Fq '"provider":"local"' "$MODEL_STREAM" || grep -Fq 'event: benchmark-phase' "$MODEL_STREAM" || grep -Fq 'local_shortcut' "$MODEL_STREAM"; then
  echo "::error::local or scripted proof path detected" >&2
  tail -n 160 "$MODEL_STREAM" >&2 || true
  exit 8
fi
if grep -Fq 'event: provider-error' "$MODEL_STREAM"; then
  echo "::error::provider error during live model proof" >&2
  tail -n 160 "$MODEL_STREAM" >&2 || true
  exit 9
fi
grep -Fq 'event: run-finish' "$MODEL_STREAM"
grep -Fq 'event: text-delta' "$MODEL_STREAM"
grep -Fq '"provider":"nvidia_nim"' "$MODEL_STREAM"
grep -Fq '"model":"' "$MODEL_STREAM"
grep -Fq 'LIVE_NIM_BROWSER_PROOF' "$MODEL_STREAM"

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
jq -e '.provider == "nvidia_nim" and (.model | type == "string" and length > 0) and (.messages | length >= 2)' "$CONVERSATION_JSON" >/dev/null
MODEL_ID="$(jq -r '.model' "$CONVERSATION_JSON")"
bash scripts/smoke/capture-browser-proof.sh "$BASE" "$CONV_ID" "$MODEL_ID" "$PROOF_DIR"
echo "conversation=$CONV_ID model=$MODEL_ID screenshot=$PROOF_DIR/webui.png event_rail=$PROOF_DIR/event-rail.png" > "$PROOF_DIR/live-proof-status.txt"
echo "LIVE model-backed browser proof passed: $BASE conversation=$CONV_ID model=$MODEL_ID"
