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
BENCH_MAX_ROUNDS="${FORGE_BENCH_MAX_ROUNDS:-24}"
BENCH_TIMEOUT_SECONDS="${FORGE_BENCH_TIMEOUT_SECONDS:-360}"
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
WORKFLOW_JSON="$PROOF_DIR/opencode-workflow-checker.json"
PROMPT_FILE="$PROOF_DIR/live-model-prompt.txt"
TOOL_PROMPT_FILE="$PROOF_DIR/tool-lifecycle-prompt.txt"
BENCH_PROMPT_FILE="scripts/smoke/full-agentic-benchmark-prompt.txt"
REQUEST_JSON="$PROOF_DIR/live-model-request.json"
TOOL_REQUEST_JSON="$PROOF_DIR/tool-lifecycle-request.json"
BENCH_REQUEST_JSON="$PROOF_DIR/full-benchmark-request.json"
STEP_LOG="$PROOF_DIR/live-proof-steps.log"
: > "$STEP_LOG"

step() { printf '[%s] %s\n' "$(date -u +%H:%M:%S)" "$*" | tee -a "$STEP_LOG"; }

fail_with_tail() {
  local code="$1"
  local msg="$2"
  local file="${3:-}"
  echo "::error::$msg" >&2
  if [ -n "$file" ] && [ -f "$file" ]; then
    tail -n 200 "$file" >&2 || true
  fi
  exit "$code"
}

need_marker() {
  local file="$1"
  local marker="$2"
  grep -Fq -- "$marker" "$file" || fail_with_tail 20 "missing marker '$marker' in $file" "$file"
}

need_regex() {
  local file="$1"
  local pattern="$2"
  grep -Eiq -- "$pattern" "$file" || fail_with_tail 21 "missing regex '$pattern' in $file" "$file"
}

reject_marker() {
  local file="$1"
  local marker="$2"
  if grep -Fq -- "$marker" "$file"; then
    fail_with_tail 22 "forbidden marker '$marker' found in $file" "$file"
  fi
}

create_conversation() {
  local title="$1"
  local out="$2"
  python3 - "$title" "$out" "$BASE" <<'PY'
import json
import subprocess
import sys

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

json_field() {
  python3 - "$1" "$2" <<'PY'
import json
import sys
path, field = sys.argv[1:]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)
print(data.get(field, ""))
PY
}

assert_conversation_nim() {
  python3 - "$1" <<'PY'
import json
import sys
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
import json
import sys
with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
names = set(data.get("names") or [])
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

post_stream() {
  local conv_id="$1"
  local request_json="$2"
  local out="$3"
  local timeout_seconds="$4"
  local curl_timeout=$((timeout_seconds - 10))
  if [ "$curl_timeout" -lt 30 ]; then
    curl_timeout=30
  fi
  timeout "$timeout_seconds" curl -fsS --connect-timeout 2 --max-time "$curl_timeout" \
    -X POST "$BASE/api/conversations/$conv_id/chat/stream" \
    -H "content-type: application/json" \
    -H "accept: text/event-stream" \
    --data-binary "@$request_json" > "$out"
}

assert_no_stream_shortcut() {
  python3 - "$1" <<'PY'
import json
import sys

path = sys.argv[1]
text = open(path, encoding="utf-8", errors="replace").read() if path else ""
events = []
name = "message"
data_lines = []
for raw in text.splitlines() + [""]:
    if not raw:
        if data_lines:
            payload = "\n".join(data_lines)
            try:
                data = json.loads(payload)
            except json.JSONDecodeError:
                data = payload
            events.append({"event": name, "data": data})
        name = "message"
        data_lines = []
    elif raw.startswith("event: "):
        name = raw[len("event: "):]
    elif raw.startswith("data: "):
        data_lines.append(raw[len("data: "):])

for event in events:
    if event.get("event") == "benchmark-phase":
        raise SystemExit("actual benchmark-phase event detected")
    data = event.get("data")
    if isinstance(data, dict):
        if data.get("provider") == "local":
            raise SystemExit("actual local provider event detected")
        if data.get("local_shortcut") is True:
            raise SystemExit("actual local_shortcut=true event detected")
PY
}

synthesize_conversation_from_stream() {
  python3 - "$BENCH_STREAM" "$BENCH_CONVERSATION_JSON" "$BENCH_CONV_ID" <<'PY'
import json
import sys
from pathlib import Path

stream_path, out_path, conv_id = sys.argv[1:]
text = Path(stream_path).read_text(encoding="utf-8", errors="replace") if Path(stream_path).exists() else ""
events = []
name = "message"
data_lines = []
for raw in text.splitlines() + [""]:
    if not raw:
        if data_lines:
            payload = "\n".join(data_lines)
            try:
                data = json.loads(payload)
            except json.JSONDecodeError:
                data = payload
            events.append({"event": name, "data": data})
        name = "message"
        data_lines = []
    elif raw.startswith("event: "):
        name = raw[len("event: "):]
    elif raw.startswith("data: "):
        data_lines.append(raw[len("data: "):])

run_finish = [e["data"] for e in events if e.get("event") == "run-finish" and isinstance(e.get("data"), dict)]
last_run = run_finish[-1] if run_finish else {}
provider = last_run.get("provider") or "nvidia_nim"
model = last_run.get("model") or "unknown"
task = last_run.get("task") or "Full six-phase agentic benchmark prompt"
text_parts = []
tool_results = []
seen_tool_ids = set()
for event in events:
    data = event.get("data")
    if event.get("event") == "text-delta":
        if isinstance(data, dict):
            text_parts.append(str(data.get("delta") or data.get("text") or data.get("content") or ""))
        elif isinstance(data, str):
            text_parts.append(data)
    if event.get("event") == "tool-result" and isinstance(data, dict):
        result_id = data.get("id") or json.dumps(data, sort_keys=True)
        if result_id not in seen_tool_ids:
            seen_tool_ids.add(result_id)
            tool_results.append(data)
final = "".join(text_parts).strip()
conversation = {
    "id": conv_id,
    "provider": provider,
    "model": model,
    "messages": [
        {"role": "User", "content": task, "tool_results": []},
        {"role": "Assistant", "content": final, "tool_results": tool_results},
    ],
    "reconstructed_from_stream": True,
    "tool_results": len(tool_results),
}
Path(out_path).write_text(json.dumps(conversation, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
}

write_status() {
  local out="$1"
  printf 'nim_conversation=%s\n' "$CONV_ID" > "$out"
  printf 'tool_conversation=%s\n' "$TOOL_CONV_ID" >> "$out"
  printf 'benchmark_conversation=%s\n' "$BENCH_CONV_ID" >> "$out"
  printf 'model=%s\n' "$MODEL_ID" >> "$out"
  printf 'benchmark_max_rounds=%s\n' "$BENCH_MAX_ROUNDS" >> "$out"
  printf 'benchmark_timeout_seconds=%s\n' "$BENCH_TIMEOUT_SECONDS" >> "$out"
  printf 'benchmark_screenshot=%s\n' "$PROOF_DIR/full-benchmark-webui.png" >> "$out"
  printf 'event_rail=%s\n' "$PROOF_DIR/event-rail.png" >> "$out"
  printf 'tool_catalog=%s\n' "$TOOL_CATALOG_JSON" >> "$out"
  printf 'workflow_checker=%s\n' "$WORKFLOW_JSON" >> "$out"
}

write_benchmark_diagnostics() {
  curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$BENCH_CONV_ID" > "$BENCH_CONVERSATION_JSON" || true
  if ! python3 - "$BENCH_CONVERSATION_JSON" <<'PY'
import json
import sys
try:
    data = json.load(open(sys.argv[1], encoding="utf-8"))
except Exception:
    raise SystemExit(1)
if data.get("provider") != "nvidia_nim" or len(data.get("messages") or []) < 2:
    raise SystemExit(1)
PY
  then
    if [ -s "$BENCH_STREAM" ]; then
      synthesize_conversation_from_stream || true
    fi
  fi
  if [ -s "$BENCH_CONVERSATION_JSON" ] && [ -s "$BENCH_STREAM" ]; then
    python3 scripts/smoke/check-full-agentic-benchmark.py "$BENCH_CONVERSATION_JSON" "$BENCH_STREAM" "$PROOF_DIR/full-benchmark-checker.json" || true
    python3 scripts/smoke/check-opencode-workflow-evidence.py "$BENCH_CONVERSATION_JSON" "$BENCH_STREAM" "$WORKFLOW_JSON" || true
  else
    printf '{"passed":false,"failed_checks":[{"name":"missing_full_benchmark_artifacts","passed":false}]}' > "$PROOF_DIR/full-benchmark-checker.json"
    printf '{"passed":false,"failed_checks":[{"name":"missing_full_benchmark_artifacts","passed":false}]}' > "$WORKFLOW_JSON"
  fi
}

capture_full_benchmark_proof() {
  timeout 120s bash scripts/smoke/capture-browser-proof.sh "$BASE" "$BENCH_CONV_ID" "$MODEL_ID" "$PROOF_DIR" tool
  for marker in "Full six-phase agentic benchmark prompt" "Phase 1" "Phase 2" "apply_patch" ".agent_test/repo_summary.md" ".agent_test/action_plan.json"; do
    need_marker "$PROOF_DIR/browser-proof.json" "$marker"
  done
  need_regex "$PROOF_DIR/browser-proof.json" 'Founder report|Founder Report'
  need_regex "$PROOF_DIR/browser-proof.json" 'Technical report|Technical Report'
  cp "$PROOF_DIR/browser-proof.json" "$PROOF_DIR/full-benchmark-browser-proof.json"
  cp "$PROOF_DIR/webui.png" "$PROOF_DIR/full-benchmark-webui.png"
}

step "cargo build forge-app"
timeout 480s cargo build -p forge-app

step "start webui"
SERVER_BIN="$ROOT/target/debug/forge"
test -x "$SERVER_BIN"
printf 'command: %s --host 127.0.0.1 --port %s\n' "$SERVER_BIN" "$PORT" > "$PROOF_DIR/server-command.txt"
RUST_BACKTRACE=1 "$SERVER_BIN" --host 127.0.0.1 --port "$PORT" > "$SERVER_LOG" 2>&1 &
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
    fail_with_tail 1 "webui process exited before health became ready on attempt $attempt" "$SERVER_LOG"
  fi
  sleep 1
done
[ "$HEALTH_OK" = "1" ] || fail_with_tail 1 "webui health never became ready after 180s" "$SERVER_LOG"

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/" -o "$PROOF_DIR/index.html"
for marker in "Forge Unified" "provider-model-visible" "live-browser-model-proof" "providerExecuted-visible" "apply_patch-visible"; do
  need_marker "$PROOF_DIR/index.html" "$marker"
done

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/tools" -o "$TOOL_CATALOG_JSON"
assert_tool_catalog "$TOOL_CATALOG_JSON"
for marker in "forge_provider_tool_catalog" "provider_visible" "patchText" "todo_write" "batch_parallel" "apply_patch"; do
  need_marker "$TOOL_CATALOG_JSON" "$marker"
done
reject_marker "$TOOL_CATALOG_JSON" "packages/opencode/"
reject_marker "$TOOL_CATALOG_JSON" "opencode_"

step "create NIM conversation"
CONV_CREATED="$PROOF_DIR/live-model-created.json"
CONV_ID="$(create_conversation "live NIM browser model proof" "$CONV_CREATED")"
test -n "$CONV_ID"
printf '%s\n' "Reply with LIVE_NIM_BROWSER_PROOF and one short sentence." > "$PROMPT_FILE"
jq -Rs '{message: ., max_rounds: 2}' "$PROMPT_FILE" > "$REQUEST_JSON"

step "model chat stream"
post_stream "$CONV_ID" "$REQUEST_JSON" "$MODEL_STREAM" 180
assert_no_stream_shortcut "$MODEL_STREAM"
if grep -Fq 'event: provider-error' "$MODEL_STREAM"; then
  fail_with_tail 9 "provider error during live model proof" "$MODEL_STREAM"
fi
for marker in "event: run-finish" "event: text-delta" '"provider":"nvidia_nim"' '"model":"' "LIVE_NIM_BROWSER_PROOF"; do
  need_marker "$MODEL_STREAM" "$marker"
done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
assert_conversation_nim "$CONVERSATION_JSON"
MODEL_ID="$(json_field "$CONVERSATION_JSON" model)"
test -n "$MODEL_ID"

step "create tool lifecycle conversation"
TOOL_CREATED="$PROOF_DIR/tool-lifecycle-created.json"
TOOL_CONV_ID="$(create_conversation "Forge visible ToolPart lifecycle proof" "$TOOL_CREATED")"
test -n "$TOOL_CONV_ID"
printf '%s\n' "Please run a Forge file tool formatter proof so I can see live ToolPart lifecycle cards, providerExecuted metadata, file-change cards, event receipts, and a final human-readable summary in the WebUI." > "$TOOL_PROMPT_FILE"
jq -Rs '{message: ., max_rounds: 2}' "$TOOL_PROMPT_FILE" > "$TOOL_REQUEST_JSON"
post_stream "$TOOL_CONV_ID" "$TOOL_REQUEST_JSON" "$TOOL_STREAM" 180
for marker in "event: tool-lifecycle" "event: tool-input-start" "event: tool-input-delta" "event: tool-input-end" "event: tool-call" "event: tool-result" "event: file-change" "providerExecuted" "ToolStateCompleted" "file_write" "file_edit" "file_delete" "file-tool-event-proof.rs"; do
  need_marker "$TOOL_STREAM" "$marker"
done
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$TOOL_CONV_ID" > "$TOOL_CONVERSATION_JSON"
for marker in "providerExecuted" "mutable_tool_part_updates" "same ToolPart row updated by callID" "ToolStateCompleted.attachments" "file-tool-event-proof.rs"; do
  need_marker "$TOOL_CONVERSATION_JSON" "$marker"
done

step "browser proof tool lifecycle"
timeout 120s bash scripts/smoke/capture-browser-proof.sh "$BASE" "$TOOL_CONV_ID" "$MODEL_ID" "$PROOF_DIR" tool
for marker in "Please run a Forge file tool formatter proof" "providerExecuted" "file_write" "file_edit" "file_delete" "apply_patch"; do
  need_marker "$PROOF_DIR/browser-proof.json" "$marker"
done
cp "$PROOF_DIR/browser-proof.json" "$PROOF_DIR/tool-lifecycle-browser-proof.json"
cp "$PROOF_DIR/webui.png" "$PROOF_DIR/tool-lifecycle-webui.png"

step "create full benchmark conversation"
BENCH_CREATED="$PROOF_DIR/full-benchmark-created.json"
BENCH_CONV_ID="$(create_conversation "Full six-phase agentic benchmark prompt" "$BENCH_CREATED")"
test -n "$BENCH_CONV_ID"
step "full benchmark budget: max_rounds=$BENCH_MAX_ROUNDS timeout=${BENCH_TIMEOUT_SECONDS}s"
jq -Rs --argjson max_rounds "$BENCH_MAX_ROUNDS" '{message: ., max_rounds: $max_rounds}' "$BENCH_PROMPT_FILE" > "$BENCH_REQUEST_JSON"
set +e
post_stream "$BENCH_CONV_ID" "$BENCH_REQUEST_JSON" "$BENCH_STREAM" "$BENCH_TIMEOUT_SECONDS"
BENCH_RC=$?
set -e
write_benchmark_diagnostics
if [ "$BENCH_RC" -ne 0 ]; then
  fail_with_tail 15 "full benchmark stream did not finish before timeout; partial conversation and checkers were preserved" "$BENCH_STREAM"
fi
assert_no_stream_shortcut "$BENCH_STREAM"
if grep -Fq 'event: provider-error' "$BENCH_STREAM"; then
  fail_with_tail 13 "provider error during full benchmark prompt" "$BENCH_STREAM"
fi
for marker in "event: run-finish" '"provider":"nvidia_nim"' '"model":"' "event: tool-call" "event: tool-result" "file_write" "file_read" "file_delete" ".agent_test/repo_summary.md" ".agent_test/investigation.md" ".agent_test/action_plan.json"; do
  need_marker "$BENCH_STREAM" "$marker"
done
if ! grep -Eq 'repo_info|file_list|file_search|file_read|shell_command|apply_patch|task|batch_parallel|todo_write' "$BENCH_STREAM"; then
  fail_with_tail 14 "full benchmark did not use advertised provider tools" "$BENCH_STREAM"
fi
step "browser proof full benchmark"
capture_full_benchmark_proof
assert_conversation_nim "$BENCH_CONVERSATION_JSON"
python3 scripts/smoke/check-full-agentic-benchmark.py "$BENCH_CONVERSATION_JSON" "$BENCH_STREAM" "$PROOF_DIR/full-benchmark-checker.json"
jq -e '.passed == true' "$PROOF_DIR/full-benchmark-checker.json" >/dev/null
python3 scripts/smoke/check-opencode-workflow-evidence.py "$BENCH_CONVERSATION_JSON" "$BENCH_STREAM" "$WORKFLOW_JSON"
jq -e '.passed == true' "$WORKFLOW_JSON" >/dev/null
write_status "$PROOF_DIR/live-proof-status.txt"

step "done"
echo "LIVE WebUI/NVIDIA NIM proof passed"
echo "base=$BASE"
echo "nim_conversation=$CONV_ID"
echo "tool_conversation=$TOOL_CONV_ID"
echo "benchmark_conversation=$BENCH_CONV_ID"
echo "model=$MODEL_ID"
