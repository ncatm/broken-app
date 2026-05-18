#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

echo "=== До (снимок broken_src) ==="
bash scripts/bench_broken.sh | tee artifacts/baseline_before.txt

echo "=== После (текущий код) ==="
cargo bench --bench baseline 2>&1 | tee artifacts/baseline_after.txt
