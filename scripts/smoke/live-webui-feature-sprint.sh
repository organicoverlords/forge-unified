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

cargo build --workspace
RUST_BACKTRACE=1 cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; git status --short > "$STATUS_OUT" 2>/dev/null || true; }
trap cleanup EXIT

for attempt in $(seq 1 120); do
  if ! kill -0 "$PID" >/dev/null 2>&1; then
    tail -n 220 "$SERVER_LOG" >&2 || true
    exit 1
  fi
  if curl -fsS --connect-timeout 2 --max-time 10 "$BASE/api/health" -o "$PROOF_DIR/health.json"; then
    break
  fi
  sleep 1
done
curl -fsS "$BASE/" -o "$PROOF_DIR/index.html"
grep -Fq "Forge Unified" "$PROOF_DIR/index.html"
grep -Fq "provider-model-visible" "$PROOF_DIR/index.html"
grep -Fq "live-browser-model-proof" "$PROOF_DIR/index.html"
echo "webui started for strict live model proof" > "$PROOF_DIR/live-proof-status.txt"
