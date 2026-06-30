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
  if command -v dbus-run-session >/dev/null 2>&1; then
    printf 'env -u DBUS_SESSION_BUS_ADDRESS -u DBUS_SYSTEM_BUS_ADDRESS NO_AT_BRIDGE=1 dbus-run-session -- '
  else
    printf 'env -u DBUS_SESSION_BUS_ADDRESS -u DBUS_SYSTEM_BUS_ADDRESS NO_AT_BRIDGE=1 '
  fi
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
    'metadata': {'capture': 'direct_chrome_smoke_harness', 'endpoint_bypassed': True, 'chrome_dbus_wrapped': wrapped == 'true'},
}
Path(json_path).write_text(json.dumps(out, ensure_ascii=False, indent=2, sort_keys=True) + '\n', encoding='utf-8')
PY
}

capture_direct() {
  local url="$1" json_file="$2" png_file="$3" label="$4"
  local dom_file="$PROOF_DIR/${label}-dom.html"
  local log_file="$PROOF_DIR/${label}-chrome.log"
  local dom_log_file="$PROOF_DIR/${label}-dom-chrome.log"
  local profile_dir="$PROOF_DIR/${label}-chrome-profile"
  local dom_profile_dir="$PROOF_DIR/${label}-dom-chrome-profile"
  mkdir -p "$profile_dir" "$dom_profile_dir"
  curl -fsS --connect-timeout 2 --max-time 20 "$url" -o "$dom_file"
  local chrome wrapped prefix
  chrome="$(find_chrome)" || { printf 'chrome not found\n' > "$log_file"; write_browser_json "$url" "$dom_file" "$png_file" "$json_file" "$log_file" false false; return 1; }
  if command -v dbus-run-session >/dev/null 2>&1; then wrapped=true; else wrapped=false; fi
  prefix="$(chrome_prefix)"
  set +e
  timeout 60s bash -c "$prefix \"$chrome\" \
    --headless=chrome \
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
    --hide-scrollbars \
    --mute-audio \
    --no-first-run \
    --run-all-compositor-stages-before-draw \
    --user-data-dir=\"$profile_dir\" \
    --screenshot=\"$png_file\" \
    --window-size=1440,1000 \
    --timeout=30000 \
    --virtual-time-budget=8000 \
    \"$url\"" > "$log_file" 2>&1
  local rc=$?
  timeout 35s bash -c "$prefix \"$chrome\" \
    --headless=chrome \
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
    --hide-scrollbars \
    --mute-audio \
    --no-first-run \
    --run-all-compositor-stages-before-draw \
    --user-data-dir=\"$dom_profile_dir\" \
    --dump-dom \
    --timeout=16000 \
    --virtual-time-budget=8000 \
    \"$url\"" > "$dom_file.tmp" 2> "$dom_log_file"
  local dom_rc=$?
  set -e
  if [ "$dom_rc" -eq 0 ] && [ -s "$dom_file.tmp" ]; then mv "$dom_file.tmp" "$dom_file"; else rm -f "$dom_file.tmp"; fi
  cat "$dom_log_file" >> "$log_file" 2>/dev/null || true
  if [ "$rc" -ne 0 ] && [ ! -s "$png_file" ]; then
    write_browser_json "$url" "$dom_file" "$png_file" "$json_file" "$log_file" false "$wrapped"
    return 1
  fi
  write_browser_json "$url" "$dom_file" "$png_file" "$json_file" true "$wrapped"
}

capture_direct "$BASE/?conversation=$CONV_ID$QUERY_SUFFIX" "$BROWSER_JSON" "$WEBUI_PNG" browser
jq -e '.success == true' "$BROWSER_JSON" >/dev/null
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
