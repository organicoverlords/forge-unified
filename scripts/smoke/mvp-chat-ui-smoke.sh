#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_SMOKE_PORT:-3310}"
BASE="http://127.0.0.1:${PORT}"
LOG="${TMPDIR:-/tmp}/forge-unified-mvp-smoke.log"

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
  -d '{"title":"mvp smoke"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"

test -n "$CONV_ID"
curl -fsS "$BASE/api/conversations/$CONV_ID" | grep -q "mvp smoke"
curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/snapshot" \
  -H 'content-type: application/json' \
  -d '{}' | grep -q "snapshot_saved"

echo "MVP chat UI smoke passed: $BASE conversation=$CONV_ID"
