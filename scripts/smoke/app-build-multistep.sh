#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [ "${FORGE_APP_BUILD_MULTI_LINTED:-0}" != "1" ]; then
  FORGE_APP_BUILD_MULTI_LINTED=1 bash -n "$0"
fi

PORT="${FORGE_APP_BUILD_MULTI_PORT:-3360}"
BASE="http://127.0.0.1:${PORT}"
OUT_DIR="${FORGE_APP_BUILD_MULTI_OUT:-$ROOT/forge-proof/app-multistep-build-proof}"
TIMEOUT_SECONDS="${FORGE_APP_BUILD_MULTI_TIMEOUT_SECONDS:-360}"
SPEC="docs/generated/proof/app-multistep-spec.md"
REPORT="docs/generated/proof/app-multistep-report.md"
SPEC_MARKER="APP_MULTI_STEP_SPEC"
REPORT_MARKER="APP_MULTI_STEP_REPORT"
VALIDATION_CMD='grep -R APP_MULTI_STEP_ docs/generated/proof/app-multistep-*.md'
mkdir -p "$OUT_DIR"

SERVER_LOG="$OUT_DIR/server.log"
STEP_LOG="$OUT_DIR/steps.log"
CREATED_JSON="$OUT_DIR/created.json"
REQUEST_JSON="$OUT_DIR/request.json"
STREAM_OUT="$OUT_DIR/chat-stream.sse"
CONVERSATION_JSON="$OUT_DIR/conversation.json"
BROWSER_JSON="$OUT_DIR/browser-proof.json"
SCREENSHOT="$OUT_DIR/webui.png"
SUMMARY="$OUT_DIR/summary.json"
SUMMARY_MD="$OUT_DIR/summary.md"
PROMPT_FILE="$OUT_DIR/prompt.txt"
SPEC_COPY="$OUT_DIR/app-multistep-spec.md"
REPORT_COPY="$OUT_DIR/app-multistep-report.md"
DIFF_FILE="$OUT_DIR/app-multistep.diff"
STATUS_OUT="$OUT_DIR/git-status.txt"
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

cleanup() {
  if [ -n "${PID:-}" ]; then
    kill "$PID" >/dev/null 2>&1 || true
  fi
  git status --short > "$STATUS_OUT" 2>/dev/null || true
  git diff -- "$SPEC" "$REPORT" > "$DIFF_FILE" 2>/dev/null || true
  [ -f "$SPEC" ] && cp "$SPEC" "$SPEC_COPY" || true
  [ -f "$REPORT" ] && cp "$REPORT" "$REPORT_COPY" || true
}
trap cleanup EXIT

json_id() {
  python3 - "$1" <<'PY'
import json, sys
print(json.load(open(sys.argv[1], encoding="utf-8")).get("id", ""))
PY
}

step "cargo build forge-app"
timeout 420s cargo build -p forge-app

step "start webui"
SERVER_BIN="$ROOT/target/debug/forge"
test -x "$SERVER_BIN"
RUST_BACKTRACE=1 "$SERVER_BIN" --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!

step "wait health"
for attempt in $(seq 1 90); do
  if curl -fsS --connect-timeout 2 --max-time 8 "$BASE/api/health" > "$OUT_DIR/health.json"; then
    break
  fi
  if ! kill -0 "$PID" >/dev/null 2>&1; then
    fail_with_tail 1 "webui exited before health became ready" "$SERVER_LOG"
  fi
  sleep 1
  if [ "$attempt" = "90" ]; then
    fail_with_tail 1 "webui health did not become ready" "$SERVER_LOG"
  fi
done

step "create multistep build conversation"
curl -fsS --connect-timeout 2 --max-time 20 \
  -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  --data-binary '{"title":"multistep app build with Forge WebUI"}' > "$CREATED_JSON"
CONV_ID="$(json_id "$CREATED_JSON")"
test -n "$CONV_ID"

cat > "$PROMPT_FILE" <<PROMPT
Build a tiny multistep repo artifact through the normal WebUI tools.

CRITICAL: A final answer before the shell validation command is a failed run.
You must use exactly this tool sequence before the final answer:
1. file_write
2. file_write
3. shell_command

Required tool calls:
1. Use file_write to create $SPEC. Include this exact marker on its own line: $SPEC_MARKER. Also include a one-line acceptance rule saying the report must reference this spec file path.
2. Use file_write to create $REPORT. Include this exact marker on its own line: $REPORT_MARKER. The report must reference $SPEC by path.
3. Use shell_command to run exactly: $VALIDATION_CMD
4. Only after the shell_command result, final answer with files modified, tests run, unresolved risks, and confidence.

Do not inspect the repo. Do not do broad analysis. Do not create other files. Do not stop after the two file writes.
PROMPT
jq -Rs '{message: ., max_rounds: 8}' "$PROMPT_FILE" > "$REQUEST_JSON"

step "run multistep app build through WebUI"
set +e
timeout "${TIMEOUT_SECONDS}s" curl -fsS --connect-timeout 2 --max-time "$((TIMEOUT_SECONDS - 10))" \
  -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  --data-binary "@$REQUEST_JSON" > "$STREAM_OUT"
STREAM_RC=$?
set -e

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON" || true

step "capture browser proof"
curl -fsS --connect-timeout 2 --max-time 60 \
  -X POST "$BASE/api/browser-proof" \
  -H 'content-type: application/json' \
  --data-binary "$(jq -n --arg url "$BASE/?conversation=$CONV_ID&proof=final" '{url:$url,width:1440,height:1000,capture_dom:true}')" > "$BROWSER_JSON" || true
if jq -e '.success == true and (.screenshot_base64 | length > 1000)' "$BROWSER_JSON" >/dev/null 2>&1; then
  jq -r '.screenshot_base64' "$BROWSER_JSON" | base64 -d > "$SCREENSHOT" || true
fi

python3 - "$STREAM_OUT" "$CONVERSATION_JSON" "$BROWSER_JSON" "$SCREENSHOT" "$SPEC" "$REPORT" "$SPEC_MARKER" "$REPORT_MARKER" "$VALIDATION_CMD" "$SUMMARY" "$SUMMARY_MD" "$STREAM_RC" <<'PY'
import json, sys
from pathlib import Path
stream_path = Path(sys.argv[1])
conv_path = Path(sys.argv[2])
proof_path = Path(sys.argv[3])
shot_path = Path(sys.argv[4])
spec_path = Path(sys.argv[5])
report_path = Path(sys.argv[6])
spec_marker = sys.argv[7]
report_marker = sys.argv[8]
validation_cmd = sys.argv[9]
summary_path = Path(sys.argv[10])
md_path = Path(sys.argv[11])
rc = int(sys.argv[12])
text = stream_path.read_text(encoding="utf-8", errors="replace") if stream_path.exists() else ""
conversation = json.loads(conv_path.read_text(encoding="utf-8", errors="replace")) if conv_path.exists() and conv_path.stat().st_size else {}
proof = json.loads(proof_path.read_text(encoding="utf-8", errors="replace")) if proof_path.exists() and proof_path.stat().st_size else {}
all_text = text + "\n" + json.dumps(conversation, sort_keys=True)
provider = conversation.get("provider") or ""
model = conversation.get("model") or ""
spec_exists = spec_path.is_file()
report_exists = report_path.is_file()
spec_text = spec_path.read_text(encoding="utf-8", errors="replace") if spec_exists else ""
report_text = report_path.read_text(encoding="utf-8", errors="replace") if report_exists else ""
checks = []
def check(name, passed, evidence=None):
    item = {"name": name, "passed": bool(passed)}
    if evidence is not None:
        item["evidence"] = evidence
    checks.append(item)
check("stream_exit_zero", rc == 0, rc)
check("provider_is_nvidia_nim", provider == "nvidia_nim" or '"provider":"nvidia_nim"' in text, provider)
check("model_recorded", bool(model) or '"model":"' in text, model)
check("run_finished", "event: run-finish" in text)
check("two_file_write_steps_seen", all(p in all_text for p in [str(spec_path), str(report_path)]) and all_text.count("file_write") >= 2, all_text.count("file_write"))
check("shell_validation_seen", "shell_command" in all_text and validation_cmd in all_text)
check("spec_file_exists", spec_exists, str(spec_path))
check("report_file_exists", report_exists, str(report_path))
check("spec_marker_present", spec_marker in spec_text)
check("report_marker_present", report_marker in report_text)
check("report_references_spec", str(spec_path) in report_text)
check("final_answer_shape", all(tok in all_text.lower() for tok in ["files", "tests", "risks", "confidence"]))
check("browser_success", proof.get("success") is True)
check("screenshot_png", shot_path.is_file() and shot_path.stat().st_size > 1024 and shot_path.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n", shot_path.stat().st_size if shot_path.exists() else 0)
failed = [c for c in checks if not c["passed"]]
out = {"passed": not failed, "provider": provider, "model": model, "spec": str(spec_path), "report": str(report_path), "checks": checks, "failed_checks": failed, "screenshot_path": str(shot_path)}
summary_path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n", encoding="utf-8")
md_path.write_text("# App multistep build proof\n\n" + "\n".join(f"- {c['name']}: `{str(c['passed']).lower()}`" for c in checks) + "\n", encoding="utf-8")
print(json.dumps(out, indent=2, sort_keys=True))
raise SystemExit(0 if not failed else 1)
PY

jq -e '.passed == true' "$SUMMARY" >/dev/null
step "done"
echo "App multistep build proof passed: $SUMMARY"
