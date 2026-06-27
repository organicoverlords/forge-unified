#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
SERVER_LOG="$PROOF_DIR/server.log"
STREAM_OUT="$PROOF_DIR/screenshot-stream.sse"
FILE_TOOL_STREAM="$PROOF_DIR/file-tool-stream.sse"
APPROVAL_JSON="$PROOF_DIR/approval-response.json"
EVENT_BUS_JSON="$PROOF_DIR/event-bus.json"
EVENT_STATUS_JSON="$PROOF_DIR/event-status.json"
CONVERSATION_JSON="$PROOF_DIR/screenshot-conversation.json"
BROWSER_PROOF_JSON="$PROOF_DIR/browser-proof.json"
EVENT_PAGE_JSON="$PROOF_DIR/event-page-proof.json"
SCREENSHOT_PNG="$PROOF_DIR/webui.png"
EVENT_PAGE_PNG="$PROOF_DIR/event-rail.png"
STATUS_OUT="$PROOF_DIR/git-status.txt"
PROMPT_FILE="$PROOF_DIR/screenshot-prompt.txt"
REQUEST_JSON="$PROOF_DIR/screenshot-request.json"
NOTE_PATH="forge-proof/live-webui-feature-sprint/natural-proof-note.txt"
FILE_TOOL_PATH="forge-proof/live-webui-feature-sprint/file-tool-event-proof.rs"
mkdir -p "$PROOF_DIR"
rm -f "$NOTE_PATH" "$FILE_TOOL_PATH" "$STREAM_OUT" "$FILE_TOOL_STREAM" "$APPROVAL_JSON" "$EVENT_BUS_JSON" "$EVENT_STATUS_JSON" "$EVENT_PAGE_JSON" "$BROWSER_PROOF_JSON" "$SCREENSHOT_PNG" "$EVENT_PAGE_PNG"

cargo build --workspace
RUST_BACKTRACE=1 cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; git status --short > "$STATUS_OUT" 2>/dev/null || true; }
trap cleanup EXIT

curl_with_retry() {
  local url="$1"
  local out="${2:-}"
  local attempt rc
  for attempt in $(seq 1 120); do
    if ! kill -0 "$PID" >/dev/null 2>&1; then
      echo "::error::forge-app exited while waiting for $url" >&2
      tail -n 220 "$SERVER_LOG" >&2 || true
      return 1
    fi
    set +e
    if [[ -n "$out" ]]; then
      curl -fsS --retry 1 --retry-delay 1 --connect-timeout 2 --max-time 10 "$url" -o "$out"
      rc=$?
    else
      curl -fsS --retry 1 --retry-delay 1 --connect-timeout 2 --max-time 10 "$url"
      rc=$?
    fi
    set -e
    if [[ "$rc" -eq 0 ]]; then return 0; fi
    if [[ "$attempt" == "1" || "$attempt" == "20" || "$attempt" == "60" || "$attempt" == "120" ]]; then
      echo "waiting for $url attempt=$attempt rc=$rc" >&2
      tail -n 80 "$SERVER_LOG" >&2 || true
    fi
    sleep 1
  done
  echo "::error::timed out waiting for $url" >&2
  tail -n 220 "$SERVER_LOG" >&2 || true
  return 1
}

latest_conversation_from_sse() {
  local sse_file="$1"
  local out_file="$2"
  awk 'prev == "event: conversation" { sub(/^data: /, ""); print } { prev = $0 }' "$sse_file" | tail -n 1 > "$out_file"
  jq -e '.messages | length >= 1' "$out_file" >/dev/null
}

wait_for_webui() {
  local health="$PROOF_DIR/health.json"
  local index="$PROOF_DIR/index.html"
  local events="$PROOF_DIR/events.html"
  curl_with_retry "$BASE/api/health" "$health"
  curl_with_retry "$BASE/" "$index"
  grep -q "Forge Unified" "$index"
  curl_with_retry "$BASE/events?static=1" "$events"
  grep -q "Forge Activity" "$events"
  curl_with_retry "$BASE/api/events/recent" "$EVENT_BUS_JSON"
  grep -q '"event_bus":"change_bus"' "$EVENT_BUS_JSON"
  curl_with_retry "$BASE/api/events/status" "$EVENT_STATUS_JSON"
  grep -q '"bridge_shape":"opencode_event_v2_bridge_status"' "$EVENT_STATUS_JSON"
  for marker in "watcher_backend" "watcher_native_binding" "watcher_subscribe_timeout_ms" "watcher_ignore_patterns" "contained_event_bridge_without_native_subscription" "packages/core/src/filesystem/watcher.ts"; do grep -Fq "$marker" "$EVENT_STATUS_JSON"; done
}

wait_for_webui

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" -H 'content-type: application/json' -d '{"title":"natural mutable toolpart lifecycle proof"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$CONV_ID"

cat > "$PROMPT_FILE" <<'PROMPT'
Please create a short proof note for this WebUI sprint, then summarize what changed in plain English so I can see the work finished.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"
curl -fsS --retry 2 --retry-delay 1 --connect-timeout 2 --max-time 120 -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$REQUEST_JSON" > "$STREAM_OUT"
for marker in "event: run-start" "event: run-finish" "event: tool-lifecycle" "event: tool-input-start" "event: tool-call" "pending_edit_approval" "approval_state" "permission_request" "natural-proof-note.txt" "packages/opencode/src/session/processor.ts" "ToolStatePending" "ToolStateRunning"; do grep -Fq "$marker" "$STREAM_OUT"; done
if test -e "$NOTE_PATH"; then echo "::error::file appeared before approval"; exit 4; fi

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
APPROVAL_ID="$(jq -r '.messages[]? | .tool_results[]? | .metadata.pending_edit_approval.approval_id? // empty' "$CONVERSATION_JSON" | head -n 1)"
test -n "$APPROVAL_ID"

curl -fsS --retry 2 --retry-delay 1 --connect-timeout 2 --max-time 60 -X POST "$BASE/api/conversations/$CONV_ID/approvals/$APPROVAL_ID/approve" -H 'content-type: application/json' -d '{"approved":true}' > "$APPROVAL_JSON"
for marker in '"approval_applied":true' '"applied":true' "Updated 1 file" "file_events" "event_bus_receipts" "event_bus_status" "opencode_event_v2_bridge_status" "filesystem.edited" "watcher.updated" "lsp.warmup.contained" "lsp.diagnostics" "LSP.Warmup.contained" "warmup_contained" "severity_counts" "diagnostic_count" "report_block" "max_per_file" "lsp_client_status" "packages/opencode/src/lsp/diagnostic.ts" "natural-proof-note.txt"; do grep -Fq "$marker" "$APPROVAL_JSON"; done

test -s "$NOTE_PATH"

cat > "$PROMPT_FILE" <<'PROMPT'
Please run an OpenCode file tool formatter proof: write a temporary Rust file named forge-proof/live-webui-feature-sprint/file-tool-event-proof.rs with deliberately compact but valid Rust, edit it once, delete it, then summarize the emitted formatter, watcher, LSP, and streamed ToolPart lifecycle events.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"
curl -fsS --retry 2 --retry-delay 1 --connect-timeout 2 --max-time 120 -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$REQUEST_JSON" > "$FILE_TOOL_STREAM"
for marker in "event: run-start" "event: run-finish" "event: tool-lifecycle" "event: tool-input-start" "event: tool-input-delta" "event: tool-input-end" "event: tool-call" "event: tool-result" "file_write" "file_edit" "file_delete" "file-tool-event-proof.rs" "opencode_file_tool_source" "opencode_event_publisher" "opencode.file_tool" "bom_strategy" "writeTextPreservingBom" "formatter_status" "opencode_formatter_source" "Format.file" "rustfmt" "packages/opencode/src/format/index.ts" "packages/core/src/file-mutation.ts" "packages/opencode/src/session/processor.ts" "SessionProcessor ensureToolCall/updateToolCall/completeToolCall/failToolCall lifecycle" "ToolStatePending" "ToolStateRunning" "ToolStateCompleted" "file.added" "file.edited" "file.deleted" "FileSystem.Event.Edited" "Watcher.Event.Updated" "LSP.Warmup.contained" "LSP.Diagnostic.report" "severity_counts" "diagnostic_count" "report_block" "max_per_file" "packages/opencode/src/lsp/diagnostic.ts" "lsp.warmup.contained" "lsp.diagnostics"; do grep -Fq "$marker" "$FILE_TOOL_STREAM"; done
if test -e "$FILE_TOOL_PATH"; then echo "::error::file tool proof file remained after delete"; exit 5; fi
latest_conversation_from_sse "$FILE_TOOL_STREAM" "$CONVERSATION_JSON"
for marker in "attachments_schema" "ToolStateCompleted.attachments" '"attachments"' '"type":"file"' '"identifier":"FilePart"' '"path":"packages/schema/src/v1/session.ts"' '"tool":"file_write"' "opencode_session_processor" "packages/opencode/src/session/processor.ts" "mutable_tool_part_updates" "opencode_mutable_tool_part_source" "before_status" "after_status" "same ToolPart row updated by callID" "bom_preserved" "bom_strategy" "writeTextPreservingBom" "formatter_status" "opencode_formatter_source" "Format.file" "packages/opencode/src/format/index.ts" "packages/core/src/file-mutation.ts"; do grep -Fq "$marker" "$CONVERSATION_JSON"; done
jq -e '[.messages[]?.metadata.mutable_tool_part_updates[]? | select(.before_status == "running" and .after_status == "completed")] | length >= 1' "$CONVERSATION_JSON" >/dev/null

curl_with_retry "$BASE/api/events/recent" "$EVENT_BUS_JSON"
jq -e '.count >= 12 and .status.bridge_shape == "opencode_event_v2_bridge_status"' "$EVENT_BUS_JSON" >/dev/null
for marker in '"event_bus":"change_bus"' '"event_type":"filesystem.edited"' '"event_type":"watcher.updated"' '"event_type":"lsp.warmup.contained"' '"event_type":"lsp.diagnostics"' "natural-proof-note.txt" "file-tool-event-proof.rs" "opencode.apply_patch" "opencode.file_tool" "severity_counts" "diagnostic_count" "max_per_file" "latest_files" "by_type" "by_source"; do grep -Fq "$marker" "$EVENT_BUS_JSON"; done
curl_with_retry "$BASE/api/events/status" "$EVENT_STATUS_JSON"
for marker in "opencode_event_v2_bridge_status" "filesystem.edited" "watcher.updated" "lsp.diagnostics" "latest_files" "watcher_backend" "watcher_subscribe_timeout_ms" "watcher_ignore_patterns" "contained_event_bridge_without_native_subscription" "packages/core/src/filesystem/watcher.ts" "packages/opencode/src/tool/write.ts" "packages/opencode/src/tool/edit.ts"; do grep -Fq "$marker" "$EVENT_STATUS_JSON"; done

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/snapshot" -H 'content-type: application/json' -d '{}' >/dev/null
curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/compact" -H 'content-type: application/json' -d '{"keep_last":2,"auto":false,"overflow":true}' > "$PROOF_DIR/compaction-response.json"
for marker in "session.next.compaction.started" "session.next.compaction.ended" "opencode.session.compaction" "event_bus_receipts" "event_bus_status" "packages/core/src/session/compaction.ts"; do grep -Fq "$marker" "$PROOF_DIR/compaction-response.json"; done
curl_with_retry "$BASE/api/events/recent" "$EVENT_BUS_JSON"
for marker in '"event_type":"session.next.compaction.started"' '"event_type":"session.next.compaction.ended"' "opencode.session.compaction" "packages/core/src/session/compaction.ts"; do grep -Fq "$marker" "$EVENT_BUS_JSON"; done
curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
for marker in "compaction_parts" "compaction_summary" "compaction_recent" "## Goal" "## Critical Context" "packages/core/src/session/compaction.ts"; do grep -Fq "$marker" "$CONVERSATION_JSON"; done

curl -fsS --retry 2 --retry-delay 1 --connect-timeout 2 --max-time 60 -X POST "$BASE/api/browser-proof" -H 'content-type: application/json' -d "{\"url\":\"$BASE/\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" > "$BROWSER_PROOF_JSON"
jq -e '.success == true' "$BROWSER_PROOF_JSON" >/dev/null
jq -r '.screenshot_base64' "$BROWSER_PROOF_JSON" | base64 -d > "$SCREENSHOT_PNG"
test -s "$SCREENSHOT_PNG"
for marker in "natural mutable toolpart lifecycle proof" "main-chat-event-rail" "OpenCode Activity" "EventV2Bridge-style recent filesystem and watcher activity" "filesystem.edited" "watcher.updated" "lsp.warmup.contained" "lsp.diagnostics" "OpenCode ToolPart metadata" "OpenCode PatchPart" "OpenCode CompactionPart" "packages/opencode/src/session/processor.ts" "OpenCode SessionProcessor lifecycle receipts"; do grep -Fq "$marker" "$BROWSER_PROOF_JSON"; done

curl -fsS --retry 2 --retry-delay 1 --connect-timeout 2 --max-time 60 -X POST "$BASE/api/browser-proof" -H 'content-type: application/json' -d "{\"url\":\"$BASE/events?static=1\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" > "$EVENT_PAGE_JSON"
jq -e '.success == true' "$EVENT_PAGE_JSON" >/dev/null
jq -r '.screenshot_base64' "$EVENT_PAGE_JSON" | base64 -d > "$EVENT_PAGE_PNG"
test -s "$EVENT_PAGE_PNG"
for marker in "Forge Activity" "Live event rail" "OpenCode-style EventV2Bridge" "Bridge status" "opencode_event_v2_bridge_status" "filesystem.edited" "watcher.updated" "lsp.warmup.contained" "lsp.diagnostics" "session.next.compaction.started" "session.next.compaction.ended" "natural-proof-note.txt" "opencode-event-rail" "static proof mode"; do grep -Fq "$marker" "$EVENT_PAGE_JSON"; done

echo "LIVE WebUI natural event bridge proof passed: $BASE conversation=$CONV_ID screenshot=$SCREENSHOT_PNG event_rail=$EVENT_PAGE_PNG status=$EVENT_STATUS_JSON"
