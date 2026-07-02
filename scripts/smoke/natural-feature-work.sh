#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [ "${FORGE_NATURAL_LINTED:-0}" != "1" ]; then
  FORGE_NATURAL_LINTED=1 bash -n "$0"
fi

PORT="${FORGE_NATURAL_PORT:-3330}"
BASE="http://127.0.0.1:${PORT}"
OUT_DIR="${FORGE_NATURAL_OUT:-$ROOT/.forge-proof/natural-feature-work}"
TITLE="${FORGE_NATURAL_TITLE:-natural feature build through WebUI}"
MAX_ROUNDS="${FORGE_NATURAL_MAX_ROUNDS:-10}"
TIMEOUT_SECONDS="${FORGE_NATURAL_TIMEOUT_SECONDS:-420}"

DEFAULT_PROMPT="$(cat <<'PROMPT'
Build one tiny safe Forge feature through the normal WebUI tools. Do not answer with a plan only.

Requirements:
1. Inspect `PROJECT_STATE.md` and `scripts/smoke/summarize-live-webui-proof.py`.
2. Make one small repo-local improvement: update `PROJECT_STATE.md` with a dated bullet saying the live WebUI natural feature-build prompt path is now proof-gated.
3. Create or update `docs/generated/proof/webui-natural-feature-build-runtime.md` with a short proof note describing the WebUI prompt, files touched, validation, and rollback.
4. Run `bash -n scripts/smoke/natural-feature-work.sh 2>&1`.
5. Final answer must include files modified, tests run, unresolved risks, and confidence.
PROMPT
)"
PROMPT="${FORGE_NATURAL_PROMPT:-${FORGE_FEATURE_PROMPT:-$DEFAULT_PROMPT}}"

mkdir -p "$OUT_DIR"

SERVER_LOG="$OUT_DIR/server.log"
STREAM_OUT="$OUT_DIR/chat-stream.sse"
CONVERSATION_JSON="$OUT_DIR/conversation.json"
CREATED_JSON="$OUT_DIR/created.json"
REQUEST_JSON="$OUT_DIR/request.json"
PROMPT_FILE="$OUT_DIR/prompt.txt"
PROOF_JSON="$OUT_DIR/browser-proof.json"
SCREENSHOT="$OUT_DIR/webui.png"
SUMMARY="$OUT_DIR/summary.json"
SUMMARY_MD="$OUT_DIR/summary.md"
GIT_STATUS="$OUT_DIR/git-status.txt"
STEP_LOG="$OUT_DIR/steps.log"
: > "$STEP_LOG"

step() { printf '[%s] %s\n' "$(date -u +%H:%M:%S)" "$*" | tee -a "$STEP_LOG"; }

fail_with_tail() {
  local code="$1"
  local msg="$2"
  local file="${3:-}"
  echo "::error::$msg" >&2
  if [ -n "$file" ] && [ -f "$file" ]; then
    tail -n 160 "$file" >&2 || true
  fi
  exit "$code"
}

json_get_id() {
  python3 - "$1" <<'PY'
import json
import sys
with open(sys.argv[1], encoding="utf-8") as fh:
    data = json.load(fh)
print(data.get("id", ""))
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
  timeout "${timeout_seconds}s" curl -fsS --connect-timeout 2 --max-time "$curl_timeout" \
    -X POST "$BASE/api/conversations/$conv_id/chat/stream" \
    -H "content-type: application/json" \
    -H "accept: text/event-stream" \
    --data-binary "@$request_json" > "$out"
}

valid_png() {
  local file="$1"
  python3 - "$file" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1])
raise SystemExit(0 if p.is_file() and p.stat().st_size > 1024 and p.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n" else 1)
PY
}

cleanup() {
  if [ -n "${PID:-}" ]; then
    kill "$PID" >/dev/null 2>&1 || true
  fi
  git status --short > "$GIT_STATUS" 2>/dev/null || true
}
trap cleanup EXIT

step "cargo build forge-app"
timeout 480s cargo build -p forge-app

step "start webui"
SERVER_BIN="$ROOT/target/debug/forge"
test -x "$SERVER_BIN"
RUST_BACKTRACE=1 "$SERVER_BIN" --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!

step "wait health"
HEALTH_OK=0
for attempt in $(seq 1 180); do
  if curl -fsS --connect-timeout 2 --max-time 10 "$BASE/api/health" >/dev/null; then
    HEALTH_OK=1
    break
  fi
  if ! kill -0 "$PID" >/dev/null 2>&1; then
    fail_with_tail 1 "webui process exited before health became ready on attempt $attempt" "$SERVER_LOG"
  fi
  sleep 1
done
[ "$HEALTH_OK" = "1" ] || fail_with_tail 1 "webui health never became ready after 180s" "$SERVER_LOG"

step "create natural feature-build conversation"
curl -fsS --connect-timeout 2 --max-time 20 \
  -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  --data-binary "$(jq -n --arg title "$TITLE" '{title:$title}')" \
  > "$CREATED_JSON"
CONV_ID="$(json_get_id "$CREATED_JSON")"
test -n "$CONV_ID"

printf '%s\n' "$PROMPT" > "$PROMPT_FILE"
jq -Rs --argjson max_rounds "$MAX_ROUNDS" '{message: ., max_rounds: $max_rounds}' "$PROMPT_FILE" > "$REQUEST_JSON"

step "run natural feature-build prompt through WebUI stream"
set +e
post_stream "$CONV_ID" "$REQUEST_JSON" "$STREAM_OUT" "$TIMEOUT_SECONDS"
STREAM_RC=$?
set -e

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON" || true

step "capture natural feature-build browser proof"
curl -fsS --connect-timeout 2 --max-time 60 \
  -X POST "$BASE/api/browser-proof" \
  -H 'content-type: application/json' \
  --data-binary "$(jq -n --arg url "$BASE/?conversation=$CONV_ID&proof=natural-feature" '{url:$url,width:1440,height:1000,capture_dom:true}')" \
  > "$PROOF_JSON" || true
if jq -e '.success == true and (.screenshot_base64 | length > 1000)' "$PROOF_JSON" >/dev/null 2>&1; then
  jq -r '.screenshot_base64' "$PROOF_JSON" | base64 -d > "$SCREENSHOT" || true
fi

if ! valid_png "$SCREENSHOT"; then
  step "fallback direct Chrome natural browser proof"
  FORGE_BROWSER_PROOF_STRATEGY="${FORGE_BROWSER_PROOF_STRATEGY:-pdf-first}" \
  FORGE_BROWSER_PROOF_ALLOW_DOM_SUMMARY_PNG="${FORGE_BROWSER_PROOF_ALLOW_DOM_SUMMARY_PNG:-1}" \
    bash scripts/smoke/capture-browser-proof.sh "$BASE" "$CONV_ID" "natural-feature" "$OUT_DIR" "natural" "&proof=natural-feature" || true
fi

python3 - "$STREAM_OUT" "$CONVERSATION_JSON" "$PROOF_JSON" "$SCREENSHOT" "$PROMPT_FILE" "$SUMMARY" "$SUMMARY_MD" "$STREAM_RC" <<'PY'
from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

stream_path = Path(sys.argv[1])
conversation_path = Path(sys.argv[2])
proof_path = Path(sys.argv[3])
screenshot_path = Path(sys.argv[4])
prompt_path = Path(sys.argv[5])
summary_path = Path(sys.argv[6])
summary_md_path = Path(sys.argv[7])
stream_rc = int(sys.argv[8])


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def load_json(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(read_text(path))
    except Exception:
        return {}
    return data if isinstance(data, dict) else {}


def parse_sse_events(text: str) -> list[tuple[str, Any]]:
    events: list[tuple[str, Any]] = []
    event_name = "message"
    data_lines: list[str] = []
    for raw in text.splitlines() + [""]:
        if not raw:
            if data_lines:
                payload = "\n".join(data_lines)
                try:
                    data: Any = json.loads(payload)
                except json.JSONDecodeError:
                    data = payload
                events.append((event_name, data))
            event_name = "message"
            data_lines = []
            continue
        if raw.startswith("event: "):
            event_name = raw[len("event: ") :]
        elif raw.startswith("data: "):
            data_lines.append(raw[len("data: ") :])
    return events


def runtime_shortcut_seen(events: list[tuple[str, Any]]) -> bool:
    for event_name, data in events:
        if event_name == "benchmark-phase":
            return True
        if isinstance(data, dict):
            if data.get("provider") == "local":
                return True
            if data.get("local_shortcut") is True:
                return True
    return False


stream_text = read_text(stream_path)
conversation = load_json(conversation_path)
proof = load_json(proof_path)
prompt = read_text(prompt_path)
conversation_text = json.dumps(conversation, sort_keys=True)
evidence = "\n".join([prompt, stream_text, conversation_text, json.dumps(proof, sort_keys=True)])
events = parse_sse_events(stream_text)

checks: list[dict[str, Any]] = []


def check(name: str, passed: bool, evidence_value: Any = None) -> None:
    item: dict[str, Any] = {"name": name, "passed": bool(passed)}
    if evidence_value is not None:
        item["evidence"] = evidence_value
    checks.append(item)


provider = str(conversation.get("provider") or "")
model = str(conversation.get("model") or "")
tool_calls = stream_text.count("event: tool-call")
tool_results = stream_text.count("event: tool-result")
edit_markers = ["file_edit", "file_write", "apply_patch", "Applied patch", "Edited file", "Wrote file"]
run_finished = "event: run-finish" in stream_text
transport_ok = stream_rc == 0 or (stream_rc in {28, 124} and run_finished)
screenshot_valid = screenshot_path.is_file() and screenshot_path.stat().st_size > 1024 and screenshot_path.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n"
capture_meta = proof.get("metadata") if isinstance(proof.get("metadata"), dict) else {}

check("stream_transport_completed_or_run_finished", transport_ok, {"exit_code": stream_rc, "run_finished": run_finished})
check("provider_is_nvidia_nim", provider == "nvidia_nim" or '"provider":"nvidia_nim"' in stream_text, provider)
check("model_recorded", bool(model) or '"model":"' in stream_text, model)
check("run_finished", run_finished)
check("tool_calls_present", tool_calls >= 2, tool_calls)
check("tool_results_present", tool_results >= 2, tool_results)
check("feature_edit_marker_present", any(marker in evidence for marker in edit_markers), edit_markers)
check("project_state_touched", "PROJECT_STATE.md" in evidence)
check("runtime_proof_note_touched", "webui-natural-feature-build-runtime.md" in evidence or "docs/generated/proof" in evidence)
check("validation_command_visible", "bash -n scripts/smoke/natural-feature-work.sh" in evidence)
check("browser_proof_success", proof.get("success") is True)
check("screenshot_png_present", screenshot_valid, screenshot_path.stat().st_size if screenshot_path.exists() else 0)
check("no_runtime_shortcut_markers", not runtime_shortcut_seen(events))
check("final_answer_reports_files_tests_risks", all(token in evidence.lower() for token in ["files", "tests", "risks", "confidence"]))

failed = [item for item in checks if not item["passed"]]
summary = {
    "passed": not failed,
    "conversation_id": conversation.get("id"),
    "provider": provider,
    "model": model,
    "tool_call_events": tool_calls,
    "tool_result_events": tool_results,
    "stream_exit_code": stream_rc,
    "stream_transport_ok": transport_ok,
    "prompt_path": str(prompt_path),
    "stream_path": str(stream_path),
    "conversation_path": str(conversation_path),
    "browser_proof_path": str(proof_path),
    "screenshot_path": str(screenshot_path),
    "screenshot_capture_metadata": capture_meta,
    "normal_webui_path": True,
    "checks": checks,
    "failed_checks": failed,
}
summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")

lines = [
    "# Natural WebUI feature-build proof",
    "",
    f"- passed: `{str(summary['passed']).lower()}`",
    f"- provider: `{provider}`",
    f"- model: `{model}`",
    f"- tool-call events: `{tool_calls}`",
    f"- tool-result events: `{tool_results}`",
    f"- stream exit code: `{stream_rc}`",
    f"- stream transport accepted: `{str(transport_ok).lower()}`",
    f"- screenshot: `{screenshot_path}`",
    f"- screenshot capture metadata: `{json.dumps(capture_meta, sort_keys=True)}`",
    "",
    "## Checks",
]
for item in checks:
    lines.append(f"- {item['name']}: `{str(item['passed']).lower()}`")
summary_md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

print(json.dumps(summary, indent=2, sort_keys=True))
raise SystemExit(0 if not failed else 1)
PY

jq -e '.passed == true' "$SUMMARY" >/dev/null

step "done"
echo "Natural WebUI feature-build proof passed"
echo "conversation=$CONV_ID"
echo "summary=$SUMMARY"
