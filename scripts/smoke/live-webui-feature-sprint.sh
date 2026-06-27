#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
SERVER_LOG="$PROOF_DIR/server.log"
STREAM_OUT="$PROOF_DIR/screenshot-stream.sse"
APPROVAL_JSON="$PROOF_DIR/approval-response.json"
EVENT_BUS_JSON="$PROOF_DIR/event-bus.json"
CONVERSATION_JSON="$PROOF_DIR/screenshot-conversation.json"
BROWSER_PROOF_JSON="$PROOF_DIR/browser-proof.json"
SCREENSHOT_PNG="$PROOF_DIR/webui.png"
STATUS_OUT="$PROOF_DIR/git-status.txt"
PROMPT_FILE="$PROOF_DIR/screenshot-prompt.txt"
REQUEST_JSON="$PROOF_DIR/screenshot-request.json"
NOTE_PATH="forge-proof/live-webui-feature-sprint/natural-proof-note.txt"
mkdir -p "$PROOF_DIR"
rm -f "$NOTE_PATH" "$STREAM_OUT" "$APPROVAL_JSON" "$EVENT_BUS_JSON"

cargo build --workspace
cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; git status --short > "$STATUS_OUT" 2>/dev/null || true; }
trap cleanup EXIT

for _ in $(seq 1 60); do curl -fsS "$BASE/api/health" >/dev/null && break; sleep 0.5; done
curl -fsS "$BASE/" | grep -q "Forge Unified"
curl -fsS "$BASE/api/events/recent" | grep -q '"event_bus":"change_bus"'

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" -H 'content-type: application/json' -d '{"title":"natural event bus proof"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$CONV_ID"

cat > "$PROMPT_FILE" <<'PROMPT'
Please create a short proof note for this WebUI sprint.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"
curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$REQUEST_JSON" > "$STREAM_OUT"
for marker in "event: run-start" "event: run-finish" "event: tool-result" "pending_edit_approval" "approval_state" "permission_request" "natural-proof-note.txt"; do grep -Fq "$marker" "$STREAM_OUT"; done
if test -e "$NOTE_PATH"; then echo "::error::file appeared before approval"; exit 4; fi

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
APPROVAL_ID="$(jq -r '.messages[]? | .tool_results[]? | .metadata.pending_edit_approval.approval_id? // empty' "$CONVERSATION_JSON" | head -n 1)"
test -n "$APPROVAL_ID"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/approvals/$APPROVAL_ID/approve" -H 'content-type: application/json' -d '{"approved":true}' > "$APPROVAL_JSON"
for marker in '"approval_applied":true' '"applied":true' "Updated 1 file" "file_events" "event_bus_receipts" "filesystem.edited" "watcher.updated" "natural-proof-note.txt"; do grep -Fq "$marker" "$APPROVAL_JSON"; done

test -s "$NOTE_PATH"

curl -fsS "$BASE/api/events/recent" > "$EVENT_BUS_JSON"
jq -e '.count >= 2' "$EVENT_BUS_JSON" >/dev/null
for marker in '"event_bus":"change_bus"' '"event_type":"filesystem.edited"' '"event_type":"watcher.updated"' "natural-proof-note.txt" "opencode.apply_patch"; do grep -Fq "$marker" "$EVENT_BUS_JSON"; done

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/snapshot" -H 'content-type: application/json' -d '{}' >/dev/null
curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/compact" -H 'content-type: application/json' -d '{"keep_last":64,"auto":false,"overflow":false}' >/dev/null
curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
for marker in "tool_lifecycle_parts" '"status":"pending"' '"status":"running"' '"status":"completed"' "event_bus_receipts" "filesystem.edited" "watcher.updated" "file_parts" "patch_parts" "compaction_parts"; do grep -Fq "$marker" "$CONVERSATION_JSON"; done

curl -fsS -X POST "$BASE/api/browser-proof" -H 'content-type: application/json' -d "{\"url\":\"$BASE/\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" > "$BROWSER_PROOF_JSON"
jq -e '.success == true' "$BROWSER_PROOF_JSON" >/dev/null
jq -r '.screenshot_base64' "$BROWSER_PROOF_JSON" | base64 -d > "$SCREENSHOT_PNG"
test -s "$SCREENSHOT_PNG"
for marker in "natural event bus proof" "event_bus_receipts" "filesystem.edited" "watcher.updated" "OpenCode ToolPart metadata" "OpenCode PatchPart"; do grep -Fq "$marker" "$BROWSER_PROOF_JSON"; done

echo "LIVE WebUI event bus proof passed: $BASE conversation=$CONV_ID screenshot=$SCREENSHOT_PNG"
