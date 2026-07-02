#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${FORGE_MODEL_SCOUT_OUT:-$ROOT/.forge-proof/model-scout}"
mkdir -p "$OUT_DIR"

: "${NIM_KEY:?NIM_KEY is required}"
BASE="${NIM_BASE_URL:-https://integrate.api.nvidia.com/v1}"
LIMIT="${FORGE_MODEL_SCOUT_LIMIT:-6}"
SLEEP_SECONDS="${FORGE_MODEL_SCOUT_SLEEP_SECONDS:-4}"
MODELS_JSON="$OUT_DIR/models.json"
RESULTS_JSONL="$OUT_DIR/results.jsonl"
SUMMARY_JSON="$OUT_DIR/summary.json"
: > "$RESULTS_JSONL"

curl -fsS "$BASE/models" \
  -H "Authorization: Bearer $NIM_KEY" \
  -H 'accept: application/json' \
  > "$MODELS_JSON"

candidates=(
  "deepseek-ai/deepseek-v4-flash"
  "mistralai/mistral-small-4-119b-2603"
  "openai/gpt-oss-120b"
  "meta/llama-3.1-405b-instruct"
  "meta/llama-3.1-70b-instruct"
  "nvidia/llama-3.3-nemotron-super-49b-v1.5"
  "qwen/qwen3-235b-a22b"
  "moonshotai/kimi-k2-instruct"
  "z-ai/glm-4.5"
  "minimax/minimax-m1-80k"
  "mistralai/mistral-large"
)

classify_error() {
  local text="${1,,}"
  if [[ "$text" == *exhausted* || "$text" == *busy* || "$text" == *capacity* || "$text" == *overloaded* || "$text" == *temporarily* ]]; then
    echo model_busy_capacity
  elif [[ "$text" == *quota* || "$text" == *billing* || "$text" == *credits* || "$text" == *hard\ limit* || "$text" == *monthly\ limit* || "$text" == *daily\ limit* ]]; then
    echo quota_capped
  elif [[ "$text" == *429* || "$text" == *rate\ limit* || "$text" == *too\ many* ]]; then
    echo rate_limited
  elif [[ "$text" == *timeout* || "$text" == *deadline* ]]; then
    echo timeout
  elif [[ "$text" == *500* || "$text" == *502* || "$text" == *503* || "$text" == *504* ]]; then
    echo server_error
  elif [[ "$text" == *unauthorized* || "$text" == *api\ key* || "$text" == *401* ]]; then
    echo missing_key
  else
    echo other
  fi
}

prompt='You are testing whether this model is suitable for a coding agent that edits a Rust web app. Reply with exactly three short lines: PASS or FAIL, main risk, and whether you can use tool calls when tools are supplied.'

tested=0
for model in "${candidates[@]}"; do
  if [ "$tested" -ge "$LIMIT" ]; then
    break
  fi
  if ! jq -e --arg model "$model" '.data[]?.id == $model' "$MODELS_JSON" >/dev/null 2>&1; then
    jq -n --arg model "$model" '{model:$model,status:"not_listed",ok:false,classification:"not_available",latency_ms:0,content:""}' >> "$RESULTS_JSONL"
    continue
  fi

  tested=$((tested + 1))
  start_ms="$(date +%s%3N)"
  body="$(jq -n --arg model "$model" --arg prompt "$prompt" '{model:$model,stream:false,max_tokens:220,temperature:0.2,messages:[{role:"user",content:$prompt}]}')"
  tmp="$OUT_DIR/response-${model//[\/:]/_}.json"
  status="$(curl -sS -o "$tmp" -w '%{http_code}' "$BASE/chat/completions" -H "Authorization: Bearer $NIM_KEY" -H 'content-type: application/json' -d "$body" || true)"
  end_ms="$(date +%s%3N)"
  latency_ms="$((end_ms-start_ms))"
  content="$(jq -r '.choices[0].message.content // empty' "$tmp" 2>/dev/null || true)"
  error_text="$(jq -r '.error.message // .message // empty' "$tmp" 2>/dev/null || true)"
  classification="ok"
  ok=false
  if [ "$status" = "200" ] && [ -n "$content" ]; then
    ok=true
  else
    classification="$(classify_error "$status $error_text")"
  fi
  jq -n --arg model "$model" --arg status "$status" --arg content "$content" --arg error_text "$error_text" --arg classification "$classification" --argjson ok "$ok" --argjson latency_ms "$latency_ms" '{model:$model,status:$status,ok:$ok,classification:$classification,cooldown_scope:"model",latency_ms:$latency_ms,content:$content,error:$error_text}' >> "$RESULTS_JSONL"
  sleep "$SLEEP_SECONDS"
done

jq -s 'sort_by((.ok|not), .latency_ms) | {generated_at: now|todate, scout_limit: env.FORGE_MODEL_SCOUT_LIMIT, sleep_seconds: env.FORGE_MODEL_SCOUT_SLEEP_SECONDS, note:"model_busy_capacity means model-level temporary busy/exhausted, not provider quota cap", recommended_order: map(select(.ok)|.model), results: .}' "$RESULTS_JSONL" > "$SUMMARY_JSON"
cat "$SUMMARY_JSON"
