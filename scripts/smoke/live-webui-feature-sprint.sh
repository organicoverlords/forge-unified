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
TOOL_CONVERSATION_JSON="$PROOF_DIR/tool-lifecycle-conversation.json"
TOOL_STREAM="$PROOF_DIR/tool-lifecycle-stream.sse"
BENCH_CONVERSATION_JSON="$PROOF_DIR/full-benchmark-conversation.json"
BENCH_STREAM="$PROOF_DIR/full-benchmark-stream.sse"
PROMPT_FILE="$PROOF_DIR/live-model-prompt.txt"
TOOL_PROMPT_FILE="$PROOF_DIR/tool-lifecycle-prompt.txt"
BENCH_PROMPT_FILE="scripts/smoke/full-agentic-benchmark-prompt.txt"
REQUEST_JSON="$PROOF_DIR/live-model-request.json"
TOOL_REQUEST_JSON="$PROOF_DIR/tool-lifecycle-request.json"
BENCH_REQUEST_JSON="$PROOF_DIR/full-benchmark-request.json"
STEP_LOG="$PROOF_DIR/live-proof-steps.log"
: > "$STEP_LOG"

step() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$STEP_LOG"; }

step "cargo build forge-app"
timeout 480s cargo build -p forge-app
step "start webui"
RUST_BACKTRACE=1 cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; git status --short > "$STATUS_OUT" 2>/dev/null || true; }
trap cleanup EXIT

step "wait health"
for attempt in $(seq 1 60); do
  if ! kill -0 "$PID" >/dev/null 2>&1; then tail -n 220 "$SERVER_LOG" >&2 || true; exit 1; fi
  if curl -fsS --connect-timeout 2 --max-time 10 "$BASE/api/health" -o "$PROOF_DIR/health.json"; then break; fi
  sleep 1
done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/" -o "$PROOF_DIR/index.html"
for marker in "Forge Unified" "provider-model-visible" "live-browser-model-proof" "opencode-tool-lifecycle-rail" "providerExecuted-visible"; do grep -Fq "$marker" "$PROOF_DIR/index.html"; done

step "create NIM conversation"
CONV_ID="$(curl -fsS --connect-timeout 2 --max-time 20 -X POST "$BASE/api/conversations" -H 'content-type: application/json' -d '{"title":"live NIM browser model proof"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$CONV_ID"
cat > "$PROMPT_FILE" <<'PROMPT'
Reply with LIVE_NIM_BROWSER_PROOF and one short sentence.
PROMPT
jq -Rs '{message: ., max_rounds: 2}' "$PROMPT_FILE" > "$REQUEST_JSON"

step "model chat stream"
timeout 180s curl -fsS --connect-timeout 2 --max-time 170 -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$REQUEST_JSON" > "$MODEL_STREAM"

step "assert model stream"
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
for marker in 'event: run-finish' 'event: text-delta' '"provider":"nvidia_nim"' '"model":"' 'LIVE_NIM_BROWSER_PROOF'; do grep -Fq "$marker" "$MODEL_STREAM"; done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
jq -e '.provider == "nvidia_nim" and (.model | type == "string" and length > 0) and (.messages | length >= 2)' "$CONVERSATION_JSON" >/dev/null
MODEL_ID="$(jq -r '.model' "$CONVERSATION_JSON")"

step "create tool lifecycle conversation"
TOOL_CONV_ID="$(curl -fsS --connect-timeout 2 --max-time 20 -X POST "$BASE/api/conversations" -H 'content-type: application/json' -d '{"title":"OpenCode visible ToolPart lifecycle proof"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$TOOL_CONV_ID"
cat > "$TOOL_PROMPT_FILE" <<'PROMPT'
Please run an OpenCode file tool formatter proof so I can see live ToolPart lifecycle cards, providerExecuted metadata, file-change cards, EventV2Bridge receipts, and a final human-readable summary in the WebUI.
PROMPT
jq -Rs '{message: ., max_rounds: 2}' "$TOOL_PROMPT_FILE" > "$TOOL_REQUEST_JSON"

timeout 180s curl -fsS --connect-timeout 2 --max-time 170 -X POST "$BASE/api/conversations/$TOOL_CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$TOOL_REQUEST_JSON" > "$TOOL_STREAM"
for marker in 'event: tool-lifecycle' 'event: tool-input-start' 'event: tool-input-delta' 'event: tool-input-end' 'event: tool-call' 'event: tool-result' 'event: file-change' 'event: event-bus' 'providerExecuted' 'providerExecuted_delta' 'doom_loop_threshold' 'opencode_provider_executed_source' 'ToolStateCompleted' 'file_write' 'file_edit' 'file_delete' 'file-tool-event-proof.rs'; do grep -Fq "$marker" "$TOOL_STREAM"; done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$TOOL_CONV_ID" > "$TOOL_CONVERSATION_JSON"
for marker in 'providerExecuted' 'opencode_provider_executed_source' 'mutable_tool_part_updates' 'same ToolPart row updated by callID' 'ToolStateCompleted.attachments' 'file-tool-event-proof.rs'; do grep -Fq "$marker" "$TOOL_CONVERSATION_JSON"; done

step "browser proof tool lifecycle"
timeout 120s bash scripts/smoke/capture-browser-proof.sh "$BASE" "$TOOL_CONV_ID" "$MODEL_ID" "$PROOF_DIR" tool
for marker in 'Please run an OpenCode file tool formatter proof' 'opencode-live-toolpart' 'providerExecuted' 'OpenCode ToolPart lifecycle metadata' 'EventV2Bridge receipts' 'file_write' 'file_edit' 'file_delete' 'Ran OpenCode-style file tool event proof'; do grep -Fq "$marker" "$PROOF_DIR/browser-proof.json"; done
cp "$PROOF_DIR/browser-proof.json" "$PROOF_DIR/tool-lifecycle-browser-proof.json"
cp "$PROOF_DIR/webui.png" "$PROOF_DIR/tool-lifecycle-webui.png"

step "create full benchmark conversation"
BENCH_CONV_ID="$(curl -fsS --connect-timeout 2 --max-time 20 -X POST "$BASE/api/conversations" -H 'content-type: application/json' -d '{"title":"Full six-phase agentic benchmark prompt"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$BENCH_CONV_ID"
jq -Rs '{message: ., max_rounds: 10}' "$BENCH_PROMPT_FILE" > "$BENCH_REQUEST_JSON"

timeout 480s curl -fsS --connect-timeout 2 --max-time 470 -X POST "$BASE/api/conversations/$BENCH_CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$BENCH_REQUEST_JSON" > "$BENCH_STREAM"
if grep -Fq '"provider":"local"' "$BENCH_STREAM" || grep -Fq 'event: benchmark-phase' "$BENCH_STREAM" || grep -Fq 'local_shortcut' "$BENCH_STREAM"; then
  echo "::error::full benchmark used local/scripted shortcut" >&2
  tail -n 200 "$BENCH_STREAM" >&2 || true
  exit 12
fi
if grep -Fq 'event: provider-error' "$BENCH_STREAM"; then
  echo "::error::provider error during full benchmark prompt" >&2
  tail -n 200 "$BENCH_STREAM" >&2 || true
  exit 13
fi
for marker in 'event: run-finish' '"provider":"nvidia_nim"' '"model":"' 'event: tool-call' 'event: tool-result'; do grep -Fq "$marker" "$BENCH_STREAM"; done
if ! grep -Eq 'repo_info|file_list|file_search|file_read|shell_command' "$BENCH_STREAM"; then
  echo "::error::full benchmark did not use repo inspection tools" >&2
  tail -n 200 "$BENCH_STREAM" >&2 || true
  exit 14
fi
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$BENCH_CONV_ID" > "$BENCH_CONVERSATION_JSON"
jq -e '.provider == "nvidia_nim" and (.model | type == "string" and length > 0) and (.messages | length >= 2)' "$BENCH_CONVERSATION_JSON" >/dev/null

step "browser proof full benchmark"
timeout 120s bash scripts/smoke/capture-browser-proof.sh "$BASE" "$BENCH_CONV_ID" "$MODEL_ID" "$PROOF_DIR" tool
for marker in 'Full six-phase agentic benchmark prompt' 'Phase 1' 'Phase 2' 'Founder report'; do grep -Fq "$marker" "$PROOF_DIR/browser-proof.json"; done
cp "$PROOF_DIR/browser-proof.json" "$PROOF_DIR/full-benchmark-browser-proof.json"
cp "$PROOF_DIR/webui.png" "$PROOF_DIR/full-benchmark-webui.png"

echo "nim_conversation=$CONV_ID tool_conversation=$TOOL_CONV_ID benchmark_conversation=$BENCH_CONV_ID model=$MODEL_ID benchmark_screenshot=$PROOF_DIR/full-benchmark-webui.png event_rail=$PROOF_DIR/event-rail.png" > "$PROOF_DIR/live-proof-status.txt"
step "done"
echo "LIVE model-backed browser proof, visible ToolPart proof, and full benchmark prompt proof passed: $BASE nim_conversation=$CONV_ID tool_conversation=$TOOL_CONV_ID benchmark_conversation=$BENCH_CONV_ID model=$MODEL_ID"
