#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [ "${FORGE_LIVE_WEBUI_LINTED:-0}" != "1" ]; then
  FORGE_LIVE_WEBUI_LINTED=1 bash -n "$0"
fi

PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
mkdir -p "$PROOF_DIR"

SERVER_LOG="$PROOF_DIR/server.log"
STATUS_OUT="$PROOF_DIR/git-status.txt"
TOOL_CATALOG_JSON="$PROOF_DIR/tool-catalog.json"
CONVERSATION_JSON="$PROOF_DIR/live-model-conversation.json"
MODEL_STREAM="$PROOF_DIR/live-model-stream.sse"
TOOL_CONVERSATION_JSON="$PROOF_DIR/tool-lifecycle-conversation.json"
TOOL_STREAM="$PROOF_DIR/tool-lifecycle-stream.sse"
BENCH_CONVERSATION_JSON="$PROOF_DIR/full-benchmark-conversation.json"
BENCH_STREAM="$PROOF_DIR/full-benchmark-stream.sse"
OPENCODE_WORKFLOW_JSON="$PROOF_DIR/opencode-workflow-checker.json"
PROMPT_FILE="$PROOF_DIR/live-model-prompt.txt"
TOOL_PROMPT_FILE="$PROOF_DIR/tool-lifecycle-prompt.txt"
BENCH_PROMPT_FILE="scripts/smoke/full-agentic-benchmark-prompt.txt"
REQUEST_JSON="$PROOF_DIR/live-model-request.json"
TOOL_REQUEST_JSON="$PROOF_DIR/tool-lifecycle-request.json"
BENCH_REQUEST_JSON="$PROOF_DIR/full-benchmark-request.json"
STEP_LOG="$PROOF_DIR/live-proof-steps.log"
: > "$STEP_LOG"

step() {
  echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$STEP_LOG"
}

need_marker() {
  local file="$1"
  local marker="$2"
  if ! grep -Fq -- "$marker" "$file"; then
    echo "::error::missing marker '$marker' in $file" >&2
    tail -n 200 "$file" >&2 || true
    exit 20
  fi
}

need_any_regex() {
  local file="$1"
  local pattern="$2"
  if ! grep -Eiq -- "$pattern" "$file"; then
    echo "::error::missing regex '$pattern' in $file" >&2
    tail -n 200 "$file" >&2 || true
    exit 21
  fi
}

reject_marker() {
  local file="$1"
  local marker="$2"
  if grep -Fq -- "$marker" "$file"; then
    echo "::error::forbidden marker '$marker' found in $file" >&2
    tail -n 200 "$file" >&2 || true
    exit 22
  fi
}

create_conversation() {
  local title="$1"
  local out="$2"
  python3 - "$title" "$out" "$BASE" <<'PY'
import json, subprocess, sys

title, out, base = sys.argv[1:]
payload = json.dumps({"title": title})
with open(out, "wb") as fh:
    subprocess.run([
        "curl", "-fsS", "--connect-timeout", "2", "--max-time", "20",
        "-X", "POST", f"{base}/api/conversations",
        "-H", "content-type: application/json",
        "--data-binary", payload,
    ], check=True, stdout=fh)
with open(out, encoding="utf-8") as fh:
    data = json.load(fh)
print(data.get("id", ""))
PY
}

model_id_from_conversation() {
  python3 - "$1" <<'PY'
import json, sys
with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
print(data.get("model", ""))
PY
}

assert_conversation_nim() {
  python3 - "$1" <<'PY'
import json, sys
with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
messages = data.get("messages") or []
provider = data.get("provider")
model = data.get("model")
if provider != "nvidia_nim" or not isinstance(model, str) or not model or len(messages) < 2:
    raise SystemExit(f"conversation is not a proved NIM conversation: provider={provider!r} model={model!r} messages={len(messages)}")
PY
}

assert_tool_catalog() {
  python3 - "$1" <<'PY'
import json, sys
with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
names = data.get("names") or []
required = {"apply_patch", "task", "todo_write", "batch_parallel"}
if data.get("catalog") != "forge_provider_tool_catalog":
    raise SystemExit("tool catalog marker mismatch")
if data.get("provider_visible") is not True:
    raise SystemExit("tool catalog is not provider-visible")
if int(data.get("tool_count") or 0) < 20:
    raise SystemExit("tool catalog has too few tools")
missing = sorted(required.difference(names))
if missing:
    raise SystemExit(f"tool catalog missing required tools: {missing}")
PY
}

write_status() {
  local out="$1"
  {
    echo "nim_conversation=$CONV_ID"
    echo "tool_conversation=$TOOL_CONV_ID"
    echo "benchmark_conversation=$BENCH_CONV_ID"
    echo "model=$MODEL_ID"
    echo "benchmark_screenshot=$PROOF_DIR/full-benchmark-webui.png"
    echo "event_rail=$PROOF_DIR/event-rail.png"
    echo "tool_catalog=$TOOL_CATALOG_JSON"
    echo "workflow_checker=$OPENCODE_WORKFLOW_JSON"
  } > "$out"
}

step "cargo build forge-app"
timeout 480s cargo build -p forge-app

step "start webui"
SERVER_BIN="$ROOT/target/debug/forge"
test -x "$SERVER_BIN"
echo "command: $SERVER_BIN --host 127.0.0.1 --port $PORT" > "$PROOF_DIR/server-command.txt"
RUST_BACKTRACE=1 "$SERVER_BIN" --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() {
  kill "$PID" >/dev/null 2>&1 || true
  git status --short > "$STATUS_OUT" 2>/dev/null || true
}
trap cleanup EXIT

step "wait health"
HEALTH_OK=0
for attempt in $(seq 1 180); do
  if curl -fsS --connect-timeout 2 --max-time 10 "$BASE/api/health" -o "$PROOF_DIR/health.json"; then
    HEALTH_OK=1
    break
  fi
  if ! kill -0 "$PID" >/dev/null 2>&1; then
    echo "::error::webui process exited before health became ready on attempt $attempt" >&2
    tail -n 220 "$SERVER_LOG" >&2 || true
    exit 1
  fi
  sleep 1
done
if [ "$HEALTH_OK" != 1 ]; then
  echo "::error::webui health never became ready after 180s" >&2
  tail -n 220 "$SERVER_LOG" >&2 || true
  exit 1
fi

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/" -o "$PROOF_DIR/index.html"
for marker in \
  "Forge Unified" \
  "provider-model-visible" \
  "live-browser-model-proof" \
  "providerExecuted-visible" \
  "apply_patch-visible"
do
  need_marker "$PROOF_DIR/index.html" "$marker"
done

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/tools" -o "$TOOL_CATALOG_JSON"
assert_tool_catalog "$TOOL_CATALOG_JSON"
for marker in \
  "forge_provider_tool_catalog" \
  "provider_visible" \
  "patchText" \
  "todo_write" \
  "batch_parallel" \
  "apply_patch"
do
  need_marker "$TOOL_CATALOG_JSON" "$marker"
done
reject_marker "$TOOL_CATALOG_JSON" "packages/opencode/"
reject_marker "$TOOL_CATALOG_JSON" "opencode_"

step "create NIM conversation"
CONV_CREATED="$PROOF_DIR/live-model-created.json"
CONV_ID="$(create_conversation "live NIM browser model proof" "$CONV_CREATED")"
test -n "$CONV_ID"
cat > "$PROMPT_FILE" <<'PROMPT'
Reply with LIVE_NIM_BROWSER_PROOF and one short sentence.
PROMPT
jq -Rs '{message: ., max_rounds: 2}' "$PROMPT_FILE" > "$REQUEST_JSON"

step "model chat stream"
timeout 180s curl -fsS --connect-timeout 2 --max-time 170 \
  -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H "content-type: application/json" \
  -H "accept: text/event-stream" \
  --data-binary "@$REQUEST_JSON" > "$MODEL_STREAM"

step "assert model stream"
if grep -Fq '"provider":"local"' "$MODEL_STREAM" || grep -Fq 'event: benchmark-phase' "$MODEL_STREAM" || grep -Eq '"local_shortcut"[[:space:]]*:[[:space:]]*true' "$MODEL_STREAM"; then
  echo "::error::local or scripted proof path detected" >&2
  tail -n 160 "$MODEL_STREAM" >&2 || true
  exit 8
fi
if grep -Fq 'event: provider-error' "$MODEL_STREAM"; then
  echo "::error::provider error during live model proof" >&2
  tail -n 160 "$MODEL_STREAM" >&2 || true
  exit 9
fi
for marker in \
  'event: run-finish' \
  'event: text-delta' \
  '"provider":"nvidia_nim"' \
  '"model":"' \
  'LIVE_NIM_BROWSER_PROOF'
do
  need_marker "$MODEL_STREAM" "$marker"
done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
assert_conversation_nim "$CONVERSATION_JSON"
MODEL_ID="$(model_id_from_conversation "$CONVERSATION_JSON")"
test -n "$MODEL_ID"

step "create tool lifecycle conversation"
TOOL_CREATED="$PROOF_DIR/tool-lifecycle-created.json"
TOOL_CONV_ID="$(create_conversation "Forge visible ToolPart lifecycle proof" "$TOOL_CREATED")"
test -n "$TOOL_CONV_ID"
cat > "$TOOL_PROMPT_FILE" <<'PROMPT'
Please run a Forge file tool formatter proof so I can see live ToolPart lifecycle cards, providerExecuted metadata, file-change cards, event bus receipts, and a final human-readable summary in the WebUI.
PROMPT
jq -Rs '{message: ., max_rounds: 2}' "$TOOL_PROMPT_FILE" > "$TOOL_REQUEST_JSON"

timeout 180s curl -fsS --connect-timeout 2 --max-time 170 \
  -X POST "$BASE/api/conversations/$TOOL_CONV_ID/chat/stream" \
  -H "content-type: application/json" \
  -H "accept: text/event-stream" \
  --data-binary "@$TOOL_REQUEST_JSON" > "$TOOL_STREAM"
for marker in \
  'event: tool-lifecycle' \
  'event: tool-input-start' \
  'event: tool-input-delta' \
  'event: tool-input-end' \
  'event: tool-call' \
  'event: tool-result' \
  'event: file-change' \
  'event: event-bus' \
  'providerExecuted' \
  'ToolStateCompleted' \
  'file_write' \
  'file_edit' \
  'file_delete' \
  'file-tool-event-proof.rs'
do
  need_marker "$TOOL_STREAM" "$marker"
done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$TOOL_CONV_ID" > "$TOOL_CONVERSATION_JSON"
for marker in \
  'providerExecuted' \
  'mutable_tool_part_updates' \
  'same ToolPart row updated by callID' \
  'ToolStateCompleted.attachments' \
  'file-tool-event-proof.rs'
do
  need_marker "$TOOL_CONVERSATION_JSON" "$marker"
done

step "browser proof tool lifecycle"
timeout 120s bash scripts/smoke/capture-browser-proof.sh "$BASE" "$TOOL_CONV_ID" "$MODEL_ID" "$PROOF_DIR" tool
for marker in \
  'Please run a Forge file tool formatter proof' \
  'providerExecuted' \
  'file_write' \
  'file_edit' \
  'file_delete' \
  'apply_patch'
do
  need_marker "$PROOF_DIR/browser-proof.json" "$marker"
done
cp "$PROOF_DIR/browser-proof.json" "$PROOF_DIR/tool-lifecycle-browser-proof.json"
cp "$PROOF_DIR/webui.png" "$PROOF_DIR/tool-lifecycle-webui.png"

step "create full benchmark conversation"
BENCH_CREATED="$PROOF_DIR/full-benchmark-created.json"
BENCH_CONV_ID="$(create_conversation "Full six-phase agentic benchmark prompt" "$BENCH_CREATED")"
test -n "$BENCH_CONV_ID"
jq -Rs '{message: ., max_rounds: 75}' "$BENCH_PROMPT_FILE" > "$BENCH_REQUEST_JSON"

timeout 1200s curl -fsS --connect-timeout 2 --max-time 1190 \
  -X POST "$BASE/api/conversations/$BENCH_CONV_ID/chat/stream" \
  -H "content-type: application/json" \
  -H "accept: text/event-stream" \
  --data-binary "@$BENCH_REQUEST_JSON" > "$BENCH_STREAM"
if grep -Fq '"provider":"local"' "$BENCH_STREAM" || grep -Fq 'event: benchmark-phase' "$BENCH_STREAM" || grep -Eq '"local_shortcut"[[:space:]]*:[[:space:]]*true' "$BENCH_STREAM"; then
  echo "::error::full benchmark used local/scripted shortcut" >&2
  tail -n 200 "$BENCH_STREAM" >&2 || true
  exit 12
fi
if grep -Fq 'event: provider-error' "$BENCH_STREAM"; then
  echo "::error::provider error during full benchmark prompt" >&2
  tail -n 200 "$BENCH_STREAM" >&2 || true
  exit 13
fi
for marker in \
  'event: run-finish' \
  '"provider":"nvidia_nim"' \
  '"model":"' \
  'event: tool-call' \
  'event: tool-result'
do
  need_marker "$BENCH_STREAM" "$marker"
done
if ! grep -Eq 'repo_info|file_list|file_search|file_read|shell_command|apply_patch|task|batch_parallel|todo_write' "$BENCH_STREAM"; then
  echo "::error::full benchmark did not use advertised provider tools" >&2
  tail -n 200 "$BENCH_STREAM" >&2 || true
  exit 14
fi
for marker in \
  'file_write' \
  'file_read' \
  'file_delete' \
  '.agent_test/repo_summary.md' \
  '.agent_test/investigation.md' \
  '.agent_test/action_plan.json' \
  'PROJECT_STATE.md'
do
  need_marker "$BENCH_STREAM" "$marker"
done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$BENCH_CONV_ID" > "$BENCH_CONVERSATION_JSON"
assert_conversation_nim "$BENCH_CONVERSATION_JSON"
python3 scripts/smoke/check-full-agentic-benchmark.py "$BENCH_CONVERSATION_JSON" "$BENCH_STREAM" "$PROOF_DIR/full-benchmark-checker.json"
python3 - "$PROOF_DIR/full-benchmark-checker.json" <<'PY'
import json, sys
with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
if data.get("passed") is not True:
    raise SystemExit("full benchmark checker did not pass")
PY
python3 scripts/smoke/check-opencode-workflow-evidence.py "$BENCH_CONVERSATION_JSON" "$BENCH_STREAM" "$OPENCODE_WORKFLOW_JSON"
python3 - "$OPENCODE_WORKFLOW_JSON" <<'PY'
import json, sys
with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
if data.get("passed") is not True:
    raise SystemExit("workflow checker did not pass")
PY

step "browser proof full benchmark"
timeout 120s bash scripts/smoke/capture-browser-proof.sh "$BASE" "$BENCH_CONV_ID" "$MODEL_ID" "$PROOF_DIR" tool
for marker in \
  'Full six-phase agentic benchmark prompt' \
  'Phase 1' \
  'Phase 2' \
  'apply_patch' \
  '.agent_test/repo_summary.md' \
  '.agent_test/action_plan.json'
do
  need_marker "$PROOF_DIR/browser-proof.json" "$marker"
done
need_any_regex "$PROOF_DIR/browser-proof.json" 'Founder report|Founder Report'
need_any_regex "$PROOF_DIR/browser-proof.json" 'Technical report|Technical Report'
cp "$PROOF_DIR/browser-proof.json" "$PROOF_DIR/full-benchmark-browser-proof.json"
cp "$PROOF_DIR/webui.png" "$PROOF_DIR/full-benchmark-webui.png"
write_status "$PROOF_DIR/live-proof-status.txt"

step "done"
echo "LIVE WebUI/NVIDIA NIM proof passed"
echo "base=$BASE"
echo "nim_conversation=$CONV_ID"
echo "tool_conversation=$TOOL_CONV_ID"
echo "benchmark_conversation=$BENCH_CONV_ID"
echo "model=$MODEL_ID"
