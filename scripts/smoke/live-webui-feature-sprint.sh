#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

PORT="${FORGE_FEATURE_PORT:-3320}"
BASE="http://127.0.0.1:${PORT}"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
PROMPT_FILE="$PROOF_DIR/prompt.txt"
REQUEST_JSON="$PROOF_DIR/chat-request.json"
STREAM_OUT="$PROOF_DIR/chat-stream.sse"
SERVER_LOG="$PROOF_DIR/server.log"
PATCH_OUT="$PROOF_DIR/webui-generated.patch"
STATUS_OUT="$PROOF_DIR/git-status.txt"
LONG_CONTEXT_FILE="$PROOF_DIR/long-prompt-context.txt"
LONG_BYTES_OUT="$PROOF_DIR/long-prompt-bytes.txt"
mkdir -p "$PROOF_DIR"

OPENCODE_PREFIX="Copy OpenCode behavior only. Use these upstream references: anomalyco/opencode@0befd9b04939589ffe7d29c62e9260bc8ee4fff6 packages/llm/src/schema/events.ts for LLM event names; packages/core/src/session/runner/publish-llm-event.ts for tool lifecycle validation; packages/opencode/src/session/processor.ts for tool state handling; packages/opencode/src/tool/edit.ts for exact file edits; packages/opencode/src/tool/apply_patch.ts for patch semantics; packages/core/src/session/compaction.ts and packages/opencode/src/session/prompt.ts for long prompt/session handling. Do not invent a custom workflow when one of these files covers the behavior. Include OPENCODE_SOURCE with the copied file path in your final answer."

PROMPT="${FORGE_FEATURE_PROMPT:-}"
if [ -z "$PROMPT" ]; then
  PROMPT="Use OpenCode edit semantics. Call file_edit directly on PROJECT_STATE.md with old_string exactly 'Updated: 2026-06-25' and new_string exactly 'Updated: 2026-06-26 — OpenCode source proof'. Do not search for the string first. Then run repo_info, file_list with path dot, and shell_command with command cargo check --workspace --all-targets. Briefly report the changed file, whether the build passed, and OPENCODE_SOURCE packages/opencode/src/tool/edit.ts plus packages/core/src/session/compaction.ts."
fi

LONG_PROMPT_BYTES="${FORGE_FEATURE_LONG_PROMPT_BYTES:-12000}"
: > "$LONG_CONTEXT_FILE"
if [ "$LONG_PROMPT_BYTES" -gt 0 ]; then
  while [ "$(wc -c < "$LONG_CONTEXT_FILE")" -lt "$LONG_PROMPT_BYTES" ]; do
    printf 'LONG_CONTEXT_LINE copied_from=packages/core/src/session/compaction.ts copied_prompt_source=packages/opencode/src/session/prompt.ts keep_final_task_authoritative\n' >> "$LONG_CONTEXT_FILE"
  done
fi

{
  printf '%s\n\n' "$OPENCODE_PREFIX"
  printf 'LONG_PROMPT_CONTEXT_BEGIN\n'
  cat "$LONG_CONTEXT_FILE"
  printf 'LONG_PROMPT_CONTEXT_END\n\n'
  printf '%s\n' "$PROMPT"
} > "$PROMPT_FILE"

wc -c "$PROMPT_FILE" > "$LONG_BYTES_OUT"
jq -Rs '{message: ., max_rounds: 1}' "$PROMPT_FILE" > "$REQUEST_JSON"

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

CONV_ID="$(curl -fsS -X POST "$BASE/api/conversations" \
  -H 'content-type: application/json' \
  -d '{"title":"live feature sprint"}' | sed -n 's/.*"id":"\([^"]*\)".*/\1/p')"

test -n "$CONV_ID"

curl -fsS -X POST "$BASE/api/conversations/$CONV_ID/chat/stream" \
  -H 'content-type: application/json' \
  -H 'accept: text/event-stream' \
  --data-binary "@$REQUEST_JSON" \
  > "$STREAM_OUT"

grep -q "event: run-start" "$STREAM_OUT"
grep -q "event: tool-call" "$STREAM_OUT"
grep -q "event: tool-result" "$STREAM_OUT"
grep -q '"name":"file_edit"' "$STREAM_OUT"
grep -q '"name":"repo_info"' "$STREAM_OUT"
grep -q '"name":"shell_command"' "$STREAM_OUT"
grep -q 'cargo check --workspace --all-targets' "$STREAM_OUT"
grep -q 'OPENCODE_SOURCE' "$STREAM_OUT"
if grep -qi "provider-error\|missing_key\|runtime is missing" "$STREAM_OUT"; then
  echo "::error::Live feature sprint produced provider-error or missing-key output."
  exit 3
fi

git status --short > "$STATUS_OUT"
git diff > "$PATCH_OUT"
test -s "$PATCH_OUT"
grep -q "OpenCode source proof" "$PATCH_OUT"
grep -q "LONG_PROMPT_CONTEXT_BEGIN" "$PROMPT_FILE"
grep -q "packages/core/src/session/compaction.ts" "$PROMPT_FILE"

echo "LIVE WebUI feature sprint passed: $BASE conversation=$CONV_ID proof_dir=$PROOF_DIR patch=$PATCH_OUT request=$REQUEST_JSON"
