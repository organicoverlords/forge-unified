#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
PROOF_DIR="${FORGE_FEATURE_PROOF_DIR:-$ROOT/forge-proof/live-webui-feature-sprint}"
mkdir -p "$PROOF_DIR"
echo "strict live model proof placeholder" > "$PROOF_DIR/live-proof-status.txt"
cargo build --workspace
