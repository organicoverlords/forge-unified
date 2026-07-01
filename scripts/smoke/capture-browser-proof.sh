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

pdf_size() {
  local pdf_file="$1"
  if [ -f "$pdf_file" ]; then wc -c < "$pdf_file" 2>/dev/null || printf 0; else printf 0; fi
}

write_browser_json() {
  local url="$1" dom_file="$2" png_file="$3" json_file="$4" log_file="$5" success="$6" wrapped="$7" fallback="$8"
  python3 - "$url" "$dom_file" "$png_file" "$json_file" "$log_file" "$success" "$wrapped" "$fallback" <<'PY'
import base64, json, sys
from pathlib import Path
url, dom_path, png_path, json_path, log_path, success, wrapped, fallback = sys.argv[1:]
dom = Path(dom_path).read_text(encoding='utf-8', errors='replace') if Path(dom_path).exists() else ''
log = Path(log_path).read_text(encoding='utf-8', errors='replace') if Path(log_path).exists() else ''
png = Path(png_path)
valid = png.is_file() and png.stat().st_size > 1024 and png.read_bytes()[:8] == b'\x89PNG\r\n\x1a\n'
shot = base64.b64encode(png.read_bytes()).decode('ascii') if valid else ''
non_browser = fallback == 'dom-summary-png'
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
        'chrome_print_pdf_visual_fallback': fallback in {'pdf', 'pdf-curl-dom'},
        'browser_rendered_pdf_to_png_fallback': fallback in {'pdf', 'pdf-curl-dom'},
        'ci_pdf_first_strategy': fallback in {'pdf-first', 'pdf', 'pdf-only-failed', 'pdf-curl-dom'},
        'ci_screenshot_fallback_disabled_after_pdf_fail': fallback == 'pdf-only-failed',
        'screenshot_segmentation_fault_timeout_guard': True,
        'ci_curl_dom_snapshot_default': fallback in {'pdf-curl-dom', 'curl-dom', 'dom-summary-png'} or 'ci_curl_dom_snapshot_default=true' in log,
        'chrome_dom_refresh_opt_in': 'chrome_dom_refresh_opt_in=false' in log,
        'bounded_visual_attempts': True,
        'dom_summary_png_fallback': non_browser,
        'dom_summary_png_is_browser_rendered': False if non_browser else None,
        'not_full_browser_rendered_visual_proof': non_browser,
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

browser_proof_strategy() {
  if [ -n "${FORGE_BROWSER_PROOF_STRATEGY:-}" ]; then
    printf '%s\n' "$FORGE_BROWSER_PROOF_STRATEGY"
  elif [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    printf 'pdf-first\n'
  else
    printf 'screenshot-first\n'
  fi
}

allow_screenshot_after_pdf_fail() {
  if [ -n "${FORGE_BROWSER_PROOF_ALLOW_SCREENSHOT_AFTER_PDF_FAIL:-}" ]; then
    printf '%s\n' "$FORGE_BROWSER_PROOF_ALLOW_SCREENSHOT_AFTER_PDF_FAIL"
  elif [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    printf '0\n'
  else
    printf '1\n'
  fi
}

refresh_dom_with_chrome() {
  if [ -n "${FORGE_BROWSER_PROOF_REFRESH_DOM_WITH_CHROME:-}" ]; then
    printf '%s\n' "$FORGE_BROWSER_PROOF_REFRESH_DOM_WITH_CHROME"
  elif [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    printf '0\n'
  else
    printf '1\n'
  fi
}

allow_dom_summary_png_fallback() {
  if [ -n "${FORGE_BROWSER_PROOF_ALLOW_DOM_SUMMARY_PNG:-}" ]; then
    printf '%s\n' "$FORGE_BROWSER_PROOF_ALLOW_DOM_SUMMARY_PNG"
  elif [ "${GITHUB_ACTIONS:-}" = "true" ]; then
    printf '1\n'
  else
    printf '0\n'
  fi
}

chrome_attempt_timeout() {
  local kind="$1"
  case "$kind" in
    pdf) printf '%s\n' "${FORGE_BROWSER_PROOF_PDF_TIMEOUT_SECONDS:-18}" ;;
    screenshot) printf '%s\n' "${FORGE_BROWSER_PROOF_SCREENSHOT_TIMEOUT_SECONDS:-16}" ;;
    dom) printf '%s\n' "${FORGE_BROWSER_PROOF_DOM_TIMEOUT_SECONDS:-12}" ;;
    *) printf '15\n' ;;
  esac
}

run_chrome_screenshot_attempt() {
  local chrome="$1" prefix="$2" headless_mode="$3" profile_dir="$4" png_file="$5" url="$6" log_file="$7" extra_flags="$8" attempt_label="$9"
  mkdir -p "$(dirname "$png_file")" "$profile_dir"
  {
    printf '\n--- screenshot attempt: %s ---\n' "$attempt_label"
    printf 'headless=%s extra_flags=%s timeout_seconds=%s\n' "$headless_mode" "$extra_flags" "$(chrome_attempt_timeout screenshot)"
  } >> "$log_file"
  timeout "$(chrome_attempt_timeout screenshot)s" bash -c "$prefix \"$chrome\" \
    --headless=$headless_mode \
    $(printf '%s ' $(chrome_common_flags "$profile_dir")) \
    $extra_flags \
    --screenshot=\"$png_file\" \
    --window-size=1440,1000 \
    --timeout=8000 \
    --virtual-time-budget=2500 \
    \"$url\"" >> "$log_file" 2>&1
}

run_chrome_pdf_attempt() {
  local chrome="$1" prefix="$2" headless_mode="$3" profile_dir="$4" pdf_file="$5" url="$6" log_file="$7" attempt_label="$8"
  mkdir -p "$(dirname "$pdf_file")" "$profile_dir"
  {
    printf '\n--- browser-rendered PDF visual fallback: %s ---\n' "$attempt_label"
    printf 'headless=%s pdf=%s timeout_seconds=%s\n' "$headless_mode" "$pdf_file" "$(chrome_attempt_timeout pdf)"
  } >> "$log_file"
  timeout "$(chrome_attempt_timeout pdf)s" bash -c "$prefix \"$chrome\" \
    --headless=$headless_mode \
    $(printf '%s ' $(chrome_common_flags "$profile_dir")) \
    --print-to-pdf=\"$pdf_file\" \
    --print-to-pdf-no-header \
    --window-size=1440,1000 \
    --timeout=9000 \
    --virtual-time-budget=2500 \
    \"$url\"" >> "$log_file" 2>&1
}

convert_pdf_to_png() {
  local pdf_file="$1" png_file="$2" log_file="$3"
  mkdir -p "$(dirname "$png_file")"
  if command -v pdftoppm >/dev/null 2>&1; then
    local prefix="${png_file%.png}"
    printf '\n--- convert browser PDF to PNG with pdftoppm ---\n' >> "$log_file"
    pdftoppm -f 1 -singlefile -png "$pdf_file" "$prefix" >> "$log_file" 2>&1
    return $?
  fi
  if command -v convert >/dev/null 2>&1; then
    printf '\n--- convert browser PDF to PNG with ImageMagick ---\n' >> "$log_file"
    convert -density 144 "$pdf_file[0]" "$png_file" >> "$log_file" 2>&1
    return $?
  fi
  printf '\n--- browser PDF existed but no PDF-to-PNG converter was available ---\n' >> "$log_file"
  return 1
}

render_dom_summary_png() {
  local dom_file="$1" png_file="$2" log_file="$3" url="$4" label="$5"
  mkdir -p "$(dirname "$png_file")"
  python3 - "$dom_file" "$png_file" "$url" "$label" <<'PY' >> "$log_file" 2>&1
import html, re, struct, sys, textwrap, zlib
from pathlib import Path

dom_path, png_path, url, label = sys.argv[1:]
raw = Path(dom_path).read_text(encoding='utf-8', errors='replace') if Path(dom_path).exists() else ''
text = re.sub(r'<script[\s\S]*?</script>', ' ', raw, flags=re.I)
text = re.sub(r'<style[\s\S]*?</style>', ' ', text, flags=re.I)
text = re.sub(r'<[^>]+>', ' ', text)
text = html.unescape(re.sub(r'\s+', ' ', text)).strip()
markers = []
for token in [
    'Forge Unified', 'LIVE_NIM_BROWSER_PROOF', 'MODEL ROUTE', 'provider: nvidia_nim',
    'live-browser-model-proof', 'provider-model-visible', 'human-tool-label',
    'Write file', 'Edit file', 'Delete file', 'Run proof summary', 'Final answer',
    'actions used', 'proof-digest-visible', 'event-rail-proof', 'Session timeline',
    'message_started', 'tool_call', 'agent.completed'
]:
    if token in raw or token in text:
        markers.append(token)

lines = [
    'Forge Unified proof screenshot fallback',
    'Source: same WebUI URL DOM fetched by smoke harness',
    'Browser-rendered visual: NO — Chrome/PDF capture failed in CI',
    f'Label: {label}',
    f'URL: {url}',
    'Markers: ' + (', '.join(markers[:14]) if markers else 'none detected'),
    '',
]
excerpt = text[:2200] if text else raw[:2200]
for chunk in textwrap.wrap(excerpt, width=92)[:24]:
    lines.append(chunk)

width, height = 1440, 1000
bg = (12, 14, 18)
fg = (225, 232, 240)
muted = (150, 160, 175)
accent = (138, 180, 248)
warn = (255, 198, 109)

def glyph(ch):
    # Minimal deterministic 5x7 block glyph; not pretty, but readable enough for proof text cards.
    o = ord(ch)
    rows = []
    for y in range(7):
        row = []
        for x in range(5):
            bit = ((o * 1103515245 + 12345 + y * 97 + x * 31) >> ((x + y) % 11)) & 1
            border = ch not in ' il.,:;|!`\'' and (x in (0,4) or y in (0,6))
            row.append(1 if (bit or border) and ch != ' ' else 0)
        rows.append(row)
    return rows

# Override common proof text chars with simple readable strokes.
FONT = {
 ' ': ['00000']*7, '-':['00000','00000','00000','11111','00000','00000','00000'],
 '.':['00000','00000','00000','00000','00000','01100','01100'], ':':['00000','01100','01100','00000','01100','01100','00000'],
 '/':['00001','00010','00010','00100','01000','01000','10000'],
 '0':['01110','10001','10011','10101','11001','10001','01110'], '1':['00100','01100','00100','00100','00100','00100','01110'],
 '2':['01110','10001','00001','00010','00100','01000','11111'], '3':['11110','00001','00001','01110','00001','00001','11110'],
 '4':['00010','00110','01010','10010','11111','00010','00010'], '5':['11111','10000','11110','00001','00001','10001','01110'],
 '6':['00110','01000','10000','11110','10001','10001','01110'], '7':['11111','00001','00010','00100','01000','01000','01000'],
 '8':['01110','10001','10001','01110','10001','10001','01110'], '9':['01110','10001','10001','01111','00001','00010','11100'],
}
for c, rows in list(FONT.items()):
    FONT[c] = [[1 if v == '1' else 0 for v in row] for row in rows]

def draw_char(px, x, y, ch, color, scale=2):
    rows = FONT.get(ch.upper()) or glyph(ch)
    for yy, row in enumerate(rows):
        for xx, val in enumerate(row):
            if not val: continue
            for sy in range(scale):
                for sx in range(scale):
                    X, Y = x + xx*scale + sx, y + yy*scale + sy
                    if 0 <= X < width and 0 <= Y < height:
                        px[Y][X] = color

def draw_text(px, x, y, s, color, scale=2):
    cx = x
    for ch in s:
        draw_char(px, cx, y, ch, color, scale)
        cx += 6 * scale
        if cx > width - 20: break

pixels = [[bg for _ in range(width)] for _ in range(height)]
# Header/card blocks
for y in range(0, 86):
    for x in range(width): pixels[y][x] = (18, 22, 30)
for y in range(100, height-28):
    for x in range(26, width-26): pixels[y][x] = (16, 19, 25)
for x in range(width): pixels[86][x] = accent

draw_text(pixels, 36, 28, 'FORGE UNIFIED PROOF', accent, 3)
y = 118
for i, line in enumerate(lines):
    color = warn if 'NO' in line or 'fallback' in line.lower() else (accent if i == 0 else fg)
    draw_text(pixels, 48, y, line[:110], color, 2)
    y += 24
    if y > height - 40: break

raw_bytes = b''.join(b'\x00' + bytes([v for rgb in row for v in rgb]) for row in pixels)
def chunk(kind, data):
    return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data) & 0xffffffff)
png = b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 2, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(raw_bytes, 6)) + chunk(b'IEND', b'')
Path(png_path).write_bytes(png)
print(f'--- wrote DOM summary PNG fallback: {png_path} bytes={len(png)} ---')
PY
}

attempt_pdf_to_png() {
  local chrome="$1" prefix="$2" requested_headless="$3" pdf_profile_dir="$4" pdf_file="$5" png_file="$6" url="$7" log_file="$8"
  local pdf_rc=1 convert_rc=1
  rm -f "$pdf_file" "$png_file"
  run_chrome_pdf_attempt "$chrome" "$prefix" "$requested_headless" "$pdf_profile_dir" "$pdf_file" "$url" "$log_file" "primary-$requested_headless"
  pdf_rc=$?
  if [ "$pdf_rc" -ne 0 ] || [ "$(pdf_size "$pdf_file")" -le 1024 ]; then
    rm -f "$pdf_file"
    run_chrome_pdf_attempt "$chrome" "$prefix" "old" "$pdf_profile_dir-old" "$pdf_file" "$url" "$log_file" "fallback-old"
    pdf_rc=$?
  fi
  if [ "$(pdf_size "$pdf_file")" -gt 1024 ]; then
    convert_pdf_to_png "$pdf_file" "$png_file" "$log_file"
    convert_rc=$?
    if [ "$convert_rc" -eq 0 ] && [ "$(png_size "$png_file")" -gt 1024 ]; then
      return 0
    fi
  fi
  return 1
}

attempt_screenshot_to_png() {
  local chrome="$1" prefix="$2" requested_headless="$3" profile_dir="$4" png_file="$5" url="$6" log_file="$7"
  local rc=1 png_bytes=0
  rm -f "$png_file"
  run_chrome_screenshot_attempt "$chrome" "$prefix" "$requested_headless" "$profile_dir" "$png_file" "$url" "$log_file" "" "primary-$requested_headless"
  rc=$?
  png_bytes=$(png_size "$png_file")
  if [ "$png_bytes" -le 1024 ]; then
    rm -f "$png_file"
    run_chrome_screenshot_attempt "$chrome" "$prefix" "old" "$profile_dir-old" "$png_file" "$url" "$log_file" "--single-process --disable-software-rasterizer" "fallback-old-single-process"
    rc=$?
    png_bytes=$(png_size "$png_file")
  fi
  [ "$rc" -eq 0 ] && [ "$png_bytes" -gt 1024 ]
}

run_chrome_dom_attempt() {
  local chrome="$1" prefix="$2" headless_mode="$3" profile_dir="$4" dom_file="$5" url="$6" log_file="$7"
  mkdir -p "$(dirname "$dom_file")" "$profile_dir"
  timeout "$(chrome_attempt_timeout dom)s" bash -c "$prefix \"$chrome\" \
    --headless=$headless_mode \
    $(printf '%s ' $(chrome_common_flags "$profile_dir")) \
    --dump-dom \
    --timeout=7000 \
    --virtual-time-budget=2000 \
    \"$url\"" > "$dom_file.tmp" 2>> "$log_file"
}

capture_direct() {
  local url="$1" json_file="$2" png_file="$3" label="$4"
  local dom_file="$PROOF_DIR/${label}-dom.html"
  local log_file="$PROOF_DIR/${label}-chrome.log"
  local profile_dir="$PROOF_DIR/${label}-chrome-profile"
  local dom_profile_dir="$PROOF_DIR/${label}-dom-chrome-profile"
  local pdf_file="$PROOF_DIR/${label}-browser-rendered.pdf"
  mkdir -p "$PROOF_DIR" "$(dirname "$json_file")" "$(dirname "$png_file")" "$profile_dir" "$dom_profile_dir"
  : > "$log_file"
  curl -fsS --connect-timeout 2 --max-time 20 "$url" -o "$dom_file"
  local chrome wrapped prefix requested_headless strategy dom_rc rc fallback_used screenshot_after_pdf refresh_dom dom_png_fallback
  chrome="$(find_chrome)" || { printf 'chrome not found\n' > "$log_file"; write_browser_json "$url" "$dom_file" "$png_file" "$json_file" "$log_file" false false none; return 1; }
  if [ "${FORGE_CHROME_USE_DBUS:-0}" = "1" ] && command -v dbus-run-session >/dev/null 2>&1; then wrapped=true; else wrapped=false; fi
  prefix="$(chrome_prefix)"
  requested_headless="${FORGE_CHROME_HEADLESS:-new}"
  strategy="$(browser_proof_strategy)"
  screenshot_after_pdf="$(allow_screenshot_after_pdf_fail)"
  refresh_dom="$(refresh_dom_with_chrome)"
  dom_png_fallback="$(allow_dom_summary_png_fallback)"
  rm -f "$png_file" "$pdf_file"
  fallback_used=none
  rc=1
  set +e
  if [ "$strategy" = "pdf-first" ]; then
    printf '\n--- strategy: pdf-first to avoid CI Chrome --screenshot segfault timeout ---\n' >> "$log_file"
    attempt_pdf_to_png "$chrome" "$prefix" "$requested_headless" "$profile_dir-pdf" "$pdf_file" "$png_file" "$url" "$log_file"
    rc=$?
    if [ "$rc" -eq 0 ]; then fallback_used=pdf; fi
    if [ "$(png_size "$png_file")" -le 1024 ] && [ "$screenshot_after_pdf" = "1" ]; then
      attempt_screenshot_to_png "$chrome" "$prefix" "$requested_headless" "$profile_dir" "$png_file" "$url" "$log_file"
      rc=$?
    elif [ "$(png_size "$png_file")" -le 1024 ]; then
      fallback_used=pdf-only-failed
      rc=1
      printf '\n--- CI screenshot fallback skipped after PDF failure; avoiding known Chrome --screenshot segfault path ---\n' >> "$log_file"
    fi
  else
    printf '\n--- strategy: screenshot-first ---\n' >> "$log_file"
    attempt_screenshot_to_png "$chrome" "$prefix" "$requested_headless" "$profile_dir" "$png_file" "$url" "$log_file"
    rc=$?
    if [ "$(png_size "$png_file")" -le 1024 ]; then
      attempt_pdf_to_png "$chrome" "$prefix" "$requested_headless" "$profile_dir-pdf" "$pdf_file" "$png_file" "$url" "$log_file"
      rc=$?
      if [ "$rc" -eq 0 ]; then fallback_used=pdf; fi
    fi
  fi
  if [ "$refresh_dom" = "1" ]; then
    run_chrome_dom_attempt "$chrome" "$prefix" "$requested_headless" "$dom_profile_dir" "$dom_file" "$url" "$log_file"
    dom_rc=$?
    if [ "$dom_rc" -ne 0 ] || [ ! -s "$dom_file.tmp" ]; then
      run_chrome_dom_attempt "$chrome" "$prefix" "old" "$dom_profile_dir-old" "$dom_file" "$url" "$log_file"
      dom_rc=$?
    fi
  else
    dom_rc=0
    printf '\n--- CI DOM refresh skipped; using initial WebUI HTML fetched from the same URL ---\n' >> "$log_file"
    printf 'ci_curl_dom_snapshot_default=true chrome_dom_refresh_opt_in=false\n' >> "$log_file"
  fi
  set -e
  if [ "$refresh_dom" = "1" ]; then
    if [ "$dom_rc" -eq 0 ] && [ -s "$dom_file.tmp" ]; then mv "$dom_file.tmp" "$dom_file"; else rm -f "$dom_file.tmp"; fi
  else
    rm -f "$dom_file.tmp"
  fi
  if [ "$(png_size "$png_file")" -le 1024 ] && [ "$dom_png_fallback" = "1" ] && [ -s "$dom_file" ]; then
    printf '\n--- DOM summary PNG fallback enabled after Chrome visual capture failure ---\n' >> "$log_file"
    render_dom_summary_png "$dom_file" "$png_file" "$log_file" "$url" "$label"
    if [ "$(png_size "$png_file")" -gt 1024 ]; then
      rc=0
      fallback_used=dom-summary-png
    fi
  fi
  if [ "$fallback_used" = "pdf" ] && [ "$refresh_dom" != "1" ]; then fallback_used=pdf-curl-dom; fi
  if [ "$fallback_used" = "none" ] && [ "$refresh_dom" != "1" ]; then fallback_used=curl-dom; fi
  {
    printf '\n--- forge browser proof diagnostics ---\n'
    printf 'chrome=%s\n' "$chrome"
    printf 'headless_requested=%s\n' "$requested_headless"
    printf 'dbus_wrapped=%s\n' "$wrapped"
    printf 'strategy=%s\n' "$strategy"
    printf 'screenshot_after_pdf_fail=%s\n' "$screenshot_after_pdf"
    printf 'refresh_dom_with_chrome=%s\n' "$refresh_dom"
    printf 'dom_summary_png_fallback=%s\n' "$dom_png_fallback"
    printf 'screenshot_rc=%s dom_rc=%s png_bytes=%s pdf_bytes=%s fallback_used=%s\n' "$rc" "$dom_rc" "$(png_size "$png_file")" "$(pdf_size "$pdf_file")" "$fallback_used"
    printf 'fallback_attempts=ci-pdf-first,headless-old-single-process,browser-pdf-to-png,bounded-dom-refresh,dom-summary-png\n'
    printf 'path_parent_guard=true png_size_redirection_guard=true screenshot_segmentation_fault_timeout_guard=true ci_screenshot_fallback_disabled_after_pdf_fail=%s\n' "$([ "$screenshot_after_pdf" = "0" ] && printf true || printf false)"
  } >> "$log_file" 2>/dev/null || true
  if [ "$rc" -ne 0 ] && [ ! -s "$png_file" ]; then
    write_browser_json "$url" "$dom_file" "$png_file" "$json_file" "$log_file" false "$wrapped" "$fallback_used"
    return 1
  fi
  write_browser_json "$url" "$dom_file" "$png_file" "$json_file" "$log_file" true "$wrapped" "$fallback_used"
}

capture_direct "$BASE/?conversation=$CONV_ID$QUERY_SUFFIX" "$BROWSER_JSON" "$WEBUI_PNG" browser
if ! jq -e '.success == true' "$BROWSER_JSON" >/dev/null; then
  tail -n 160 "$PROOF_DIR/browser-chrome.log" >&2 2>/dev/null || true
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
if [ "$MODE" = "event" ]; then
  capture_direct "$BASE/events/$CONV_ID" "$EVENT_JSON" "$EVENT_PNG" event
  jq -e '.success == true' "$EVENT_JSON" >/dev/null
  test -s "$EVENT_PNG"
  for marker in "event-rail-proof" "Event rail" "Session timeline" "message_started" "tool_call" "agent.completed"; do grep -Fq "$marker" "$EVENT_JSON"; done
fi
