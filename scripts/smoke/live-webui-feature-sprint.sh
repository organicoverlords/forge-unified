#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
SERVER_LOG="$PROOF_DIR/server.log"
PATCH_OUT="$PROOF_DIR/webui-generated.patch"
STATUS_OUT="$PROOF_DIR/git-status.txt"
LONG_CONTEXT_FILE="$PROOF_DIR/long-prompt-context.txt"
mkdir -p "$PROOF_DIR"

OPENCODE_PREFIX="Copy OpenCode behavior only. Use these upstream references: anomalyco/opencode@0befd9b04939589ffe7d29c62e9260bc8ee4fff6 packages/llm/src/schema/events.ts for LLM event names; packages/core/src/session/runner/publish-llm-event.ts for tool lifecycle validation; packages/opencode/src/session/processor.ts for tool state handling; packages/opencode/src/session/prompt.ts and packages/core/src/session/compaction.ts for natural prompt/session handling; packages/opencode/src/tool/plan.ts for planning and next-feature suggestions; packages/opencode/src/tool/edit.ts for exact file edits; packages/opencode/src/tool/apply_patch.ts for patch semantics. Do not invent a custom workflow when one of these files covers the behavior. Include OPENCODE_SOURCE with the copied file path in your final answer."

LONG_PROMPT_BYTES="${FORGE_FEATURE_LONG_PROMPT_BYTES:-12000}"
: > "$LONG_CONTEXT_FILE"
if [ "$LONG_PROMPT_BYTES" -gt 0 ]; then
  while [ "$(wc -c < "$LONG_CONTEXT_FILE")" -lt "$LONG_PROMPT_BYTES" ]; do
    printf 'LONG_CONTEXT_LINE copied_from=packages/core/src/session/compaction.ts copied_prompt_source=packages/opencode/src/session/prompt.ts keep_final_task_authoritative\n' >> "$LONG_CONTEXT_FILE"
  done
fi

cargo build --workspace
cargo run -p forge-app -- --host 127.0.0.1 --port "$PORT" >"$SERVER_LOG" 2>&1 &
PID=$!
cleanup() {
  kill "$PID" >/dev/null 2>&1 || true
  git status --short > "$STATUS_OUT" 2>/dev/null || true
  git diff > "$PATCH_OUT" 2>/dev/null || true
}
trap cleanup EXIT

for _ in $(seq 1 60); do
  if curl -fsS "$BASE/api/health" >/dev/null; then
    break
  fi
  sleep 0.5
done

curl -fsS "$BASE/" | grep -q "Forge Unified"
curl -fsS "$BASE/api/health" | grep -q '"status":"ok"'

run_prompt() {
  local slug="$1"
  local title="$2"
  local body="$3"
  local prompt_file="$PROOF_DIR/${slug}-prompt.txt"
  local request_json="$PROOF_DIR/${slug}-request.json"
  local stream_out="$PROOF_DIR/${slug}-stream.sse"
  local bytes_out="$PROOF_DIR/${slug}-prompt-bytes.txt"

  {
    printf '%s\n\n' "$OPENCODE_PREFIX"
    printf 'LONG_PROMPT_CONTEXT_BEGIN\n'
    cat "$LONG_CONTEXT_FILE"
    printf 'LONG_PROMPT_CONTEXT_END\n\n'
    printf '%s\n' "$body"
  } > "$prompt_file"

  wc -c "$prompt_file" > "$bytes_out"
  jq -Rs '{message: ., max_rounds: 1}' "$prompt_file" > "$request_json"

  local conv_id
  conv_id="$(curl -fsS -X POST "$BASE/api/conversations" \
    -H 'content-type: application/json' \
    -d "{\"title\":\"$title\"}" | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"
  test -n "$conv_id"

  curl -fsS -X POST "$BASE/api/conversations/$conv_id/chat/stream" \
    -H 'content-type: application/json' \
    -H 'accept: text/event-stream' \
    --data-binary "@$request_json" \
    > "$stream_out"

  grep -q "event: run-start" "$stream_out"
  grep -q "event:" "$stream_out"
  grep -q 'OPENCODE_SOURCE' "$stream_out"
  if grep -qi "provider-error\|missing_key\|runtime is missing" "$stream_out"; then
    echo "::error::Live prompt $slug produced provider-error or missing-key output."
    exit 3
  fi

  echo "$stream_out"
}

SUMMARY_STREAM="$(run_prompt repo-summary "repo summary" "Analyze and summarize this repository through the normal WebUI chat interface. Call repo_info, then call file_list with path dot. Summarize the architecture, current proof status, and highest-risk gaps. Do not edit files in this prompt. End with OPENCODE_SOURCE packages/opencode/src/session/prompt.ts plus packages/core/src/session/compaction.ts.")"
grep -q '"name":"repo_info"' "$SUMMARY_STREAM"
grep -q '"name":"file_list"' "$SUMMARY_STREAM"

SUGGEST_STREAM="$(run_prompt next-feature "next feature suggestion" "Analyze this repository and suggest the next OpenCode-capsule feature to build. Call repo_info, file_list with path dot, file_read on PROJECT_STATE.md, and propose_patch with a concise code-edit suggestion. Do not apply the edit in this prompt. End with OPENCODE_SOURCE packages/opencode/src/tool/plan.ts plus packages/opencode/src/tool/apply_patch.ts.")"
grep -q '"name":"repo_info"' "$SUGGEST_STREAM"
grep -q '"name":"file_list"' "$SUGGEST_STREAM"
grep -q '"name":"file_read"' "$SUGGEST_STREAM"
grep -q '"name":"propose_patch"' "$SUGGEST_STREAM"

EDIT_STREAM="$(run_prompt edit-build "edit build proof" "Use OpenCode edit semantics. Call file_edit directly on PROJECT_STATE.md with old_string exactly 'Updated: 2026-06-25' and new_string exactly 'Updated: 2026-06-26 — OpenCode source proof'. Do not search for the string first. Then run repo_info, file_list with path dot, and shell_command with command cargo check --workspace --all-targets. Briefly report the changed file, whether the build passed, and OPENCODE_SOURCE packages/opencode/src/tool/edit.ts plus packages/core/src/session/compaction.ts.")"
grep -q '"name":"file_edit"' "$EDIT_STREAM"
grep -q '"name":"repo_info"' "$EDIT_STREAM"
grep -q '"name":"shell_command"' "$EDIT_STREAM"
grep -q 'cargo check --workspace --all-targets' "$EDIT_STREAM"

git status --short > "$STATUS_OUT"
git diff > "$PATCH_OUT"
test -s "$PATCH_OUT"
grep -q "OpenCode source proof" "$PATCH_OUT"
grep -q "LONG_PROMPT_CONTEXT_BEGIN" "$PROOF_DIR/repo-summary-prompt.txt"
grep -q "packages/core/src/session/compaction.ts" "$PROOF_DIR/repo-summary-prompt.txt"

echo "LIVE WebUI natural prompt suite passed: $BASE proof_dir=$PROOF_DIR patch=$PATCH_OUT"
