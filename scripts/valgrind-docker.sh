#!/usr/bin/env bash
# Прогон integration-тестов под Valgrind в Linux (Docker).
# Использование: ./scripts/valgrind-docker.sh [broken-app|reference-app]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CRATE_NAME="${1:-broken-app}"
CRATE="$ROOT/$CRATE_NAME"
LOG="${VALGRIND_LOG:-$CRATE/artifacts/valgrind.log}"
IMAGE="${VALGRIND_IMAGE:-rust:1-bookworm}"

if [[ ! -d "$CRATE" ]]; then
  echo "Крейт не найден: $CRATE" >&2
  exit 1
fi

mkdir -p "$(dirname "$LOG")"

echo "==> Valgrind в Docker (image: $IMAGE, crate: $CRATE_NAME)"
echo "==> Лог: $LOG"

docker run --rm \
  ${VALGRIND_PLATFORM:+--platform "$VALGRIND_PLATFORM"} \
  -v "$ROOT:/workspace:ro" \
  -v "$CRATE/artifacts:/workspace/$CRATE_NAME/artifacts" \
  -w "/workspace/$CRATE_NAME" \
  "$IMAGE" \
  bash -euxo pipefail -c "
    export DEBIAN_FRONTEND=noninteractive
    export CARGO_TARGET_DIR=/tmp/cargo-target
    apt-get update -qq
    apt-get install -y -qq valgrind file > /dev/null

    cargo test --tests --no-run

    FAILED=0
    : > /workspace/$CRATE_NAME/artifacts/valgrind.log
    echo '# Valgrind (Docker rust:1-bookworm, ./scripts/valgrind-docker.sh $CRATE_NAME)' \
      >> /workspace/$CRATE_NAME/artifacts/valgrind.log

    for bin in \"\$CARGO_TARGET_DIR\"/debug/deps/integration-* \"\$CARGO_TARGET_DIR\"/debug/deps/concurrency-*; do
      [[ -f \"\$bin\" && -x \"\$bin\" ]] || continue
      [[ \"\$bin\" == *.d ]] && continue
      file \"\$bin\" | grep -q ELF || continue

      echo \"========== valgrind: \$bin ==========\" | tee -a /workspace/$CRATE_NAME/artifacts/valgrind.log
      if valgrind \
        --leak-check=full \
        --show-leak-kinds=all \
        --errors-for-leak-kinds=definite,indirect \
        --track-origins=yes \
        --error-exitcode=1 \
        \"\$bin\" \
        2>&1 | tee -a /workspace/$CRATE_NAME/artifacts/valgrind.log; then
        echo \"OK: \$bin\" | tee -a /workspace/$CRATE_NAME/artifacts/valgrind.log
      else
        echo \"FAIL: \$bin\" | tee -a /workspace/$CRATE_NAME/artifacts/valgrind.log
        FAILED=1
      fi
      echo | tee -a /workspace/$CRATE_NAME/artifacts/valgrind.log
    done

    if [[ \"\$FAILED\" -ne 0 ]]; then
      echo 'Valgrind: обнаружены ошибки' >&2
      exit 1
    fi
    echo 'Valgrind: все тестовые бинарники чисты'
  "

echo "==> Готово: $LOG"
