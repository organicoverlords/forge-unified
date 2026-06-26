#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_SMOKE_PORT:-3310}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_SMOKE_PROOF_DIR:-$ROOT/.forge-proof/webui-smoke}"
PUBLIC_PROOF_DIR="${FORGE_PUBLIC_SMOKE_PROOF_DIR:-$ROOT/forge-proof/webui-smoke}"
mkdir -p "$PROOF_DIR" "$PUBLIC_PROOF_DIR"
LOG="$PROOF_DIR/server.log"
STREAM_OUT="$PROOF_DIR/chat-stream.sse"
PROOF_JSON="$PROOF_DIR/browser-proof.json"
PROOF_PNG="$PROOF_DIR/webui.png"

cargo build --workspace

cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$LOG" 2>&1 &
PID=$!
cleanup() {
  kill "$PID" >/dev/null 2>&1 || true
  cp -a "$PROOF_DIR/." "$PUBLIC_PROOF_DIR/" >/dev/null 2>&1 || true
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
  -d '{"title":"live natural patch smoke"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"

test -n "$CONV_ID"
curl -fsS "$BASE/api/conversations/$CONV_ID" | grep -q "live natural patch smoke"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  -d '{"message":"Use the normal chat interface and the live model provider. Patch this app by using file_edit on crates/webui/src/events.rs. Replace the exact text Server-sent event endpoints for chat runs. with Server-sent event endpoints for chat runs and live patch proof. Then inspect the repository with repo_info and file_list path dot, and build-check the patched app by calling shell_command with command cargo check --workspace --all-targets. Then briefly report whether the patch and build passed.","max_rounds":1}' \
  > "$STREAM_OUT"

grep -q "event: run-start" "$STREAM_OUT"
grep -q "event:" "$STREAM_OUT"
if grep -qi "provider-error\|missing_key\|runtime is missing" "$STREAM_OUT"; then
  echo "::error::Live smoke produced provider-error or missing-key output."
  exit 3
fi
grep -q "event: tool-input-start" "$STREAM_OUT"
grep -q "event: tool-input-delta" "$STREAM_OUT"
grep -q "event: tool-input-end" "$STREAM_OUT"
grep -q "event: tool-call" "$STREAM_OUT"
grep -q "event: tool-result" "$STREAM_OUT"
grep -q '"name":"repo_info"' "$STREAM_OUT"
grep -q '"name":"file_list"' "$STREAM_OUT"
grep -q '"name":"file_edit"' "$STREAM_OUT"
grep -q 'crates/webui/src/events.rs' "$STREAM_OUT"
grep -q 'live patch proof' "$STREAM_OUT"
grep -q '"name":"shell_command"' "$STREAM_OUT"
grep -q 'cargo check --workspace --all-targets' "$STREAM_OUT"

grep -q "live patch proof" crates/webui/src/events.rs

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/snapshot" \
  -H 'content-type: application/json' \
  -d '{}' | grep -q "snapshot_saved"

curl -fsS -X POST "$BASE/api/browser-proof" \
  -H 'content-type: application/json' \
  -d "{\"url\":\"$BASE/\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" \
  > "$PROOF_JSON"

jq -e '.success == true' "$PROOF_JSON" >/dev/null
jq -r '.screenshot_base64' "$PROOF_JSON" | base64 -d > "$PROOF_PNG"

echo "LIVE WebUI patch-and-build smoke passed: $BASE conversation=$CONV_ID proof_dir=$PROOF_DIR public_proof_dir=$PUBLIC_PROOF_DIR"
