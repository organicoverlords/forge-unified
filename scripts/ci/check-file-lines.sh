#!/usr/bin/env bash
set -euo pipefail

MAX_LINES="${FORGE_MAX_FILE_LINES:-500}"
FAILED=0

should_skip_file() {
  local file="$1"
  file="${file#./}"
  case "$file" in
    scripts/smoke/capture-browser-proof.sh) return 0 ;;
    *) return 1 ;;
  esac
}

while IFS= read -r file; do
  if should_skip_file "$file"; then
    continue
  fi
  lines="$(wc -l < "$file" | tr -d ' ')"
  if [ "$lines" -gt "$MAX_LINES" ]; then
    echo "::error file=$file::File exceeds ${MAX_LINES} lines (${lines}). Split it before merging."
    FAILED=1
  fi
done < <(
  find crates scripts -type f \( -name '*.rs' -o -name '*.sh' \) \
    | grep -v '/target/' \
    | sort
)

if [ "$FAILED" -ne 0 ]; then
  echo "File line gate failed. Maximum allowed lines per checked source file: ${MAX_LINES}."
  exit 1
fi

echo "File line gate passed. All checked source files are <= ${MAX_LINES} lines."
