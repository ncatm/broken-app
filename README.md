# Проектная работа: broken-app + reference-app

Модуль 5 — поиск ошибок, UB, утечек и оптимизация Rust-кода.

## Структура

- `broken-app/` — исправленное и ускоренное приложение
- `reference-app/` — эталон поведения (commit `38ebe81`)

## Быстрый старт

```bash
cd broken-app
cargo test
cargo run --bin demo
cargo bench --bench baseline
cargo bench --bench criterion
```

## Исправленные дефекты

См. [broken-app/artifacts/OPTIMIZATIONS.md](broken-app/artifacts/OPTIMIZATIONS.md).

## Динамический анализ

### Тесты

```bash
cd broken-app && cargo test 2>&1 | tee artifacts/cargo_test.log
```

Лог: `broken-app/artifacts/cargo_test.log` (14 тестов).

### Miri (Linux/macOS, nightly)

```bash
rustup toolchain install nightly
rustup +nightly component add miri
cd broken-app
cargo +nightly miri test 2>&1 | tee artifacts/miri.log
```

### Valgrind (Docker, Linux)

```bash
# из корня репозитория (нужен Docker)
./scripts/valgrind-docker.sh              # broken-app
./scripts/valgrind-docker.sh reference-app
```

Лог: `broken-app/artifacts/valgrind.log`.

Скрипт собирает тесты в `rust:1-bookworm`, запускает `integration-*` и `concurrency-*` под Valgrind. Учитываются только `definite`/`indirect` утечки (шум std Rust игнорируется).

### Sanitizers (nightly, Linux)

```bash
# из корня репозитория (нужен Docker)
./scripts/sanitizers-docker.sh broken-app
```

Логи: `broken-app/artifacts/asan.log`, `broken-app/artifacts/tsan.log`.

## Профилирование

```bash
cd broken-app
cargo run --release --bin profile_flamegraph
```

Артефакты профиля:
- `broken-app/artifacts/flamegraph.svg`
- `broken-app/artifacts/profile_hotspots.txt`
- `broken-app/artifacts/profile_run.log`

## Бенчмарки до/после

```bash
cd broken-app
cargo bench --bench baseline          # после → artifacts/baseline_after.txt
bash scripts/bench_broken.sh          # до (снимок broken_src)
rustc -O artifacts/bench_before_snippet.rs -o /tmp/b && /tmp/b
cargo bench --bench criterion
```

Артефакты: `broken-app/artifacts/`.

## Reference-app

```bash
cd reference-app && cargo test
./scripts/valgrind-docker.sh reference-app   # из корня репозитория
```

Эталонный commit: `38ebe81795e90196227585a1d1890d7952ba7889`.

Артефакты эталона: [`reference-app/artifacts/`](reference-app/artifacts/) (`cargo_test.log`, `miri.log`, `valgrind.log`, `baseline.txt`, `criterion.txt`).

## Сдача

1. Публичный форк на GitHub с коммитами исправлений и `artifacts/`.
2. Ссылка на репозиторий в тренажёре.
3. `reference-app` без изменений или с указанием hash в README.
