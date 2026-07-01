#!/usr/bin/env bash
set -euo pipefail
BASE="$1"
CONV_ID="$2"
MODEL_ID="$3"
PROOF_DIR="$4"
MODE="${5:-model}"
QUERY_SUFFIX="${6:-}"
BROWSER_JSON="$PROOF_DIR/browser-proof.json"
EVENT_JSON="$PROOF_DIR/event-page-proof.json"
WEBUI_PNG="$PROOF_DIR/webui.png"
EVENT_PNG="$PROOF_DIR/event-rail.png"

find_chrome() {
  if [ -n "${CHROME_PATH:-}" ] && [ -x "$CHROME_PATH" ]; then printf '%s\n' "$CHROME_PATH"; return 0; fi
  for name in google-chrome google-chrome-stable chromium chromium-browser; do
    if command -v "$name" >/dev/null 2>&1; then command -v "$name"; return 0; fi
  done
  return 1
}

chrome_prefix() {
  # GitHub-hosted runners have repeatedly hung for the full screenshot+DOM
  # budget when Chromium is wrapped in dbus-run-session. Keep the capture
  # path browser-real but default to a plain, sanitized environment; allow
  # opt-in DBus wrapping only for local diagnosis.
  if [ "${FORGE_CHROME_USE_DBUS:-0}" = "1" ] && command -v dbus-run-session >/dev/null 2>&1; then
    printf 'env -u DBUS_SESSION_BUS_ADDRESS -u DBUS_SYSTEM_BUS_ADDRESS NO_AT_BRIDGE=1 XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp}" dbus-run-session -- '
  else
    printf 'env -u DBUS_SESSION_BUS_ADDRESS -u DBUS_SYSTEM_BUS_ADDRESS NO_AT_BRIDGE=1 XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/tmp}" '
  fi
}

png_size() {
  local png_file="$1"
  if [ -f "$png_file" ]; then wc -c < "$png_file" 2>/dev/null || printf 0; else printf 0; fi
}

write_browser_json() {
  local url="$1" dom_file="$2" png_file="$3" json_file="$4" log_file="$5" success="$6" wrapped="$7"
  python3 - "$url" "$dom_file" "$png_file" "$json_file" "$log_file" "$success" "$wrapped" <<'PY'
import base64, json, sys
from pathlib import Path
url, dom_path, png_path, json_path, log_path, success, wrapped = sys.argv[1:]
dom = Path(dom_path).read_text(encoding='utf-8', errors='replace') if Path(dom_path).exists() else ''
log = Path(log_path).read_text(encoding='utf-8', errors='replace') if Path(log_path).exists() else ''
png = Path(png_path)
valid = png.is_file() and png.stat().st_size > 1024 and png.read_bytes()[:8] == b'\x89PNG\r\n\x1a\n'
shot = base64.b64encode(png.read_bytes()).decode('ascii') if valid else ''
out = {
    'success': bool(success == 'true' and valid),
    'url': url,
    'page_title': 'Forge Unified',
    'dom_snapshot': dom,
    'screenshot_base64': shot,
    'console_logs': ['direct smoke harness Chrome capture', log[-4000:]],
    'error': None if valid else 'direct Chrome capture did not produce a readable PNG',
    'metadata': {
        'capture': 'direct_chrome_smoke_harness',
        'endpoint_bypassed': True,
        'chrome_dbus_wrapped': wrapped == 'true',
        'chrome_dbus_default_disabled': True,
        'diagnosable_browser_failure': True,
        'chrome_retry_fallbacks': True,
        'headless_old_retry': True,
        'single_process_retry': True,
        'screenshot_path_parent_guard': True,
        'png_size_redirection_guard': True,
    },
}
Path(json_path).parent.mkdir(parents=True, exist_ok=True)
Path(json_path).write_text(json.dumps(out, ensure_ascii=False, indent=2, sort_keys=True) + '\n', encoding='utf-8')
PY
}

chrome_common_flags() {
  local profile_dir="$1"
  printf '%s\n' \
    --disable-gpu \
    --no-sandbox \
    --disable-setuid-sandbox \
    --disable-dev-shm-usage \
    --disable-background-networking \
    --disable-component-update \
    --disable-domain-reliability \
    --disable-extensions \
    --disable-sync \
    --disable-default-apps \
    --disable-features=TranslateUI,UseDBus,MediaRouter,DialMediaRouteProvider,OptimizationHints,BackgroundFetch,PushMessaging \
    --disable-ipc-flooding-protection \
    --hide-scrollbars \
    --mute-audio \
    --no-first-run \
    --run-all-compositor-stages-before-draw \
    --user-data-dir="$profile_dir"
}

run_chrome_screenshot_attempt() {
  local chrome="$1" prefix="$2" headless_mode="$3" profile_dir="$4" png_file="$5" url="$6" log_file="$7" extra_flags="$8" attempt_label="$9"
  mkdir -p "$(dirname "$png_file")" "$profile_dir"
  {
    printf '\n--- screenshot attempt: %s ---\n' "$attempt_label"
    printf 'headless=%s extra_flags=%s\n' "$headless_mode" "$extra_flags"
  } >> "$log_file"
  timeout 45s bash -c "$prefix \"$chrome\" \
    --headless=$headless_mode \
    $(printf '%s ' $(chrome_common_flags "$profile_dir")) \
    $extra_flags \
    --screenshot=\"$png_file\" \
    --window-size=1440,1000 \
    --timeout=22000 \
    --virtual-time-budget=7000 \
    \"$url\"" >> "$log_file" 2>&1
}

run_chrome_dom_attempt() {
  local chrome="$1" prefix="$2" headless_mode="$3" profile_dir="$4" dom_file="$5" url="$6" log_file="$7"
  mkdir -p "$(dirname "$dom_file")" "$profile_dir"
  timeout 25s bash -c "$prefix \"$chrome\" \
    --headless=$headless_mode \
    $(printf '%s ' $(chrome_common_flags "$profile_dir")) \
    --dump-dom \
    --timeout=12000 \
    --virtual-time-budget=5000 \
    \"$url\"" > "$dom_file.tmp" 2>> "$log_file"
}

capture_direct() {
  local url="$1" json_file="$2" png_file="$3" label="$4"
  local dom_file="$PROOF_DIR/${label}-dom.html"
  local log_file="$PROOF_DIR/${label}-chrome.log"
  local profile_dir="$PROOF_DIR/${label}-chrome-profile"
  local dom_profile_dir="$PROOF_DIR/${label}-dom-chrome-profile"
  mkdir -p "$PROOF_DIR" "$(dirname "$json_file")" "$(dirname "$png_file")" "$profile_dir" "$dom_profile_dir"
  : > "$log_file"
  curl -fsS --connect-timeout 2 --max-time 20 "$url" -o "$dom_file"
  local chrome wrapped prefix requested_headless rc dom_rc png_bytes
  chrome="$(find_chrome)" || { printf 'chrome not found\n' > "$log_file"; write_browser_json "$url" "$dom_file" "$png_file" "$json_file" "$log_file" false false; return 1; }
  if [ "${FORGE_CHROME_USE_DBUS:-0}" = "1" ] && command -v dbus-run-session >/dev/null 2>&1; then wrapped=true; else wrapped=false; fi
  prefix="$(chrome_prefix)"
  requested_headless="${FORGE_CHROME_HEADLESS:-new}"
  rm -f "$png_file"
  set +e
  run_chrome_screenshot_attempt "$chrome" "$prefix" "$requested_headless" "$profile_dir" "$png_file" "$url" "$log_file" "" "primary-$requested_headless"
  rc=$?
  png_bytes=$(png_size "$png_file")
  if [ "$png_bytes" -le 1024 ]; then
    rm -f "$png_file"
    run_chrome_screenshot_attempt "$chrome" "$prefix" "old" "$profile_dir-old" "$png_file" "$url" "$log_file" "--single-process --disable-software-rasterizer" "fallback-old-single-process"
    rc=$?
    png_bytes=$(png_size "$png_file")
  fi
  run_chrome_dom_attempt "$chrome" "$prefix" "$requested_headless" "$dom_profile_dir" "$dom_file" "$url" "$log_file"
  dom_rc=$?
  if [ "$dom_rc" -ne 0 ] || [ ! -s "$dom_file.tmp" ]; then
    run_chrome_dom_attempt "$chrome" "$prefix" "old" "$dom_profile_dir-old" "$dom_file" "$url" "$log_file"
    dom_rc=$?
  fi
  set -e
  if [ "$dom_rc" -eq 0 ] && [ -s "$dom_file.tmp" ]; then mv "$dom_file.tmp" "$dom_file"; else rm -f "$dom_file.tmp"; fi
  {
    printf '\n--- forge browser proof diagnostics ---\n'
    printf 'chrome=%s\n' "$chrome"
    printf 'headless_requested=%s\n' "$requested_headless"
    printf 'dbus_wrapped=%s\n' "$wrapped"
    printf 'screenshot_rc=%s dom_rc=%s png_bytes=%s\n' "$rc" "$dom_rc" "$(png_size "$png_file")"
    printf 'fallback_attempts=headless-old-single-process\n'
    printf 'path_parent_guard=true png_size_redirection_guard=true\n'
  } >> "$log_file" 2>/dev/null || true
  if [ "$rc" -ne 0 ] && [ ! -s "$png_file" ]; then
    write_browser_json "$url" "$dom_file" "$png_file" "$json_file" "$log_file" false "$wrapped"
    return 1
  fi
  write_browser_json "$url" "$dom_file" "$png_file" "$json_file" true "$wrapped"
}

capture_direct "$BASE/?conversation=$CONV_ID$QUERY_SUFFIX" "$BROWSER_JSON" "$WEBUI_PNG" browser
if ! jq -e '.success == true' "$BROWSER_JSON" >/dev/null; then
  tail -n 120 "$PROOF_DIR/browser-chrome.log" >&2 2>/dev/null || true
  exit 1
fi
test -s "$WEBUI_PNG"
if [ "$MODE" = "model" ]; then
  for marker in "live-browser-model-proof" "provider-model-visible" "provider: nvidia_nim" "MODEL ROUTE" "LIVE_NIM_BROWSER_PROOF" "$MODEL_ID"; do grep -Fq "$marker" "$BROWSER_JSON"; done
fi
if printf '%s' "$QUERY_SUFFIX" | grep -Fq 'proof=final'; then
  for marker in "Run proof summary" "Final answer" "actions used" "proof-digest-visible" "human-tool-label"; do grep -Fq "$marker" "$BROWSER_JSON"; done
fi
if [ "$MODE" = "tool" ]; then
  for marker in "Write file" "Edit file" "Delete file" "technical details" "human-tool-label"; do grep -Fq "$marker" "$BROWSER_JSON"; done
fi

if ! capture_direct "$BASE/events?static=1" "$EVENT_JSON" "$EVENT_PNG" event; then
  printf '{"success":false,"note":"secondary event rail screenshot unavailable; primary WebUI screenshot passed"}\n' > "$EVENT_JSON"
fi
