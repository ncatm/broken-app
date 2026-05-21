#!/usr/bin/env bash
# Запуск ASan/TSan для crate в Linux Docker.
# Использование: ./scripts/sanitizers-docker.sh [broken-app|reference-app]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CRATE_NAME="${1:-broken-app}"
CRATE="$ROOT/$CRATE_NAME"
IMAGE="${SAN_IMAGE:-rustlang/rust:nightly-bookworm}"

if [[ ! -d "$CRATE" ]]; then
  echo "Крейт не найден: $CRATE" >&2
  exit 1
fi

mkdir -p "$CRATE/artifacts"

echo "==> Sanitizers в Docker (image: $IMAGE, crate: $CRATE_NAME)"

docker run --rm \
  ${SAN_PLATFORM:+--platform "$SAN_PLATFORM"} \
  -v "$ROOT:/workspace:ro" \
  -v "$CRATE/artifacts:/workspace/$CRATE_NAME/artifacts" \
  -w "/workspace/$CRATE_NAME" \
  "$IMAGE" \
  bash -euxo pipefail -c "
    export DEBIAN_FRONTEND=noninteractive
    export CARGO_TARGET_DIR=/tmp/cargo-target
    export RUST_BACKTRACE=1

    apt-get update -qq
    apt-get install -y -qq clang lld > /dev/null

    rustup component add rust-src

    TARGET=\$(rustc -vV | awk '/host:/{print \$2}')

    echo '# ASan: cargo +nightly test --tests -Zbuild-std' > artifacts/asan.log
    echo \"# target=\$TARGET\" >> artifacts/asan.log
    RUSTFLAGS='-Zsanitizer=address -C debuginfo=1' \
      cargo +nightly test \
      --tests \
      -Zbuild-std \
      --target \"\$TARGET\" \
      2>&1 | tee -a artifacts/asan.log

    echo '# TSan: cargo +nightly test --tests -Zbuild-std' > artifacts/tsan.log
    echo \"# target=\$TARGET\" >> artifacts/tsan.log
    RUSTFLAGS='-Zsanitizer=thread -C debuginfo=1' \
      cargo +nightly test \
      --tests \
      -Zbuild-std \
      --target \"\$TARGET\" \
      2>&1 | tee -a artifacts/tsan.log
  "

echo "==> Готово: $CRATE/artifacts/asan.log и tsan.log"
