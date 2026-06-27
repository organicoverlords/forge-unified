#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
SERVER_LOG="$PROOF_DIR/server.log"
STATUS_OUT="$PROOF_DIR/git-status.txt"
PROMPT_FILE="$PROOF_DIR/screenshot-prompt.txt"
REQUEST_JSON="$PROOF_DIR/screenshot-request.json"
STREAM_OUT="$PROOF_DIR/screenshot-stream.sse"
APPROVAL_JSON="$PROOF_DIR/approval-response.json"
SNAPSHOT_JSON="$PROOF_DIR/snapshot-response.json"
COMPACTION_JSON="$PROOF_DIR/compaction-response.json"
CONVERSATION_JSON="$PROOF_DIR/screenshot-conversation.json"
BROWSER_PROOF_JSON="$PROOF_DIR/browser-proof.json"
SCREENSHOT_PNG="$PROOF_DIR/webui.png"
NOTE_PATH="forge-proof/live-webui-feature-sprint/natural-proof-note.txt"
mkdir -p "$PROOF_DIR"
rm -f "$NOTE_PATH" "$STREAM_OUT" "$APPROVAL_JSON" "$SNAPSHOT_JSON" "$COMPACTION_JSON"

cargo build --workspace
cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; git status --short > "$STATUS_OUT" 2>/dev/null || true; }
trap cleanup EXIT

for _ in $(seq 1 60); do
  curl -fsS "$BASE/api/health" >/dev/null && break
  sleep 0.5
done

curl -fsS "$BASE/" | grep -q "Forge Unified"
curl -fsS "$BASE/api/health" | grep -q '"status":"ok"'
for marker in "OpenCode TextPart" "ReasoningPart" "SnapshotPart" "CompactionPart" "FilePart" "OpenCode ToolPart" "OpenCode PatchPart" "edit approvals"; do
  curl -fsS "$BASE/" | grep -q "$marker"
done

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" -H 'content-type: application/json' -d '{"title":"natural file creation and repo inspection proof"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
test -n "$CONV_ID"

cat > "$PROMPT_FILE" <<'PROMPT'
Please create a short proof note for this WebUI sprint.

Keep the reply brief and tell me what changed.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$REQUEST_JSON" > "$STREAM_OUT"

for marker in "event: run-start" "event: run-finish" "event: tool-result" "natural-proof-note.txt" "Edit approval required before applying patch" "pending_edit_approval" "approval_state" '"status":"pending"' "Approve edit to apply the patch" "permission_request" '"permission":"edit"' '"always":["*"]' "opencode_permission_source" "packages/opencode/src/tool/apply_patch.ts"; do
  grep -Fq "$marker" "$STREAM_OUT"
done
if test -e "$NOTE_PATH"; then echo "::error::apply_patch wrote before approval"; exit 4; fi
if grep -qi "provider-error\|missing_key\|runtime is missing" "$STREAM_OUT"; then echo "::error::provider failure in natural prompt"; exit 3; fi

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
APPROVAL_ID="$(jq -r '.messages[]? | .tool_results[]? | .metadata.pending_edit_approval.approval_id? // empty' "$CONVERSATION_JSON" | head -n 1)"
test -n "$APPROVAL_ID"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/approvals/$APPROVAL_ID/approve" -H 'content-type: application/json' -d '{"approved":true}' > "$APPROVAL_JSON"
for marker in '"approval_applied":true' '"approved_via_api":true' '"applied":true' '"status":"approved"' "Success. Updated the following files" "Updated 1 file" "file_events" "file.added" "natural-proof-note.txt"; do
  grep -Fq "$marker" "$APPROVAL_JSON"
done

test -s "$NOTE_PATH"
grep -q "Natural prompt completed" "$NOTE_PATH"

cat > "$PROMPT_FILE" <<'PROMPT'
Please inspect this repository and summarize what you find.
PROMPT
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"
curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" -H 'content-type: application/json' -H 'accept: text/event-stream' --data-binary "@$REQUEST_JSON" >> "$STREAM_OUT"
for marker in "repo_info" "file_list" "Repository status" "Top-level repository entries" "raw_output" "compact_repo_info" "compact_file_list" "Inspected the repository" "top-level files were listed"; do
  grep -Fq "$marker" "$STREAM_OUT"
done

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/snapshot" -H 'content-type: application/json' -d '{}' > "$SNAPSHOT_JSON"
grep -q '"snapshot_saved":true' "$SNAPSHOT_JSON"
grep -q "Snapshot saved at" "$SNAPSHOT_JSON"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/compact" -H 'content-type: application/json' -d '{"keep_last":64,"auto":false,"overflow":false}' > "$COMPACTION_JSON"
for marker in '"compaction_created":true' '"type":"compaction"' '"auto":false' '"overflow":false' '"identifier":"CompactionPart"' "packages/opencode/src/session/compaction.ts:create/process"; do
  grep -Fq "$marker" "$COMPACTION_JSON"
done

curl -fsS "$BASE/api/conversations/$CONV_ID" > "$CONVERSATION_JSON"
for marker in \
  "pending_edit_approval" '"status":"pending"' '"status":"approved"' '"approved_via_api":true' "Approved and applied edit" "Updated 1 file" \
  "file_events" "file.added" "natural-proof-note.txt" "permission_request" '"permission":"edit"' "opencode_permission_source" \
  "text_parts" '"type":"text"' '"identifier":"TextPart"' "opencode_text_part_source" \
  "reasoning_parts" '"type":"reasoning"' '"identifier":"ReasoningPart"' "opencode_reasoning_part_source" "public_progress_summary" '"private_chain_of_thought":false' \
  "snapshot_parts" '"type":"snapshot"' '"identifier":"SnapshotPart"' "opencode_snapshot_part_source" "Snapshot saved at" \
  "compaction_parts" '"type":"compaction"' '"identifier":"CompactionPart"' "opencode_compaction_part_source" "packages/opencode/src/session/compaction.ts:create/process" \
  "file_parts" '"type":"file"' '"identifier":"FilePart"' '"url":"workspace://forge-proof/live-webui-feature-sprint/natural-proof-note.txt"' '"mime":"text/plain"' "opencode_file_part_source" \
  "tool_parts" "tool_lifecycle_parts" '"type":"tool"' '"status":"pending"' '"status":"running"' '"status":"completed"' "ToolStatePending" "ToolStateRunning" "ToolStateCompleted" "ToolStateError" "ensureToolCall/updateToolCall/completeToolCall/failToolCall" \
  "patch_parts" '"type":"patch"' '"hash":"patch_' '"identifier":"PatchPart"' "packages/schema/src/v1/session.ts" \
  "Inspected the repository" "repo_info" "file_list" "Repository status" "Top-level repository entries" "raw_output"; do
  grep -Fq "$marker" "$CONVERSATION_JSON"
done

curl -fsS -X POST "$BASE/api/browser-proof" -H 'content-type: application/json' -d "{\"url\":\"$BASE/\",\"width\":1440,\"height\":1000,\"capture_dom\":true}" > "$BROWSER_PROOF_JSON"
jq -e '.success == true' "$BROWSER_PROOF_JSON" >/dev/null
jq -r '.screenshot_base64' "$BROWSER_PROOF_JSON" | base64 -d > "$SCREENSHOT_PNG"
test -s "$SCREENSHOT_PNG"
for marker in "natural file creation and repo inspection proof" "OpenCode edit permission request" "Approve edit" "Edit approval metadata" "Approved and applied edit" "Updated 1 file" "ADDED" "natural-proof-note.txt" "permission_request" "opencode_permission_source" "Inspected the repository" "repo_info" "file_list" "Repository status" "Top-level repository entries" "OpenCode TextPart metadata" "OpenCode ReasoningPart" "ReasoningPart metadata" "public_progress_summary" "OpenCode SnapshotPart" "SnapshotPart metadata" "Snapshot saved at" "OpenCode CompactionPart" "CompactionPart metadata" "OpenCode FilePart" "FilePart metadata" "workspace://forge-proof/live-webui-feature-sprint/natural-proof-note.txt" "OpenCode ToolPart metadata" "pending" "running" "completed" "OpenCode PatchPart" "PatchPart metadata" "patch_"; do
  grep -Fq "$marker" "$BROWSER_PROOF_JSON"
done

echo "LIVE WebUI natural edit approval + compact repo inspection + ToolPart lifecycle proof passed: $BASE conversation=$CONV_ID screenshot=$SCREENSHOT_PNG"
