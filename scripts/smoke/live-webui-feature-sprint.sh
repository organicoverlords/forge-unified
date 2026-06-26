#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
SERVER_LOG="$PROOF_DIR/server.log"
STATUS_OUT="$PROOF_DIR/git-status.txt"
PROMPT_FILE="$PROOF_DIR/screenshot-prompt.txt"
REQUEST_JSON="$PROOF_DIR/screenshot-request.json"
STREAM_OUT="$PROOF_DIR/screenshot-stream.sse"
CONVERSATION_JSON="$PROOF_DIR/screenshot-conversation.json"
BROWSER_PROOF_JSON="$PROOF_DIR/browser-proof.json"
SCREENSHOT_PNG="$PROOF_DIR/webui.png"
mkdir -p "$PROOF_DIR"

cargo build --workspace
cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() {
  kill "$PID" >/dev/null 2>&1 || true
  git status --short > "$STATUS_OUT" 2>/dev/null || true
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
  -d '{"title":"human readable prompt completed"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$CONV_ID"

cat > "$PROMPT_FILE" <<'PROMPT'
Complete a small repo check and answer like a coding agent final response, not like a test marker. Keep it concise and human-readable.

Include these headings:
Summary
What I checked
Result
Source

In Source, mention packages/opencode/src/session/prompt.ts.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  --data-binary "@$REQUEST_JSON" \
  > "$STREAM_OUT"

grep -q "event: run-start" "$STREAM_OUT"
grep -q "event: run-finish" "$STREAM_OUT"
grep -q "Summary" "$STREAM_OUT"
grep -q "What I checked" "$STREAM_OUT"
grep -q "Result" "$STREAM_OUT"
grep -q "Source" "$STREAM_OUT"
if grep -qi "provider-error\|missing_key\|runtime is missing" "$STREAM_OUT"; then
  echo "::error::Screenshot prompt produced provider-error or missing-key output."
  exit 3
fi

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
grep -q "Summary" "$CONVERSATION_JSON"
grep -q "What I checked" "$CONVERSATION_JSON"
grep -q "Result" "$CONVERSATION_JSON"
grep -q "Source" "$CONVERSATION_JSON"

curl -fsS -X POST "$BASE/api/browser-proof" \
  -H 'content-type: application/json' \
  -d "{\"url\":\"$BASE/\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" \
  > "$BROWSER_PROOF_JSON"

jq -e '.success == true' "$BROWSER_PROOF_JSON" >/dev/null
jq -r '.screenshot_base64' "$BROWSER_PROOF_JSON" | base64 -d > "$SCREENSHOT_PNG"
test -s "$SCREENSHOT_PNG"
grep -q "human readable prompt completed" "$BROWSER_PROOF_JSON"
grep -q "Summary" "$BROWSER_PROOF_JSON"
grep -q "What I checked" "$BROWSER_PROOF_JSON"
grep -q "Result" "$BROWSER_PROOF_JSON"
grep -q "Source" "$BROWSER_PROOF_JSON"

echo "LIVE WebUI human-readable completed-prompt screenshot proof passed: $BASE conversation=$CONV_ID screenshot=$SCREENSHOT_PNG"