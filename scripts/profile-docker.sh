#!/usr/bin/env bash
# Профилирование crate в Linux Docker + генерация flamegraph.svg.
# Использование: ./scripts/profile-docker.sh [broken-app|reference-app]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CRATE_NAME="${1:-broken-app}"
CRATE="$ROOT/$CRATE_NAME"
IMAGE="${PROFILE_IMAGE:-rust:1-bookworm}"

if [[ ! -d "$CRATE" ]]; then
  echo "Крейт не найден: $CRATE" >&2
  exit 1
fi

mkdir -p "$CRATE/artifacts"

echo "==> Профилирование в Docker (image: $IMAGE, crate: $CRATE_NAME)"

docker run --rm --privileged \
  ${PROFILE_PLATFORM:+--platform "$PROFILE_PLATFORM"} \
  -v "$ROOT:/workspace:ro" \
  -v "$CRATE/artifacts:/workspace/$CRATE_NAME/artifacts" \
  -w "/workspace/$CRATE_NAME" \
  "$IMAGE" \
  bash -euxo pipefail -c "
    export DEBIAN_FRONTEND=noninteractive
    export CARGO_TARGET_DIR=/tmp/cargo-target

    apt-get update -qq
    apt-get install -y -qq linux-perf > /dev/null

    cargo build --release --bench baseline
    BIN=\$(ls /tmp/cargo-target/release/deps/baseline-* | head -n 1)

    perf record -F 99 -g -o artifacts/perf.data -- \"\$BIN\" > /dev/null
    perf script -i artifacts/perf.data > artifacts/perf.folded.input

    cargo install inferno --locked --force > /dev/null
    inferno-collapse-perf artifacts/perf.folded.input > artifacts/perf.folded
    inferno-flamegraph artifacts/perf.folded > artifacts/flamegraph.svg

    perf report -i artifacts/perf.data --stdio --no-children --sort=dso,symbol > artifacts/perf_report.txt || true
  "

echo "==> Готово: $CRATE/artifacts/flamegraph.svg и perf_report.txt"
