#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [ "${FORGE_FAST_WEBUI_LINTED:-0}" != "1" ]; then
  FORGE_FAST_WEBUI_LINTED=1 bash -n "$0"
fi

PORT="${FORGE_FAST_PORT:-3340}"
BASE="http://127.0.0.1:${PORT}"
OUT_DIR="${FORGE_FAST_OUT:-$ROOT/forge-proof/fast-webui-proof}"
TIMEOUT_SECONDS="${FORGE_FAST_TIMEOUT_SECONDS:-140}"
mkdir -p "$OUT_DIR"

SERVER_LOG="$OUT_DIR/server.log"
STEP_LOG="$OUT_DIR/steps.log"
INDEX_HTML="$OUT_DIR/index.html"
CATALOG_JSON="$OUT_DIR/tool-catalog.json"
CREATED_JSON="$OUT_DIR/created.json"
REQUEST_JSON="$OUT_DIR/request.json"
STREAM_OUT="$OUT_DIR/chat-stream.sse"
CONVERSATION_JSON="$OUT_DIR/conversation.json"
BROWSER_JSON="$OUT_DIR/browser-proof.json"
SCREENSHOT="$OUT_DIR/webui.png"
SUMMARY="$OUT_DIR/summary.json"
SUMMARY_MD="$OUT_DIR/summary.md"
STATUS_OUT="$OUT_DIR/git-status.txt"
: > "$STEP_LOG"

step() { printf '[%s] %s\n' "$(date -u +%H:%M:%S)" "$*" | tee -a "$STEP_LOG"; }

fail_with_tail() {
  local code="$1"
  local msg="$2"
  local file="${3:-}"
  echo "::error::$msg" >&2
  if [ -n "$file" ] && [ -f "$file" ]; then
    tail -n 120 "$file" >&2 || true
  fi
  exit "$code"
}

cleanup() {
  if [ -n "${PID:-}" ]; then
    kill "$PID" >/dev/null 2>&1 || true
  fi
  git status --short > "$STATUS_OUT" 2>/dev/null || true
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

step "check static UI markers"
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/" -o "$INDEX_HTML"
for marker in \
  "Forge Unified" \
  "provider-model-visible" \
  "human-tool-label" \
  "proof-digest-visible" \
  "Session timeline" \
  "session-turn-central" \
  "session-turn-diffs-group" \
  "copy-retry-actions" \
  "collapsible-tool-card" \
  "deferred-technical-content" \
  "timeline-file-diff-groups" \
  "timeline-action-groups" \
  "turn-receipt-toolbar" \
  "file-diff-summary-visible" \
  "stable-session-receipts" \
  "human-action-summary"; do
  grep -Fq "$marker" "$INDEX_HTML" || fail_with_tail 2 "missing UI marker $marker" "$INDEX_HTML"
done
if grep -Fq "OpenCode-style" "$INDEX_HTML"; then
  fail_with_tail 2 "visible OpenCode-style branding leaked into Forge UI" "$INDEX_HTML"
fi

step "check provider tool catalog"
curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/tools" -o "$CATALOG_JSON"
for marker in "forge_provider_tool_catalog" "provider_visible" "apply_patch" "todo_write" "batch_parallel" "task"; do
  grep -Fq "$marker" "$CATALOG_JSON" || fail_with_tail 3 "missing tool catalog marker $marker" "$CATALOG_JSON"
done

step "create fast live conversation"
curl -fsS --connect-timeout 2 --max-time 20 \
  -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  --data-binary '{"title":"fast WebUI proof"}' > "$CREATED_JSON"
CONV_ID="$(json_id "$CREATED_JSON")"
test -n "$CONV_ID"

cat > "$OUT_DIR/prompt.txt" <<'PROMPT'
Reply exactly with: LIVE_FAST_WEBUI_PROOF provider route ok.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$OUT_DIR/prompt.txt" > "$REQUEST_JSON"

step "run fast NIM/WebUI stream"
set +e
timeout "${TIMEOUT_SECONDS}s" curl -fsS --connect-timeout 2 --max-time "$((TIMEOUT_SECONDS - 10))" \
  -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  --data-binary "@$REQUEST_JSON" > "$STREAM_OUT"
STREAM_RC=$?
set -e

curl -fsS --connect-timeout 2 --max-time 20 "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON" || true

step "capture readable browser proof (non-blocking for route capability)"
CAPTURE_RC=0
bash scripts/smoke/capture-browser-proof.sh "$BASE" "$CONV_ID" "" "$OUT_DIR" "fast" "&proof=final" || CAPTURE_RC=$?
if [ "$CAPTURE_RC" -ne 0 ]; then
  step "browser proof capture degraded rc=$CAPTURE_RC; route capability checks continue"
fi

python3 - "$STREAM_OUT" "$CONVERSATION_JSON" "$BROWSER_JSON" "$SCREENSHOT" "$SUMMARY" "$SUMMARY_MD" "$STREAM_RC" "$CAPTURE_RC" "$INDEX_HTML" "$CATALOG_JSON" <<'PY'
import json, sys
from pathlib import Path
stream = Path(sys.argv[1])
conv = Path(sys.argv[2])
proof = Path(sys.argv[3])
shot = Path(sys.argv[4])
summary = Path(sys.argv[5])
summary_md = Path(sys.argv[6])
rc = int(sys.argv[7])
capture_rc = int(sys.argv[8])
index_html = Path(sys.argv[9])
catalog_json = Path(sys.argv[10])
text = stream.read_text(encoding="utf-8", errors="replace") if stream.exists() else ""
conversation = json.loads(conv.read_text(encoding="utf-8", errors="replace")) if conv.exists() and conv.stat().st_size else {}
proof_data = json.loads(proof.read_text(encoding="utf-8", errors="replace")) if proof.exists() and proof.stat().st_size else {}
provider = conversation.get("provider") or ""
model = conversation.get("model") or ""
proof_text = json.dumps(proof_data)
static_text = (index_html.read_text(encoding="utf-8", errors="replace") if index_html.exists() else "") + "\n" + (catalog_json.read_text(encoding="utf-8", errors="replace") if catalog_json.exists() else "")
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
check("fast_marker_seen", "LIVE_FAST_WEBUI_PROOF" in text or "LIVE_FAST_WEBUI_PROOF" in json.dumps(conversation))
check("static_readable_ui_hooks_present", all(m in static_text for m in ["Run proof summary", "Final answer", "human-tool-label", "human-action-summary", "session-turn-central", "collapsible-tool-card"]))
check("tool_catalog_static_ui_seen", all(m in static_text for m in ["Available actions", "Run tools in parallel", "Apply patch", "todo_write", "batch_parallel"]))
check("no_opencode_style_branding", "OpenCode-style" not in static_text)
check("raw_json_not_primary_static_result", "{&quot;" not in static_text)
check("browser_capture_command_zero", capture_rc == 0, capture_rc, required=False)
check("browser_success", proof_data.get("success") is True, proof_data.get("error"), required=False)
check("readable_proof_ui", all(m in proof_text for m in ["Run proof summary", "Final answer", "human-tool-label"]), required=False)
check("central_session_turn_ui", all(m in proof_text for m in ["Session timeline", "Turn 1:", "copy final answer", "copy turn", "retry"]), required=False)
check("session_part_hooks_visible", all(m in proof_text for m in ["session-turn-central", "assistant-parts", "message-part", "copy-retry-actions"]), required=False)
check("tool_card_hooks_visible", all(m in proof_text for m in ["collapsible-tool-card", "deferred-technical-content", "provider-model-visible"]), required=False)
check("turn_receipt_grouping_visible", all(m in proof_text for m in ["Receipts grouped by turn", "file receipts", "tool cards", "copy receipts", "stable-session-receipts"]), required=False)
check("timeline_actions_visible", all(m in proof_text for m in ["copy timeline", "retry latest prompt", "copy latest files", "timeline-action-groups"]), required=False)
check("screenshot_png", shot.is_file() and shot.stat().st_size > 1024 and shot.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n", shot.stat().st_size if shot.exists() else 0, required=False)
failed_required = [c for c in checks if c["required"] and not c["passed"]]
degraded = [c for c in checks if not c["required"] and not c["passed"]]
out = {
    "passed": not failed_required,
    "route_capability_passed": not failed_required,
    "browser_capture_passed": not degraded,
    "browser_capture_required": False,
    "provider": provider,
    "model": model,
    "stream_path": str(stream),
    "conversation_path": str(conv),
    "screenshot_path": str(shot),
    "checks": checks,
    "failed_checks": failed_required,
    "degraded_checks": degraded,
}
summary.write_text(json.dumps(out, indent=2, sort_keys=True) + "\n", encoding="utf-8")
summary_md.write_text("# Fast WebUI proof\n\n" + "\n".join(f"- {c['name']}: `{str(c['passed']).lower()}`" + ("" if c["required"] else " _(non-blocking)_") for c in checks) + "\n", encoding="utf-8")
print(json.dumps(out, indent=2, sort_keys=True))
raise SystemExit(0 if not failed_required else 1)
PY

jq -e '.passed == true' "$SUMMARY" >/dev/null
step "done"
echo "FAST WebUI proof passed: $SUMMARY"
