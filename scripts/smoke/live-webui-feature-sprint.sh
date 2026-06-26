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
NOTE_PATH="forge-proof/live-webui-feature-sprint/natural-proof-note.txt"
mkdir -p "$PROOF_DIR"
rm -f "$NOTE_PATH" "$STREAM_OUT"

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
curl -fsS "$BASE/" | grep -q "file-change"
curl -fsS "$BASE/" | grep -q "file-card"

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  -d '{"title":"natural file creation and repo inspection proof"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$CONV_ID"

cat > "$PROMPT_FILE" <<'PROMPT'
Please create a short proof note for this WebUI sprint.

Keep the reply brief and tell me what changed.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  --data-binary "@$REQUEST_JSON" \
  > "$STREAM_OUT"

cat > "$PROMPT_FILE" <<'PROMPT'
Please inspect this repository and summarize what you find.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  --data-binary "@$REQUEST_JSON" \
  >> "$STREAM_OUT"

grep -q "event: run-start" "$STREAM_OUT"
grep -q "event: run-finish" "$STREAM_OUT"
grep -q "event: tool-result" "$STREAM_OUT"
grep -q "event: file-change" "$STREAM_OUT"
grep -q "file.added" "$STREAM_OUT"
grep -q "natural-proof-note.txt" "$STREAM_OUT"
grep -q "Created.*natural-proof-note.txt" "$STREAM_OUT"
grep -q "Updated 1 file" "$STREAM_OUT"
grep -q "repo_info" "$STREAM_OUT"
grep -q "file_list" "$STREAM_OUT"
grep -q "Inspected the repository" "$STREAM_OUT"
grep -q "top-level files were listed" "$STREAM_OUT"
if grep -qi "provider-error\|missing_key\|runtime is missing" "$STREAM_OUT"; then
  echo "::error::Natural prompts produced provider-error or missing-key output."
  exit 3
fi

test -s "$NOTE_PATH"
grep -q "Natural prompt completed" "$NOTE_PATH"

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
grep -q "Created.*natural-proof-note.txt" "$CONVERSATION_JSON"
grep -q "Updated 1 file" "$CONVERSATION_JSON"
grep -q "file_events" "$CONVERSATION_JSON"
grep -q "natural-proof-note.txt" "$CONVERSATION_JSON"
grep -q "Inspected the repository" "$CONVERSATION_JSON"
grep -q "repo_info" "$CONVERSATION_JSON"
grep -q "file_list" "$CONVERSATION_JSON"

curl -fsS -X POST "$BASE/api/browser-proof" \
  -H 'content-type: application/json' \
  -d "{\"url\":\"$BASE/\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" \
  > "$BROWSER_PROOF_JSON"

jq -e '.success == true' "$BROWSER_PROOF_JSON" >/dev/null
jq -r '.screenshot_base64' "$BROWSER_PROOF_JSON" | base64 -d > "$SCREENSHOT_PNG"
test -s "$SCREENSHOT_PNG"
grep -q "natural file creation and repo inspection proof" "$BROWSER_PROOF_JSON"
grep -q "Created.*natural-proof-note.txt" "$BROWSER_PROOF_JSON"
grep -q "Updated 1 file" "$BROWSER_PROOF_JSON"
grep -q "ADDED" "$BROWSER_PROOF_JSON"
grep -q "natural-proof-note.txt" "$BROWSER_PROOF_JSON"
grep -q "Inspected the repository" "$BROWSER_PROOF_JSON"
grep -q "repo_info" "$BROWSER_PROOF_JSON"
grep -q "file_list" "$BROWSER_PROOF_JSON"

echo "LIVE WebUI natural file creation + repo inspection screenshot proof passed: $BASE conversation=$CONV_ID screenshot=$SCREENSHOT_PNG"
