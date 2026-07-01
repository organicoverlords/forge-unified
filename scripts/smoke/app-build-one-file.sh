#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [ "${FORGE_APP_BUILD_ONE_FILE_LINTED:-0}" != "1" ]; then
  FORGE_APP_BUILD_ONE_FILE_LINTED=1 bash -n "$0"
fi

PORT="${FORGE_APP_BUILD_PORT:-3350}"
BASE="http://127.0.0.1:${PORT}"
OUT_DIR="${FORGE_APP_BUILD_OUT:-$ROOT/forge-proof/app-build-proof}"
TIMEOUT_SECONDS="${FORGE_APP_BUILD_TIMEOUT_SECONDS:-220}"
TARGET="docs/generated/proof/app-built-one-file.md"
MARKER="APP_BUILD_PROOF_WEBUI_NIM"
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
BUILT_COPY="$OUT_DIR/app-built-one-file.md"
DIFF_FILE="$OUT_DIR/app-build.diff"
STATUS_OUT="$OUT_DIR/git-status.txt"
: > "$STEP_LOG"

step() { printf '[%s] %s\n' "$(date -u +%H:%M:%S)" "$*" | tee -a "$STEP_LOG"; }

fail_with_tail() {
  local code="$1"
  local msg="$2"
  local file="${3:-}"
  echo "::error::$msg" >&2
  if [ -n "$file" ] && [ -f "$file" ]; then
    tail -n 140 "$file" >&2 || true
  fi
  exit "$code"
}

cleanup() {
  if [ -n "${PID:-}" ]; then
    kill "$PID" >/dev/null 2>&1 || true
  fi
  git status --short > "$STATUS_OUT" 2>/dev/null || true
  git diff -- "$TARGET" > "$DIFF_FILE" 2>/dev/null || true
  if [ -f "$TARGET" ]; then cp "$TARGET" "$BUILT_COPY"; fi
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

step "create build conversation"
curl -fsS --connect-timeout 2 --max-time 20 \
  -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  --data-binary '{"title":"build one file with Forge WebUI"}' > "$CREATED_JSON"
CONV_ID="$(json_id "$CREATED_JSON")"
test -n "$CONV_ID"

cat > "$PROMPT_FILE" <<PROMPT
Use the file_write tool to create exactly this file: $TARGET

File content must include this exact marker on its own line: $MARKER

Keep the content short. Do not inspect the repo. Do not run shell commands. After writing the file, final answer with files modified, tests run, unresolved risks, and confidence.
PROMPT
jq -Rs '{message: ., max_rounds: 3}' "$PROMPT_FILE" > "$REQUEST_JSON"

step "run one-file app build through WebUI"
set +e
timeout "${TIMEOUT_SECONDS}s" curl -fsS --connect-timeout 2 --max-time "$((TIMEOUT_SECONDS - 10))" \
  -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  --data-binary "@$REQUEST_JSON" > "$STREAM_OUT"
STREAM_RC=$?
set -e

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON" || true

step "capture browser proof (non-blocking for app capability)"
CAPTURE_RC=0
bash scripts/smoke/capture-browser-proof.sh "$BASE" "$CONV_ID" "" "$OUT_DIR" "fast" "&proof=final" || CAPTURE_RC=$?
if [ "$CAPTURE_RC" -ne 0 ]; then
  step "browser proof capture degraded rc=$CAPTURE_RC; app capability checks continue"
fi

python3 - "$STREAM_OUT" "$CONVERSATION_JSON" "$BROWSER_JSON" "$SCREENSHOT" "$TARGET" "$MARKER" "$SUMMARY" "$SUMMARY_MD" "$STREAM_RC" "$CAPTURE_RC" <<'PY'
import json, sys
from pathlib import Path
stream_path, conv_path, proof_path, shot_path, target_path, marker, summary_path, md_path = [Path(x) for x in sys.argv[1:9]]
rc = int(sys.argv[9])
capture_rc = int(sys.argv[10])
text = stream_path.read_text(encoding="utf-8", errors="replace") if stream_path.exists() else ""
conversation = json.loads(conv_path.read_text(encoding="utf-8", errors="replace")) if conv_path.exists() and conv_path.stat().st_size else {}
proof = json.loads(proof_path.read_text(encoding="utf-8", errors="replace")) if proof_path.exists() and proof_path.stat().st_size else {}
provider = conversation.get("provider") or ""
model = conversation.get("model") or ""
target_exists = target_path.is_file()
target_text = target_path.read_text(encoding="utf-8", errors="replace") if target_exists else ""
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
check("file_write_tool_seen", "file_write" in text)
check("target_path_seen", str(target_path) in text or str(target_path) in json.dumps(conversation))
check("target_file_exists", target_exists, str(target_path))
check("target_marker_present", str(marker) in target_text)
check("final_answer_shape", all(tok in (text + json.dumps(conversation)).lower() for tok in ["files", "tests", "risks", "confidence"]))
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
    "target": str(target_path),
    "marker": str(marker),
    "checks": checks,
    "failed_checks": failed_required,
    "degraded_checks": failed_optional,
    "screenshot_path": str(shot_path),
}
summary_path.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n", encoding="utf-8")
md_path.write_text("# App build one-file proof\n\n" + "\n".join(f"- {c['name']}: `{str(c['passed']).lower()}`" + ("" if c["required"] else " _(non-blocking)_") for c in checks) + "\n", encoding="utf-8")
print(json.dumps(out, indent=2, sort_keys=True))
raise SystemExit(0 if not failed_required else 1)
PY

jq -e '.passed == true' "$SUMMARY" >/dev/null
step "done"
echo "App built file proof passed: $SUMMARY"
