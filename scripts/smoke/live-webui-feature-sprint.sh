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
PROMPT_FILE="$PROOF_DIR/live-model-prompt.txt"
TOOL_PROMPT_FILE="$PROOF_DIR/tool-lifecycle-prompt.txt"
REQUEST_JSON="$PROOF_DIR/live-model-request.json"
TOOL_REQUEST_JSON="$PROOF_DIR/tool-lifecycle-request.json"
STEP_LOG="$PROOF_DIR/live-proof-steps.log"
: > "$STEP_LOG"

step() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$STEP_LOG"; }

step "cargo build"
timeout 240s cargo build --workspace
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
grep -Fq "Forge Unified" "$PROOF_DIR/index.html"
grep -Fq "provider-model-visible" "$PROOF_DIR/index.html"
grep -Fq "live-browser-model-proof" "$PROOF_DIR/index.html"
grep -Fq "opencode-tool-lifecycle-rail" "$PROOF_DIR/index.html"
grep -Fq "providerExecuted-visible" "$PROOF_DIR/index.html"

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
grep -Fq 'event: run-finish' "$MODEL_STREAM"
grep -Fq 'event: text-delta' "$MODEL_STREAM"
grep -Fq '"provider":"nvidia_nim"' "$MODEL_STREAM"
grep -Fq '"model":"' "$MODEL_STREAM"
grep -Fq 'LIVE_NIM_BROWSER_PROOF' "$MODEL_STREAM"

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

step "browser proof $MODEL_ID"
timeout 120s bash scripts/smoke/capture-browser-proof.sh "$BASE" "$TOOL_CONV_ID" "$MODEL_ID" "$PROOF_DIR"
for marker in 'Please run an OpenCode file tool formatter proof' 'opencode-live-toolpart' 'providerExecuted' 'OpenCode ToolPart lifecycle metadata' 'EventV2Bridge receipts' 'file_write' 'file_edit' 'file_delete' 'Ran OpenCode-style file tool event proof'; do grep -Fq "$marker" "$PROOF_DIR/browser-proof.json"; done

echo "nim_conversation=$CONV_ID tool_conversation=$TOOL_CONV_ID model=$MODEL_ID screenshot=$PROOF_DIR/webui.png event_rail=$PROOF_DIR/event-rail.png" > "$PROOF_DIR/live-proof-status.txt"
step "done"
echo "LIVE model-backed browser proof plus visible ToolPart lifecycle proof passed: $BASE nim_conversation=$CONV_ID tool_conversation=$TOOL_CONV_ID model=$MODEL_ID"
