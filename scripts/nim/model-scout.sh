#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${FORGE_MODEL_SCOUT_OUT:-$ROOT/.forge-proof/model-scout}"
mkdir -p "$OUT_DIR"

: "${NIM_KEY:?NIM_KEY is required}"
BASE="${NIM_BASE_URL:-https://integrate.api.nvidia.com/v1}"
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

prompt='You are testing whether this model is suitable for a coding agent that edits a Rust web app. Reply with exactly three short lines: PASS or FAIL, main risk, and whether you can use tool calls when tools are supplied.'

for model in "${candidates[@]}"; do
  start_ms="$(date +%s%3N)"
  body="$(jq -n --arg model "$model" --arg prompt "$prompt" '{model:$model,stream:false,max_tokens:220,temperature:0.2,messages:[{role:"user",content:$prompt}]}')"
  tmp="$OUT_DIR/response-${model//[\/:]/_}.json"
  status="000"
  if status="$(curl -sS -o "$tmp" -w '%{http_code}' "$BASE/chat/completions" -H "Authorization: Bearer $NIM_KEY" -H 'content-type: application/json' -d "$body" || true)"; then
    :
  fi
  end_ms="$(date +%s%3N)"
  latency_ms="$((end_ms-start_ms))"
  content="$(jq -r '.choices[0].message.content // empty' "$tmp" 2>/dev/null || true)"
  ok=false
  if [ "$status" = "200" ] && [ -n "$content" ]; then ok=true; fi
  jq -n --arg model "$model" --arg status "$status" --arg content "$content" --argjson ok "$ok" --argjson latency_ms "$latency_ms" '{model:$model,status:$status,ok:$ok,latency_ms:$latency_ms,content:$content}' >> "$RESULTS_JSONL"
done

jq -s 'sort_by((.ok|not), .latency_ms) | {generated_at: now|todate, recommended_order: map(select(.ok)|.model), results: .}' "$RESULTS_JSONL" > "$SUMMARY_JSON"
cat "$SUMMARY_JSON"
