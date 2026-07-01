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
rm -f docs/generated/proof/app-multistep-*.md

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
CREATED_LIST="$OUT_DIR/app-multistep-files.txt"
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
  find docs/generated/proof -maxdepth 1 -type f -name 'app-multistep-*.md' -print | sort > "$CREATED_LIST" 2>/dev/null || true
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
  --data-binary '{"title":"real multistep app build with Forge WebUI"}' > "$CREATED_JSON"
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
1. Use file_write to create $SPEC with exactly this content:
# App Multistep Spec
$SPEC_MARKER
Acceptance: $REPORT must reference $SPEC.

2. Use file_write to create $REPORT with exactly this content:
# App Multistep Report
$REPORT_MARKER
Spec reference: $SPEC

3. Use shell_command to run exactly: $VALIDATION_CMD
4. Only after the shell_command result, final answer with files modified, tests run, unresolved risks, and confidence.

Do not inspect the repo. Do not do broad analysis. Do not create other files. Do not write PLACEHOLDER anywhere. Do not stop after the two file writes.
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
find docs/generated/proof -maxdepth 1 -type f -name 'app-multistep-*.md' -print | sort > "$CREATED_LIST" 2>/dev/null || true

step "capture browser proof (non-blocking for app capability)"
CAPTURE_RC=0
bash scripts/smoke/capture-browser-proof.sh "$BASE" "$CONV_ID" "" "$OUT_DIR" "fast" "&proof=final" || CAPTURE_RC=$?
if [ "$CAPTURE_RC" -ne 0 ]; then
  step "browser proof capture degraded rc=$CAPTURE_RC; multistep capability checks continue"
fi

python3 - "$STREAM_OUT" "$CONVERSATION_JSON" "$BROWSER_JSON" "$SCREENSHOT" "$SPEC" "$REPORT" "$SPEC_MARKER" "$REPORT_MARKER" "$VALIDATION_CMD" "$CREATED_LIST" "$SUMMARY" "$SUMMARY_MD" "$STREAM_RC" "$CAPTURE_RC" <<'PY'
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
created_list_path = Path(sys.argv[10])
summary_path = Path(sys.argv[11])
md_path = Path(sys.argv[12])
rc = int(sys.argv[13])
capture_rc = int(sys.argv[14])
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
created_files = [line.strip() for line in created_list_path.read_text(encoding="utf-8", errors="replace").splitlines() if line.strip()] if created_list_path.exists() else []
expected_files = sorted([str(spec_path), str(report_path)])

def has_exact_line(body, line):
    return line in [part.strip() for part in body.splitlines()]

checks = []
def check(name, passed, evidence=None, required=True):
    item = {"name": name, "passed": bool(passed), "required": bool(required)}
    if evidence is not None:
        item["evidence"] = evidence
    checks.append(item)
check("stream_exit_zero", rc == 0, rc)
check("provider_is_nvidia_nim", provider == "nvidia_nim" or '"provider":"nvidia_nim"' in text, provider)
check("model_recorded", bool(model) or '"model":"' in text, model)
check("run_finished", "event: run-finish" in text)
check("required_sequence_repair_recorded", "forge_required_tool_sequence_repairs" in all_text)
check("two_file_write_steps_seen", all(p in all_text for p in [str(spec_path), str(report_path)]) and all_text.count("file_write") >= 2, all_text.count("file_write"))
check("shell_validation_seen", "shell_command" in all_text and validation_cmd in all_text)
check("spec_file_exists", spec_exists, str(spec_path))
check("report_file_exists", report_exists, str(report_path))
check("no_extra_app_multistep_files", created_files == expected_files, created_files)
check("spec_marker_exact_line", has_exact_line(spec_text, spec_marker))
check("report_marker_exact_line", has_exact_line(report_text, report_marker))
check("no_placeholder_content", "PLACEHOLDER" not in spec_text and "PLACEHOLDER" not in report_text)
check("report_references_spec", str(spec_path) in report_text)
check("final_answer_shape", all(tok in all_text.lower() for tok in ["files", "tests", "risks", "confidence"]))
check("browser_capture_command_zero", capture_rc == 0, capture_rc, required=False)
check("browser_success", proof.get("success") is True, proof.get("error"), required=False)
check("screenshot_png", shot_path.is_file() and shot_path.stat().st_size > 1024 and shot_path.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n", shot_path.stat().st_size if shot_path.exists() else 0, required=False)
failed_required = [c for c in checks if c["required"] and not c["passed"]]
failed_optional = [c for c in checks if not c["required"] and not c["passed"]]
out = {
    "passed": not failed_required,
    "app_capability_passed": not failed_required,
    "browser_capture_passed": not failed_optional,
    "browser_capture_required": False,
    "provider": provider,
    "model": model,
    "spec": str(spec_path),
    "report": str(report_path),
    "created_app_multistep_files": created_files,
    "checks": checks,
    "failed_checks": failed_required,
    "degraded_checks": failed_optional,
    "screenshot_path": str(shot_path),
}
summary_path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n", encoding="utf-8")
md_path.write_text("# App multistep build proof\n\n" + "\n".join(f"- {c['name']}: `{str(c['passed']).lower()}`" + ("" if c["required"] else " _(non-blocking)_") for c in checks) + "\n", encoding="utf-8")
print(json.dumps(out, indent=2, sort_keys=True))
raise SystemExit(0 if not failed_required else 1)
PY

jq -e '.passed == true' "$SUMMARY" >/dev/null
step "done"
echo "App multistep build proof passed: $SUMMARY"
