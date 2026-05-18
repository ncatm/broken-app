#!/usr/bin/env bash
# Замер «до» на исходном коде (снимок в artifacts/broken_src).
set -euo pipefail
cd "$(dirname "$0")/.."
ROOT="$(pwd)"
TMP="$(mktemp -d)"
trap 'cp -f "$TMP"/*.rs "$ROOT/src/" 2>/dev/null || cp -f "$ROOT/artifacts/broken_src/"*.rs "$ROOT/src/" && rm -rf "$TMP"' EXIT

cp -f src/*.rs "$TMP/"
cp -f artifacts/broken_src/*.rs src/

cargo build --release --quiet
BIN="$ROOT/target/release/baseline"
cargo build --release --quiet --bench baseline

echo "=== baseline (broken code, release) ==="
"$BIN" 2>&1 || cargo run --release --bench baseline 2>&1
